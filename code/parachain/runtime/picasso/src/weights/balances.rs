
//! Autogenerated weights for `balances`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-01, STEPS: `50`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `3a6013dfb40d`, CPU: `Intel(R) Xeon(R) CPU @ 3.10GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/jif3kmz9kgiwz8hg8nzb9d2kiga1rnga-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `balances`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> balances::WeightInfo for WeightInfo<T> {
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1592`
		//  Estimated: `5206`
		// Minimum execution time: 107_160 nanoseconds.
		Weight::from_ref_time(109_240_000)
			.saturating_add(Weight::from_proof_size(5206))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `2603`
		// Minimum execution time: 67_842 nanoseconds.
		Weight::from_ref_time(68_888_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_balance_creating() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1673`
		//  Estimated: `2603`
		// Minimum execution time: 46_363 nanoseconds.
		Weight::from_ref_time(47_797_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_balance_killing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1673`
		//  Estimated: `2603`
		// Minimum execution time: 52_413 nanoseconds.
		Weight::from_ref_time(53_785_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1591`
		//  Estimated: `7809`
		// Minimum execution time: 107_086 nanoseconds.
		Weight::from_ref_time(112_225_000)
			.saturating_add(Weight::from_proof_size(7809))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1457`
		//  Estimated: `2603`
		// Minimum execution time: 81_083 nanoseconds.
		Weight::from_ref_time(83_584_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_unreserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1491`
		//  Estimated: `2603`
		// Minimum execution time: 41_772 nanoseconds.
		Weight::from_ref_time(43_222_000)
			.saturating_add(Weight::from_proof_size(2603))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
