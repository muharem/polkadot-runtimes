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

//! Autogenerated weights for `pallet_glutton`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-01-05, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `ggwpez-ref-hw`, CPU: `AMD EPYC 7232P 8-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("./glutton-kusama-chain-spec.json")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=./glutton-kusama-chain-spec.json
// --steps=50
// --repeat=20
// --pallet=pallet_glutton
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./glutton-kusama-weights/
// --header=./file_header.txt

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_glutton`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_glutton::WeightInfo for WeightInfo<T> {
	/// Storage: `Glutton::TrashDataCount` (r:1 w:1)
	/// Proof: `Glutton::TrashDataCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::TrashData` (r:0 w:1000)
	/// Proof: `Glutton::TrashData` (`max_values`: Some(65000), `max_size`: Some(1036), added: 3016, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_grow(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1489`
		// Minimum execution time: 10_880_000 picoseconds.
		Weight::from_parts(10_960_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 3_555
			.saturating_add(Weight::from_parts(13_399_149, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// Storage: `Glutton::TrashDataCount` (r:1 w:1)
	/// Proof: `Glutton::TrashDataCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::TrashData` (r:0 w:1000)
	/// Proof: `Glutton::TrashData` (`max_values`: Some(65000), `max_size`: Some(1036), added: 3016, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 1000]`.
	fn initialize_pallet_shrink(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147`
		//  Estimated: `1489`
		// Minimum execution time: 10_490_000 picoseconds.
		Weight::from_parts(10_670_000, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 2_158
			.saturating_add(Weight::from_parts(1_456_795, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
	}
	/// The range of component `i` is `[0, 100000]`.
	fn waste_ref_time_iter(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 820_000 picoseconds.
		Weight::from_parts(880_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 4
			.saturating_add(Weight::from_parts(126_641, 0).saturating_mul(i.into()))
	}
	/// Storage: `Glutton::TrashData` (r:5000 w:0)
	/// Proof: `Glutton::TrashData` (`max_values`: Some(65000), `max_size`: Some(1036), added: 3016, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[0, 5000]`.
	fn waste_proof_size_some(i: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `119142 + i * (1022 ±0)`
		//  Estimated: `990 + i * (3016 ±0)`
		// Minimum execution time: 760_000 picoseconds.
		Weight::from_parts(472_825_372, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 4_079
			.saturating_add(Weight::from_parts(8_097_053, 0).saturating_mul(i.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(i.into())))
			.saturating_add(Weight::from_parts(0, 3016).saturating_mul(i.into()))
	}
	/// Storage: `Glutton::Storage` (r:1 w:0)
	/// Proof: `Glutton::Storage` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::Compute` (r:1 w:0)
	/// Proof: `Glutton::Compute` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::TrashData` (r:1737 w:0)
	/// Proof: `Glutton::TrashData` (`max_values`: Some(65000), `max_size`: Some(1036), added: 3016, mode: `MaxEncodedLen`)
	fn on_idle_high_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1900525`
		//  Estimated: `5239782`
		// Minimum execution time: 98_756_541_000 picoseconds.
		Weight::from_parts(99_731_588_000, 0)
			.saturating_add(Weight::from_parts(0, 5239782))
			.saturating_add(T::DbWeight::get().reads(1739))
	}
	/// Storage: `Glutton::Storage` (r:1 w:0)
	/// Proof: `Glutton::Storage` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::Compute` (r:1 w:0)
	/// Proof: `Glutton::Compute` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::TrashData` (r:5 w:0)
	/// Proof: `Glutton::TrashData` (`max_values`: Some(65000), `max_size`: Some(1036), added: 3016, mode: `MaxEncodedLen`)
	fn on_idle_low_proof_waste() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9575`
		//  Estimated: `16070`
		// Minimum execution time: 98_242_689_000 picoseconds.
		Weight::from_parts(99_206_555_000, 0)
			.saturating_add(Weight::from_parts(0, 16070))
			.saturating_add(T::DbWeight::get().reads(7))
	}
	/// Storage: `Glutton::Storage` (r:1 w:0)
	/// Proof: `Glutton::Storage` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Glutton::Compute` (r:1 w:0)
	/// Proof: `Glutton::Compute` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn empty_on_idle() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `1493`
		// Minimum execution time: 7_000_000 picoseconds.
		Weight::from_parts(7_200_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `Glutton::Compute` (r:0 w:1)
	/// Proof: `Glutton::Compute` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set_compute() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_750_000 picoseconds.
		Weight::from_parts(5_910_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Glutton::Storage` (r:0 w:1)
	/// Proof: `Glutton::Storage` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn set_storage() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_820_000 picoseconds.
		Weight::from_parts(6_010_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
