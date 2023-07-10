#![allow(unused_parens, unused_imports, clippy::unnecessary_cast)]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

// The weight info trait for `pallet_assets_registry`.
pub trait WeightInfo {
	fn register_asset() -> Weight;
	fn register_cosmwasm_asset() -> Weight;
	fn update_asset() -> Weight;
	fn set_creation_fee() -> Weight;
	fn update_asset_location() -> Weight;
	fn mint_cosmwasm() -> Weight;
	fn burn_cosmwasm() -> Weight;
}

impl WeightInfo for () {
	fn register_asset() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn register_cosmwasm_asset() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn update_asset() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn set_creation_fee() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn update_asset_location() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn mint_cosmwasm() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn burn_cosmwasm() -> Weight {
		Weight::from_ref_time(100_000)
	}
}

/// Weights for pallet_assets_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn register_asset() -> Weight {
		Weight::from_ref_time(9_958_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn register_cosmwasm_asset() -> Weight {
		Weight::from_ref_time(100_000)
	}

	fn update_asset() -> Weight {
		Weight::from_ref_time(9_958_000_u64)
	}

	fn set_creation_fee() -> Weight {
		Weight::from_ref_time(9_958_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn update_asset_location() -> Weight {
		Weight::from_ref_time(9_958_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn mint_cosmwasm() -> Weight {
		Weight::from_ref_time(9_958_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
	}

	fn burn_cosmwasm() -> Weight {
		Weight::from_ref_time(9_958_000_u64).saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
