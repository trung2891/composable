use crate::{
	assets, auth,
	error::{ContractError, ContractResult},
	events::make_event,
	exec, msg, state,
};

use cosmwasm_std::{
	to_binary, wasm_execute, Binary, CosmosMsg, Deps, DepsMut, Env, Ibc3ChannelOpenResponse,
	IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg,
	IbcChannelOpenResponse, IbcMsg, IbcOrder, IbcPacketAckMsg, IbcPacketReceiveMsg,
	IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, IbcTimeoutBlock, MessageInfo, Reply,
	Response, SubMsg, SubMsgResult, ensure_eq,
};
use cw2::set_contract_version;
use cw20::Cw20ExecuteMsg;
use cw_utils::ensure_from_older_version;
use xc_core::{
	proto::{decode_packet, Encodable},
	shared::XcPacket,
	CallOrigin, Displayed, Funds, XCVMAck, gateway::BridgeMsg,
};

const CONTRACT_NAME: &str = "composable:xcvm-gateway";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const EXEC_PROGRAM_REPLY_ID: u64 = 0;
pub(crate) const INSTANTIATE_INTERPRETER_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_open(
	_deps: DepsMut,
	_env: Env,
	msg: IbcChannelOpenMsg,
) -> ContractResult<IbcChannelOpenResponse> {
	let (channel, version) = match msg {
		IbcChannelOpenMsg::OpenInit { channel } => (channel, None),
		IbcChannelOpenMsg::OpenTry { channel, counterparty_version } =>
			(channel, Some(counterparty_version)),
	};
	const IBC_VERSION: &str = xc_core::gateway::IBC_VERSION;
	if version.is_some() && version.as_deref() != Some(IBC_VERSION) {
		Err(ContractError::InvalidIbcVersion(version.unwrap()))
	} else if channel.order != IbcOrder::Unordered {
		Err(ContractError::InvalidIbcOrdering(channel.order))
	} else {
		let version = version.unwrap_or_else(|| String::from(IBC_VERSION));
		Ok(Some(Ibc3ChannelOpenResponse { version }))
	}
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_connect(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelConnectMsg,
) -> ContractResult<IbcBasicResponse> {
	let channel = msg.channel();
	state::IBC_CHANNEL_INFO.save(
		deps.storage,
		channel.endpoint.channel_id.clone(),
		&state::ChannelInfo {
			id: channel.endpoint.channel_id.to_string(),
			counterparty_endpoint: channel.counterparty_endpoint.clone(),
			connection_id: channel.connection_id.to_string(),
		},
	)?;
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_connect").add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_channel_close(
	deps: DepsMut,
	_env: Env,
	msg: IbcChannelCloseMsg,
) -> ContractResult<IbcBasicResponse> {
	let channel = msg.channel();
	match state::IBC_CHANNEL_NETWORK.load(deps.storage, channel.endpoint.channel_id.clone()) {
		Ok(channel_network) => {
			state::IBC_CHANNEL_NETWORK.remove(deps.storage, channel.endpoint.channel_id.clone());
			state::IBC_NETWORK_CHANNEL.remove(deps.storage, channel_network);
		},
		// Nothing to do, the channel might have never been registered to a network.
		Err(_) => {},
	}
	state::IBC_CHANNEL_INFO.remove(deps.storage, channel.endpoint.channel_id.clone());
	Ok(IbcBasicResponse::new().add_event(
		make_event("ibc_close").add_attribute("channel_id", channel.endpoint.channel_id.clone()),
	))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_receive(
	_deps: DepsMut,
	env: Env,
	msg: IbcPacketReceiveMsg,
) -> ContractResult<IbcReceiveResponse> {
	let response = IbcReceiveResponse::default().add_event(make_event("receive"));
	let msg = (|| -> ContractResult<_> {
		let packet: XcPacket = decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
		let call_origin =
			CallOrigin::Remote { relayer: msg.relayer, user_origin: packet.user_origin };
		let execute_program = msg::ExecuteProgramMsg {
			salt: packet.salt,
			program: packet.program,
			assets: packet.assets,
		};
		let msg = msg::ExecuteMsg::ExecuteProgramPrivileged { call_origin, execute_program };
		let msg = wasm_execute(env.contract.address, &msg, Default::default())?;
		Ok(SubMsg::reply_always(msg, EXEC_PROGRAM_REPLY_ID))
	})();
	Ok(match msg {
		Ok(msg) => response.set_ack(XCVMAck::OK).add_submessage(msg),
		Err(err) =>
			response.add_event(make_ibc_failure_event(err.to_string())).set_ack(XCVMAck::KO),
	})
}

pub fn make_ibc_failure_event(reason: String) -> cosmwasm_std::Event {
	make_event("receive")
		.add_attribute("result", "failure")
		.add_attribute("reason", reason.to_string())
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_ack(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketAckMsg,
) -> ContractResult<IbcBasicResponse> {
	let ack = XCVMAck::try_from(msg.acknowledgement.data.as_slice())
		.map_err(|_| ContractError::InvalidAck)?;
	let packet: XcPacket =
		decode_packet(&msg.original_packet.data).map_err(ContractError::Protobuf)?;
	let messages = match ack {
		XCVMAck::OK => {
			// https://github.com/cosmos/ibc/pull/998
			Ok(<_>::default())
		},
		XCVMAck::KO => Ok(<_>::default()),
		_ => Err(ContractError::InvalidAck),
	}?;
	Ok(IbcBasicResponse::default()
		.add_event(make_event("ack").add_attribute("ack", ack.value().to_string())))
}

#[cfg_attr(not(feature = "library"), cosmwasm_std::entry_point)]
pub fn ibc_packet_timeout(
	deps: DepsMut,
	_env: Env,
	msg: IbcPacketTimeoutMsg,
) -> ContractResult<IbcBasicResponse> {
	let packet: XcPacket = decode_packet(&msg.packet.data).map_err(ContractError::Protobuf)?;
	// https://github.com/cosmos/ibc/pull/998
	Ok(IbcBasicResponse::default())
}


/// Handle a request gateway message.
/// The call must originate from an interpreter.
fn handle_bridge_forward_no_assets(
	_: auth::Interpreter,
	deps: DepsMut,
	info: MessageInfo,
	msg: BridgeMsg,
) -> ContractResult<Response> {
	
	ensure_eq!(msg.execute_program.assets.0.len(), 0, ContractError::AssetsNonTransferrable);
	let channel_id = state::IBC_NETWORK_CHANNEL
		.load(deps.storage, msg.network_id)
		.map_err(|_| ContractError::UnknownChannel)?;
	let packet = XcPacket {
		interpreter: String::from(info.sender).into_bytes(),
		user_origin: msg.interpreter_origin.user_origin,
		salt: msg.execute_program.salt,
		program: msg.execute_program.program,
		assets: msg.execute_program.assets,
	};
	let mut event = make_event("bridge")
		.add_attribute("network_id", msg.network_id.to_string())
		.add_attribute(
			"assets",
			serde_json_wasm::to_string(&packet.assets)
				.map_err(|_| ContractError::FailedToSerialize)?,
		)
		.add_attribute(
			"program",
			serde_json_wasm::to_string(&packet.program)
				.map_err(|_| ContractError::FailedToSerialize)?,
		);
	if !packet.salt.is_empty() {
		// TODO(mina86): We're unnecessarily clone packet.salt here.  What we
		// want here is ‘to_base64(&packet.salt)’.
		let salt_attr = Binary::from(packet.salt.as_slice()).to_string();
		event = event.add_attribute("salt", salt_attr);
	}
	Ok(Response::default().add_event(event).add_message(IbcMsg::SendPacket {
		channel_id,
		data: Binary::from(packet.encode()),
		// TODO: should be a parameter or configuration
		timeout: IbcTimeout::with_block(IbcTimeoutBlock { revision: 0, height: 10000 }),
	}))
}