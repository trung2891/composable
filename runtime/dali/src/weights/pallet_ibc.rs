
//! Autogenerated weights for `pallet_ibc`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=20
// --output=runtime/dali/src/weights
// --log
// error

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_ibc`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ibc::WeightInfo for WeightInfo<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:0 w:1)
	// Storage: Ibc ClientUpdateHeight (r:0 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:2 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:3 w:1)
	fn update_tendermint_client() -> Weight {
		(671_465_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Ibc ConnectionCounter (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ConnectionClient (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:0 w:1)
	fn connection_open_init() -> Weight {
		(124_823_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System BlockHash (r:2 w:0)
	// Storage: Ibc HostConsensusStates (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_try_open_tendermint() -> Weight {
		(569_090_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: System BlockHash (r:2 w:0)
	// Storage: Ibc HostConsensusStates (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_open_ack_tendermint() -> Weight {
		(559_747_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn conn_open_confirm_tendermint() -> Weight {
		(264_591_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Ibc ChannelCounter (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ChannelsConnection (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e636541636b2f706f7274732f70696e672f63] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e6365526563762f706f7274732f70696e672f] (r:0 w:1)
	// Storage: unknown [0x6962632f6e65787453657175656e636553656e642f706f7274732f70696e672f] (r:0 w:1)
	fn channel_open_init() -> Weight {
		(132_427_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_try_tendermint() -> Weight {
		(287_295_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_ack_tendermint() -> Weight {
		(286_629_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_open_confirm_tendermint() -> Weight {
		(282_122_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	fn channel_close_init() -> Weight {
		(122_356_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn channel_close_confirm_tendermint() -> Weight {
		(282_985_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketReceiptCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	// Storage: unknown [0x6962632f72656365697074732f706f7274732f70696e672f6368616e6e656c73] (r:1 w:1)
	fn recv_packet_tendermint(i: u32, ) -> Weight {
		(347_919_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((81_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6d6d69746d656e74732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	fn ack_packet_tendermint(i: u32, j: u32, ) -> Weight {
		(349_849_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((87_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 1_000
			.saturating_add((73_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:1 w:0)
	// Storage: Ibc ClientUpdateHeight (r:1 w:0)
	// Storage: Ibc PacketCounter (r:1 w:1)
	// Storage: unknown [0x6962632f6368616e6e656c456e64732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:1 w:0)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6d6d69746d656e74732f706f7274732f70696e672f6368616e6e] (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	fn timeout_packet_tendermint(i: u32, ) -> Weight {
		(358_124_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((93_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x3a6368696c645f73746f726167653a64656661756c743a6962632f] (r:1 w:0)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Ibc HostConsensusStates (r:1 w:1)
	// Storage: System Digest (r:1 w:1)
	fn on_finalize(a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 917_000
			.saturating_add((52_566_000 as Weight).saturating_mul(a as Weight))
			// Standard Error: 917_000
			.saturating_add((19_085_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 917_000
			.saturating_add((47_927_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 917_000
			.saturating_add((17_322_000 as Weight).saturating_mul(d as Weight))
			// Standard Error: 917_000
			.saturating_add((11_314_000 as Weight).saturating_mul(e as Weight))
			// Standard Error: 917_000
			.saturating_add((17_311_000 as Weight).saturating_mul(f as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(a as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(c as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(d as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(e as Weight)))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(f as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Ibc ConnectionCounter (r:1 w:1)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ConnectionClient (r:1 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:1 w:0)
	// Storage: unknown [0x6962632f636f6e6e656374696f6e732f636f6e6e656374696f6e2d30] (r:0 w:1)
	fn initiate_connection() -> Weight {
		(132_839_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Ibc ClientCounter (r:1 w:1)
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: ParachainInfo ParachainId (r:1 w:0)
	// Storage: Ibc ClientUpdateTime (r:0 w:1)
	// Storage: Ibc ClientUpdateHeight (r:0 w:1)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636c6965] (r:0 w:2)
	// Storage: unknown [0x6962632f636c69656e74732f30372d74656e6465726d696e742d302f636f6e73] (r:0 w:1)
	fn create_client() -> Weight {
		(128_127_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
}
