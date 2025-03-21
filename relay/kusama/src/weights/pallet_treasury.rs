// Copyright (C) Parity Technologies and the various Polkadot contributors, see Contributions.md
// for a list of specific contributors.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_treasury`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-06, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=./kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_treasury
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_treasury`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_treasury::WeightInfo for WeightInfo<T> {
	/// Storage: `Treasury::ProposalCount` (r:1 w:1)
	/// Proof: `Treasury::ProposalCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:0 w:1)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	fn spend_local() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1887`
		// Minimum execution time: 15_780_000 picoseconds.
		Weight::from_parts(16_140_000, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	fn remove_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `194`
		//  Estimated: `1887`
		// Minimum execution time: 8_940_000 picoseconds.
		Weight::from_parts(9_200_000, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Treasury::Deactivated` (r:1 w:1)
	/// Proof: `Treasury::Deactivated` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Approvals` (r:1 w:1)
	/// Proof: `Treasury::Approvals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Proposals` (r:99 w:99)
	/// Proof: `Treasury::Proposals` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:99 w:99)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::BountyApprovals` (r:1 w:1)
	/// Proof: `Bounties::BountyApprovals` (`max_values`: Some(1), `max_size`: Some(402), added: 897, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 99]`.
	fn on_initialize_proposals(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136 + p * (157 ±0)`
		//  Estimated: `1887 + p * (2603 ±0)`
		// Minimum execution time: 26_350_000 picoseconds.
		Weight::from_parts(34_336_772, 0)
			.saturating_add(Weight::from_parts(0, 1887))
			// Standard Error: 12_302
			.saturating_add(Weight::from_parts(26_779_000, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(p.into()))
	}
	/// Storage: `AssetRate::ConversionRateToNative` (r:1 w:0)
	/// Proof: `AssetRate::ConversionRateToNative` (`max_values`: None, `max_size`: Some(1238), added: 3713, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::SpendCount` (r:1 w:1)
	/// Proof: `Treasury::SpendCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Spends` (r:0 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(1853), added: 4328, mode: `MaxEncodedLen`)
	fn spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `182`
		//  Estimated: `4703`
		// Minimum execution time: 26_650_000 picoseconds.
		Weight::from_parts(27_160_000, 0)
			.saturating_add(Weight::from_parts(0, 4703))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(1853), added: 4328, mode: `MaxEncodedLen`)
	/// Storage: `XcmPallet::QueryCounter` (r:1 w:1)
	/// Proof: `XcmPallet::QueryCounter` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:0)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::SupportedVersion` (r:1 w:0)
	/// Proof: `XcmPallet::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueueHeads` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueueHeads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcmPallet::Queries` (r:0 w:1)
	/// Proof: `XcmPallet::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `325`
		//  Estimated: `5318`
		// Minimum execution time: 65_210_000 picoseconds.
		Weight::from_parts(66_011_000, 0)
			.saturating_add(Weight::from_parts(0, 5318))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(1853), added: 4328, mode: `MaxEncodedLen`)
	/// Storage: `XcmPallet::Queries` (r:1 w:1)
	/// Proof: `XcmPallet::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn check_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `273`
		//  Estimated: `5318`
		// Minimum execution time: 33_230_000 picoseconds.
		Weight::from_parts(33_670_000, 0)
			.saturating_add(Weight::from_parts(0, 5318))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Treasury::Spends` (r:1 w:1)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(1853), added: 4328, mode: `MaxEncodedLen`)
	fn void_spend() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `245`
		//  Estimated: `5318`
		// Minimum execution time: 18_160_000 picoseconds.
		Weight::from_parts(18_470_000, 0)
			.saturating_add(Weight::from_parts(0, 5318))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
