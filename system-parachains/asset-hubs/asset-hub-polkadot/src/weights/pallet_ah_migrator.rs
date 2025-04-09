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

//! Autogenerated weights for `pallet_ah_migrator`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 46.0.0
//! DATE: 2025-03-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `59a913dd07aa`, CPU: `QEMU Virtual CPU version 2.5+`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: 1024

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --extrinsic=*
// --runtime=target/production/wbuild/asset-hub-polkadot-runtime/asset_hub_polkadot_runtime.wasm
// --pallet=pallet_ah_migrator
// --header=/_work/fellowship-001/runtimes/runtimes/.github/scripts/cmd/file_header.txt
// --output=./system-parachains/asset-hubs/asset-hub-polkadot/src/weights
// --wasm-execution=compiled
// --steps=50
// --repeat=20
// --heap-pages=4096

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_ah_migrator`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_ah_migrator::WeightInfo for WeightInfo<T> {
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:0)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `217`
		//  Estimated: `3682`
		// Minimum execution time: 22_000_000 picoseconds.
		Weight::from_parts(22_000_000, 3682)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `System::Account` (r:255 w:255)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_multisigs(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `52 + n * (135 ±0)`
		//  Estimated: `1493 + n * (2603 ±0)`
		// Minimum execution time: 33_000_000 picoseconds.
		Weight::from_parts(10_616_351, 1493)
			// Standard Error: 76_270
			.saturating_add(Weight::from_parts(12_166_224, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `System::Account` (r:255 w:255)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Holds` (r:255 w:255)
	/// Proof: `Balances::Holds` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:255 w:255)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(67), added: 2542, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:255 w:255)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_accounts(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `52 + n * (135 ±0)`
		//  Estimated: `1493 + n * (3774 ±0)`
		// Minimum execution time: 100_000_000 picoseconds.
		Weight::from_parts(100_000_000, 1493)
			// Standard Error: 134_082
			.saturating_add(Weight::from_parts(78_888_511, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(n.into()))
	}
	/// Storage: `System::Account` (r:255 w:255)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_liquid_accounts(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `52 + n * (135 ±0)`
		//  Estimated: `1493 + n * (2603 ±0)`
		// Minimum execution time: 50_000_000 picoseconds.
		Weight::from_parts(21_139_169, 1493)
			// Standard Error: 102_841
			.saturating_add(Weight::from_parts(20_038_640, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `Claims::Vesting` (r:255 w:255)
	/// Proof: `Claims::Vesting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_claims(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `152`
		//  Estimated: `1493 + n * (2475 ±0)`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(22_401_795, 1493)
			// Standard Error: 23_344
			.saturating_add(Weight::from_parts(1_955_496, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2475).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Proxy::Proxies` (r:0 w:255)
	/// Proof: `Proxy::Proxies` (`max_values`: None, `max_size`: Some(1241), added: 3716, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_proxy_proxies(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(20_375_957, 1493)
			// Standard Error: 30_997
			.saturating_add(Weight::from_parts(2_435_518, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `System::Account` (r:255 w:255)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_proxy_announcements(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `52 + n * (135 ±0)`
		//  Estimated: `1493 + n * (2603 ±0)`
		// Minimum execution time: 36_000_000 picoseconds.
		Weight::from_parts(38_032_894, 1493)
			// Standard Error: 74_055
			.saturating_add(Weight::from_parts(12_958_023, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `Vesting::Vesting` (r:255 w:255)
	/// Proof: `Vesting::Vesting` (`max_values`: None, `max_size`: Some(1057), added: 3532, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Vesting::StorageVersion` (r:0 w:1)
	/// Proof: `Vesting::StorageVersion` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_vesting_schedules(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `242`
		//  Estimated: `1493 + n * (3532 ±0)`
		// Minimum execution time: 32_000_000 picoseconds.
		Weight::from_parts(32_000_000, 1493)
			// Standard Error: 389_200
			.saturating_add(Weight::from_parts(12_350_053, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 3532).saturating_mul(n.into()))
	}
	/// Storage: `NominationPools::SubPoolsStorage` (r:255 w:255)
	/// Proof: `NominationPools::SubPoolsStorage` (`max_values`: None, `max_size`: Some(1197), added: 3672, mode: `MaxEncodedLen`)
	/// Storage: `NominationPools::CounterForSubPoolsStorage` (r:1 w:1)
	/// Proof: `NominationPools::CounterForSubPoolsStorage` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_nom_pools_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `340`
		//  Estimated: `1493 + n * (3672 ±0)`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(28_991_175, 1493)
			// Standard Error: 43_359
			.saturating_add(Weight::from_parts(4_113_463, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 3672).saturating_mul(n.into()))
	}
	/// Storage: `FastUnstake::Queue` (r:255 w:255)
	/// Proof: `FastUnstake::Queue` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `FastUnstake::CounterForQueue` (r:1 w:1)
	/// Proof: `FastUnstake::CounterForQueue` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_fast_unstake_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `151`
		//  Estimated: `1493 + n * (2531 ±0)`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(19_973_739, 1493)
			// Standard Error: 30_644
			.saturating_add(Weight::from_parts(2_938_986, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::DecidingCount` (r:0 w:16)
	/// Proof: `Referenda::DecidingCount` (`max_values`: None, `max_size`: Some(14), added: 2489, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumCount` (r:0 w:1)
	/// Proof: `Referenda::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::TrackQueue` (r:0 w:16)
	/// Proof: `Referenda::TrackQueue` (`max_values`: None, `max_size`: Some(2012), added: 4487, mode: `MaxEncodedLen`)
	fn receive_referenda_values() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 57_000_000 picoseconds.
		Weight::from_parts(61_000_000, 1493)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(34_u64))
	}
	/// Storage: `Preimage::PreimageFor` (r:255 w:255)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:255 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:255 w:255)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:0 w:255)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_active_referendums(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199 + n * (2146 ±0)`
		//  Estimated: `1493 + n * (4196819 ±0)`
		// Minimum execution time: 50_000_000 picoseconds.
		Weight::from_parts(50_000_000, 1493)
			// Standard Error: 858_339
			.saturating_add(Weight::from_parts(33_986_192, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 4196819).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::ReferendumInfoFor` (r:0 w:255)
	/// Proof: `Referenda::ReferendumInfoFor` (`max_values`: None, `max_size`: Some(936), added: 3411, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_complete_referendums(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(32_795_164, 1493)
			// Standard Error: 187_466
			.saturating_add(Weight::from_parts(1_409_176, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `Preimage::PreimageFor` (r:255 w:255)
	/// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:255 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:255 w:255)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Agenda` (r:0 w:255)
	/// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_scheduler_agenda(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `199 + n * (2146 ±0)`
		//  Estimated: `1493 + n * (4196819 ±0)`
		// Minimum execution time: 48_000_000 picoseconds.
		Weight::from_parts(48_000_000, 1493)
			// Standard Error: 199_151
			.saturating_add(Weight::from_parts(32_374_537, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 4196819).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Scheduler::Lookup` (r:0 w:255)
	/// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_scheduler_lookup(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 23_000_000 picoseconds.
		Weight::from_parts(25_547_900, 1493)
			// Standard Error: 22_694
			.saturating_add(Weight::from_parts(1_154_528, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `VoterList::ListNodes` (r:255 w:255)
	/// Proof: `VoterList::ListNodes` (`max_values`: None, `max_size`: Some(154), added: 2629, mode: `MaxEncodedLen`)
	/// Storage: `VoterList::CounterForListNodes` (r:1 w:1)
	/// Proof: `VoterList::CounterForListNodes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_bags_list_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `1493 + n * (2629 ±0)`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(28_482_706, 1493)
			// Standard Error: 31_807
			.saturating_add(Weight::from_parts(3_269_583, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2629).saturating_mul(n.into()))
	}
	/// Storage: `Indices::Accounts` (r:255 w:255)
	/// Proof: `Indices::Accounts` (`max_values`: None, `max_size`: Some(69), added: 2544, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_indices(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `1493 + n * (2544 ±0)`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(21_036_100, 1493)
			// Standard Error: 29_250
			.saturating_add(Weight::from_parts(2_362_692, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2544).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `ConvictionVoting::VotingFor` (r:0 w:255)
	/// Proof: `ConvictionVoting::VotingFor` (`max_values`: None, `max_size`: Some(27241), added: 29716, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_conviction_voting_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 39_000_000 picoseconds.
		Weight::from_parts(13_306_185, 1493)
			// Standard Error: 301_903
			.saturating_add(Weight::from_parts(23_944_483, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Bounties::Bounties` (r:0 w:255)
	/// Proof: `Bounties::Bounties` (`max_values`: None, `max_size`: Some(177), added: 2652, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_bounties_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 20_000_000 picoseconds.
		Weight::from_parts(22_669_470, 1493)
			// Standard Error: 26_425
			.saturating_add(Weight::from_parts(1_245_063, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `AssetRate::ConversionRateToNative` (r:0 w:255)
	/// Proof: `AssetRate::ConversionRateToNative` (`max_values`: None, `max_size`: Some(1238), added: 3713, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_asset_rates(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 21_000_000 picoseconds.
		Weight::from_parts(23_643_974, 1493)
			// Standard Error: 42_063
			.saturating_add(Weight::from_parts(3_048_640, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `AhOps::RcCrowdloanContribution` (r:255 w:255)
	/// Proof: `AhOps::RcCrowdloanContribution` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_crowdloan_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `1493 + n * (2587 ±0)`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(42_000_000, 1493)
			// Standard Error: 954_888
			.saturating_add(Weight::from_parts(29_135_687, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2587).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Referenda::MetadataOf` (r:0 w:255)
	/// Proof: `Referenda::MetadataOf` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_referenda_metadata(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 16_000_000 picoseconds.
		Weight::from_parts(20_160_738, 1493)
			// Standard Error: 26_413
			.saturating_add(Weight::from_parts(1_179_469, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Treasury::Spends` (r:0 w:255)
	/// Proof: `Treasury::Spends` (`max_values`: None, `max_size`: Some(2456), added: 4931, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_treasury_messages(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1493`
		// Minimum execution time: 24_000_000 picoseconds.
		Weight::from_parts(25_294_352, 1493)
			// Standard Error: 49_188
			.saturating_add(Weight::from_parts(4_475_577, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `System::Account` (r:255 w:255)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_preimage_legacy_status(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `52 + n * (135 ±0)`
		//  Estimated: `1493 + n * (2603 ±0)`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(28_000_000, 1493)
			// Standard Error: 1_011_665
			.saturating_add(Weight::from_parts(14_491_791, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 2603).saturating_mul(n.into()))
	}
	/// Storage: `Preimage::RequestStatusFor` (r:255 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::DmpDataMessageCounts` (r:1 w:1)
	/// Proof: `AhMigrator::DmpDataMessageCounts` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 255]`.
	fn receive_preimage_request_status(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `204 + n * (47 ±0)`
		//  Estimated: `1493 + n * (2566 ±0)`
		// Minimum execution time: 29_000_000 picoseconds.
		Weight::from_parts(29_000_000, 1493)
			// Standard Error: 171_655
			.saturating_add(Weight::from_parts(8_425_567, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 2566).saturating_mul(n.into()))
	}
	/// Storage: `AhMigrator::AhMigrationStage` (r:1 w:1)
	/// Proof: `AhMigrator::AhMigrationStage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn force_set_stage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `1486`
		// Minimum execution time: 12_000_000 picoseconds.
		Weight::from_parts(12_000_000, 1486)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	/// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::AhMigrationStage` (r:1 w:1)
	/// Proof: `AhMigrator::AhMigrationStage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::AhTotalIssuanceBefore` (r:0 w:1)
	/// Proof: `AhMigrator::AhTotalIssuanceBefore` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	fn start_migration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `185`
		//  Estimated: `3650`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(37_000_000, 3650)
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::AhTotalIssuanceBefore` (r:1 w:0)
	/// Proof: `AhMigrator::AhTotalIssuanceBefore` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `AhMigrator::AhMigrationStage` (r:1 w:1)
	/// Proof: `AhMigrator::AhMigrationStage` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	fn finish_migration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `109`
		//  Estimated: `3593`
		// Minimum execution time: 18_000_000 picoseconds.
		Weight::from_parts(18_000_000, 3593)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
