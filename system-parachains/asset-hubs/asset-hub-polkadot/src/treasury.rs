// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::*;
use frame_support::traits::{tokens::UnityOrOuterConversion, FromContains};
use polkadot_runtime_common::impls::{ContainsParts, VersionedLocatableAsset};
use system_parachains_common::pay::{LocalPay, VersionedLocatableAccount};

parameter_types! {
	pub const SpendPeriod: BlockNumber = 24 * RC_DAYS;
	pub const Burn: Permill = Permill::from_percent(1);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const PayoutSpendPeriod: BlockNumber = 30 * RC_DAYS;
	pub const MaxApprovals: u32 = 100;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type RejectOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
	type RuntimeEvent = RuntimeEvent;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type MaxApprovals = MaxApprovals;
	type WeightInfo = (); // TODO: weights::pallet_treasury::WeightInfo<Runtime>;
	type SpendOrigin = TreasurySpender;
	type AssetKind = VersionedLocatableAsset;
	type Beneficiary = VersionedLocatableAccount;
	type BeneficiaryLookup = IdentityLookup<Self::Beneficiary>;
	// TODO: should we continue using `RelayTreasuryPalletAccount` or we move all assets to
	// `Treasury::account_id()``
	type Paymaster = LocalPay<
		NativeAndAssets,
		xcm_config::RelayTreasuryPalletAccount,
		xcm_config::LocationToAccountId,
	>;
	type BalanceConverter = AssetRateWithNative;
	type PayoutPeriod = PayoutSpendPeriod;
	// TODO polkadot_runtime_common::impls::benchmarks::TreasuryArguments;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = benchmarking::MockedTreasuryArguments;
}

parameter_types! {
	pub const BountyDepositBase: Balance = DOLLARS;
	pub const BountyDepositPayoutDelay: BlockNumber = 0;
	pub const BountyUpdatePeriod: BlockNumber = 90 * RC_DAYS;
	pub const MaximumReasonLength: u32 = 16384;
	// TODO: update deposits
	pub const DataDepositPerByte: Balance = system_para_deposit(0, 1);
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = 10 * DOLLARS;
	pub const CuratorDepositMax: Balance = 200 * DOLLARS;
	pub const BountyValueMinimum: Balance = 10 * DOLLARS;
}

impl pallet_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type ChildBountyManager = ChildBounties;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type OnSlash = Treasury;
	type WeightInfo = (); // TODO: weights::pallet_bounties::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MaxActiveChildBountyCount: u32 = 100;
	pub const ChildBountyValueMinimum: Balance = BountyValueMinimum::get() / 10;
}

impl pallet_child_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type MaxActiveChildBountyCount = MaxActiveChildBountyCount;
	type ChildBountyValueMinimum = ChildBountyValueMinimum;
	type WeightInfo = (); // TODO: weights::pallet_child_bounties::WeightInfo<Runtime>;
}

/// The [frame_support::traits::tokens::ConversionFromAssetBalance] implementation provided by the
/// `AssetRate` pallet instance.
///
/// With additional decoration to identify different IDs/locations of native asset and provide a
/// one-to-one balance conversion for them.
pub type AssetRateWithNative = UnityOrOuterConversion<
	ContainsParts<
		FromContains<
			xcm_builder::IsChildSystemParachain<ParaId>,
			xcm_builder::IsParentsOnly<ConstU8<1>>,
		>,
	>,
	AssetRate,
>;

impl pallet_asset_rate::Config for Runtime {
	type WeightInfo = (); // TODO weights::pallet_asset_rate::WeightInfo<Runtime>;
	type RuntimeEvent = RuntimeEvent;
	type CreateOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
	type RemoveOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
	type UpdateOrigin = EitherOfDiverse<EnsureRoot<AccountId>, Treasurer>;
	type Currency = Balances;
	type AssetKind = <Runtime as pallet_treasury::Config>::AssetKind;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = polkadot_runtime_common::impls::benchmarks::AssetRateArguments;
}

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking {
	use super::*;

	pub struct MockedTreasuryArguments;
	impl pallet_treasury::ArgumentsFactory<VersionedLocatableAsset, VersionedLocatableAccount>
		for MockedTreasuryArguments
	{
		fn create_asset_kind(_seed: u32) -> VersionedLocatableAsset {
			todo!()
		}
		fn create_beneficiary(_seed: [u8; 32]) -> VersionedLocatableAccount {
			todo!()
		}
	}
}
