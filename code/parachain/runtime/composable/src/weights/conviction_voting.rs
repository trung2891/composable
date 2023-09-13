#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_conviction_voting`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_conviction_voting::WeightInfo for WeightInfo<T> {
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:1 w:1)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13551`
		//  Estimated: `42428`
		// Minimum execution time: 154_104_000 picoseconds.
		Weight::from_parts(162_701_000, 0)
			.saturating_add(Weight::from_parts(0, 42428))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14272`
		//  Estimated: `83866`
		// Minimum execution time: 241_839_000 picoseconds.
		Weight::from_parts(251_787_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `14024`
		//  Estimated: `83866`
		// Minimum execution time: 198_871_000 picoseconds.
		Weight::from_parts(208_410_000, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13110`
		//  Estimated: `30706`
		// Minimum execution time: 86_480_000 picoseconds.
		Weight::from_parts(90_343_000, 0)
			.saturating_add(Weight::from_parts(0, 30706))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:512 w:512)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 512]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29746 + r * (365 ±0)`
		//  Estimated: `83866 + r * (3411 ±0)`
		// Minimum execution time: 82_384_000 picoseconds.
		Weight::from_parts(1_967_705_239, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 169_648
			.saturating_add(Weight::from_parts(46_550_419, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(6))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:512 w:512)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(936), added: 3411, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(38963), added: 41438, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 512]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `29661 + r * (365 ±0)`
		//  Estimated: `83866 + r * (3411 ±0)`
		// Minimum execution time: 50_266_000 picoseconds.
		Weight::from_parts(1_956_854_151, 0)
			.saturating_add(Weight::from_parts(0, 83866))
			// Standard Error: 172_335
			.saturating_add(Weight::from_parts(46_688_704, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 3411).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(311), added: 2786, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12323`
		//  Estimated: `30706`
		// Minimum execution time: 114_930_000 picoseconds.
		Weight::from_parts(122_209_000, 0)
			.saturating_add(Weight::from_parts(0, 30706))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}