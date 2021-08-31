//!

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(
	bad_style,
	bare_trait_objects,
	const_err,
	improper_ctypes,
	non_shorthand_field_patterns,
	no_mangle_generic_items,
	overflowing_literals,
	path_statements,
	patterns_in_fns_without_body,
	private_in_public,
	unconditional_recursion,
	unused_allocation,
	unused_comparisons,
	unused_parens,
	while_true,
	trivial_casts,
	trivial_numeric_casts,
	unused_extern_crates
)]

mod rate_model;

#[frame_support::pallet]
pub mod pallet {

	use codec::{Codec, EncodeLike, FullCodec};
	use composable_traits::{
		lending::{Lending, LendingConfigInput},
		oracle::Oracle,
		vault::{Deposit, Vault, VaultConfig},
	};
	use frame_support::{
		pallet_prelude::*,
		traits::{
			fungibles::{Inspect, Mutate, Transfer},
			tokens::{fungibles::MutateHold, DepositConsequence},
		},
		PalletId,
	};
	use frame_system::{ensure_signed, pallet_prelude::OriginFor, Config as SystemConfig};
	use num_traits::{Bounded, SaturatingSub};
	use sp_runtime::{
		helpers_128bit::multiply_by_rational,
		traits::{
			AccountIdConversion, AtLeast32BitUnsigned, CheckedAdd, CheckedMul, CheckedSub, Convert,
			Hash, Zero,
		},
		Permill, Perquintill,
	};
	use sp_std::fmt::Debug;

	pub const PALLET_ID: PalletId = PalletId(*b"Lending!");

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Oracle: Oracle;
		type Vault: Vault;
		type PairId: EncodeLike
			+ Clone
			+ Codec
			+ Debug
			+ PartialEq
			+ From<(<Self::Vault as Vault>::VaultId, <Self::Vault as Vault>::VaultId)>;
		type Balance;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		Overflow,
		/// vault provided does not exist
		VaultNotFound,
	}

	#[derive(Encode, Decode, Default)]
	pub struct LendingConfig<VaultId: Default + Codec, AccountId: Codec + Default> {
		/// asset users want to borrow
		pub deposit: VaultId,
		/// asset users will put as collateral
		pub collateral: VaultId,
		/// can pause borrow & deposits of assets
		pub manager: AccountId,
		pub reserve_factor: Permill,
		pub collateral_factor: Permill,
	}

	/// stores all market pairs of assets to be assets/collateral
	// only assets supported by `Oracle` are possible
	#[pallet::storage]
	#[pallet::getter(fn allocations)]
	pub type Pairs<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::PairId,
		LendingConfig<<T::Vault as Vault>::VaultId, T::AccountId>,
		ValueQuery,
	>;

	impl<T: Config> Lending for Pallet<T> {
		type AssetId = <T::Vault as Vault>::AssetId;

		type VaultId = <T::Vault as Vault>::VaultId;

		type AccountId = T::AccountId;

		type PairId = T::PairId;

		type Error = Error<T>;

		type Balance = T::Balance;

		type BlockNumber = T::BlockNumber;

		fn create(
			rent: Deposit<Self::Balance, Self::BlockNumber>,
			deposit: <T::Vault as Vault>::VaultId,
			collateral: <T::Vault as Vault>::VaultId,
			config: LendingConfigInput<Self::AccountId>,
		) -> Result<Self::PairId, DispatchError> {
			let collateral_asset = T::Vault::asset_id(&collateral)?;
			let deposit_asset = T::Vault::asset_id(&deposit)?;
			let pair_id = T::PairId::from((collateral.clone(), deposit.clone()));
			let config = LendingConfig::<<T::Vault as Vault>::VaultId, T::AccountId> {
				manager: config.manager,
				reserve_factor: config.reserve_factor,
				collateral_factor: config.collateral_factor,
				collateral,
				deposit,
			};
			// upsetting is reconfiguration
			// PALL-18 Integrate Oracle Pallet
			// use `.ok_or(...)?` to provide an error compatible with `Result<<T as pallet::Config>::PairId, sp_runtime::DispatchError>`
			// expected composable_traits::oracle::Oracle::AssetId, found composable_traits::vault::Vault::AssetId
			//<T::Oracle as Oracle>::get_price(collateral_asset)?;
			//<T::Oracle as Oracle>::get_price(deposit_asset)?;
			Pairs::<T>::insert(config.manager.clone(), pair_id.clone(), config);
			Ok(pair_id)
		}

		fn get_pair_in_vault(vault: Self::VaultId) -> Result<Vec<Self::PairId>, Self::Error> {
			todo!()
		}

		fn get_pairs_all() -> Result<Vec<Self::PairId>, Self::Error> {
			todo!()
		}

		fn borrow(
			pair: Self::PairId,
			debt_owner: &Self::AccountId,
			amount_to_borrow: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn repay_borrow(
			pair: Self::PairId,
			from: &Self::AccountId,
			beneficiary: &Self::AccountId,
			repay_amount: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn redeem(
			pair: Self::PairId,
			to: &Self::AccountId,
			redeem_amount: Self::Balance,
		) -> Result<(), Self::Error> {
			todo!()
		}

		fn calculate_liquidation_fee(amount: Self::Balance) -> Self::Balance {
			todo!()
		}

		fn total_borrows(pair: Self::PairId) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn accrue_interest(pair: Self::PairId) -> Result<(), Self::Error> {
			todo!()
		}

		fn borrow_balance_current(
			pair: Self::PairId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn withdraw_fees(to_withdraw: Self::Balance) -> Result<(), Self::Error> {
			todo!()
		}

		fn collateral_of_account(
			pair: Self::PairId,
			account: &Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn collateral_required(
			pair: Self::PairId,
			borrow_amount: Self::Balance,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn get_borrow_limit(
			pair: Self::PairId,
			account: Self::AccountId,
		) -> Result<Self::Balance, Self::Error> {
			todo!()
		}

		fn account_id() -> Self::AccountId {
			todo!()
		}
	}
}
