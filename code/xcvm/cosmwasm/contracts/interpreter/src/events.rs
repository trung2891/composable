use cosmwasm_std::{Addr, Event};
use serde::{Deserialize, Serialize};
use xc_core::{service::dex::ExchangeId, shared, InterpreterOrigin, NetworkId, UserId};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.exchange.succeeded")]
pub struct CvmInterpreterExchangeSucceeded {
	pub exchange_id: ExchangeId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.exchange.started")]
pub struct CvmInterpreterExchangeStarted {
	pub exchange_id: ExchangeId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.execution.started")]
pub struct CvmInterpreterExecutionStarted {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instantiated")]
pub struct CvmInterpreterInstantiated {
	pub interpreter_origin: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.transferred")]
pub struct CvmInterpreterTransferred {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.owner.added")]
pub struct CvmInterpreterOwnerAdded {
	pub owner: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.owner.removed")]
pub struct CvmInterpreterOwnerRemoved {
	pub owner: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.execution.failed")]
pub struct CvmInterpreterExchangeFailed {
	pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instruction.spawned")]
pub struct CvmInterpreterInstructionSpawned {
	pub origin_network_id: NetworkId,
	pub origin_user_id: UserId,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.self.failed")]
pub struct CvmInterpreterSelfFailed {
	pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.crosschain.failed")]
pub struct CvmInterpreterCrosschainFailed {
	pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.instruction.call.initiated")]
pub struct CvmInterpreterInstructionCallInitiated {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
#[serde(rename = "cvm.interpreter.step.executed")]
pub struct CvmInterpreterStepExecuted {
	#[serde(serialize_with = "hex::serialize", deserialize_with = "hex::deserialize")]
	#[cfg_attr(feature = "std", schemars(schema_with = "String::json_schema"))]
	pub tag: Vec<u8>,
}

/// used to generate schema, so that each events schema is available in one place
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "std", derive(schemars::JsonSchema))]
pub enum CvmInterpreter {
	StepExecuted(CvmInterpreterStepExecuted),
	SelfFailed(CvmInterpreterSelfFailed),
	ExchangeStarted(CvmInterpreterExchangeStarted),
	InstructionCallInitiated(CvmInterpreterInstructionCallInitiated),
	InstructionSpawned(CvmInterpreterInstructionSpawned),
	ExchangeFailed(CvmInterpreterExchangeFailed),
	OwnerRemoved(CvmInterpreterOwnerRemoved),
	OwnerAdded(CvmInterpreterOwnerAdded),
	ExecutionStarted(CvmInterpreterExecutionStarted),
	Transferred(CvmInterpreterTransferred),
	Instantiated(CvmInterpreterInstantiated),
	Exchanged(CvmInterpreterExchangeSucceeded),
	CrosschainFailed(CvmInterpreterCrosschainFailed),
}

// beneath is something to be generate by macro
// https://github.com/CosmWasm/cosmwasm/discussions/1871

impl CvmInterpreterCrosschainFailed {
	pub fn new(reason: String) -> Event {
		Event::new("cvm.interpreter.crosschain.failed").add_attribute("reason", reason)
	}
}

impl CvmInterpreterStepExecuted {
	pub fn new(tag: &[u8]) -> Event {
		Event::new("cvm.interpreter.step.executed").add_attribute("tag", hex::encode(tag))
	}
}

impl CvmInterpreterSelfFailed {
	pub fn new(reason: String) -> Event {
		Event::new("cvm.interpreter.self.failed").add_attribute("reason", reason)
	}
}

impl CvmInterpreterExchangeStarted {
	pub fn new(exchange_id: ExchangeId) -> Event {
		Event::new("cvm.interpreter.exchange.started")
			.add_attribute("exchange_id", exchange_id.to_string())
	}
}

impl CvmInterpreterInstructionCallInitiated {
	pub fn new() -> Event {
		Event::new("cvm.interpreter.instruction.call.initiated")
	}
}

impl CvmInterpreterInstructionSpawned {
	pub fn new(
		origin_network_id: NetworkId,
		origin_user_id: UserId,
		network_id: NetworkId,
	) -> Event {
		Event::new("cvm.interpreter.instruction.spawned")
			.add_attribute(
				"origin_network_id",
				serde_json_wasm::to_string(&origin_network_id)
					.expect("network id is controlled by us and it is always serde"),
			)
			.add_attribute(
				"origin_user_id",
				serde_json_wasm::to_string(&origin_user_id)
					.expect("user id is controlled by us and it is always serde"),
			)
			.add_attribute(
				"network_id",
				serde_json_wasm::to_string(&network_id)
					.expect("network id is controlled by us and it is always serde"),
			)
	}
}

impl CvmInterpreterExchangeFailed {
	pub fn new(reason: String) -> Event {
		Event::new("cvm.interpreter.exchange.failed").add_attribute("reason", reason)
	}
}

impl CvmInterpreterOwnerRemoved {
	pub fn new(owners: Vec<Addr>) -> Event {
		let mut e = Event::new("cvm.interpreter.owner.removed");
		for owner in owners {
			e = e.add_attribute("owner", owner.to_string())
		}
		e
	}
}

impl CvmInterpreterOwnerAdded {
	pub fn new(owners: Vec<Addr>) -> Event {
		let mut e = Event::new("cvm.interpreter.owner.added");
		for owner in owners {
			e = e.add_attribute("owner", owner.to_string())
		}
		e
	}
}

impl CvmInterpreterExecutionStarted {
	pub fn new() -> Event {
		Event::new("cvm.interpreter.execution.started")
	}
}

impl CvmInterpreterTransferred {
	pub fn new() -> Event {
		Event::new("cvm.interpreter.transferred")
	}
}

impl CvmInterpreterInstantiated {
	pub const NAME: &str = "cvm.interpreter.instantiated";
	pub const INTERPRETER_ORIGIN: &str = "interpreter_origin";
	pub fn new(interpreter_origin: &InterpreterOrigin) -> Event {
		Event::new(Self::NAME).add_attribute(
			Self::INTERPRETER_ORIGIN,
			shared::encode_base64(interpreter_origin).expect("origin is managed by"),
		)
	}
}

impl CvmInterpreterExchangeSucceeded {
	pub fn new(exchange_id: ExchangeId) -> Event {
		Event::new("cvm.interpreter.exchanged")
			.add_attribute("exchange_id", exchange_id.to_string())
	}
}
