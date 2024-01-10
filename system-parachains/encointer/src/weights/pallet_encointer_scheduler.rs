
//! Autogenerated weights for `pallet_encointer_scheduler`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `caribe`, CPU: `12th Gen Intel(R) Core(TM) i7-1260P`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("encointer-rococo-local-dev"), DB CACHE: 1024

// Executed Command:
// target/release/encointer-collator
// benchmark
// pallet
// --chain=encointer-rococo-local-dev
// --steps=50
// --repeat=20
// --pallet=pallet_encointer_scheduler
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=polkadot-parachains/encointer-runtime/src/weights/pallet_encointer_scheduler.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_encointer_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_encointer_scheduler::WeightInfo for WeightInfo<T> {
	/// Storage: EncointerScheduler CurrentPhase (r:1 w:1)
	/// Proof: EncointerScheduler CurrentPhase (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: EncointerScheduler CurrentCeremonyIndex (r:1 w:1)
	/// Proof: EncointerScheduler CurrentCeremonyIndex (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:1)
	/// Proof: EncointerScheduler NextPhaseTimestamp (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: EncointerScheduler PhaseDurations (r:3 w:0)
	/// Proof: EncointerScheduler PhaseDurations (max_values: None, max_size: Some(25), added: 2500, mode: MaxEncodedLen)
	/// Storage: Timestamp Now (r:1 w:0)
	/// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: EncointerCeremonies ReputationLifetime (r:1 w:0)
	/// Proof Skipped: EncointerCeremonies ReputationLifetime (max_values: Some(1), max_size: None, mode: Measured)
	fn next_phase() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `409`
		//  Estimated: `8490`
		// Minimum execution time: 61_397_000 picoseconds.
		Weight::from_parts(63_823_000, 0)
			.saturating_add(Weight::from_parts(0, 8490))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: EncointerScheduler NextPhaseTimestamp (r:1 w:1)
	/// Proof: EncointerScheduler NextPhaseTimestamp (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn push_by_one_day() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `73`
		//  Estimated: `1493`
		// Minimum execution time: 14_946_000 picoseconds.
		Weight::from_parts(15_438_000, 0)
			.saturating_add(Weight::from_parts(0, 1493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerScheduler PhaseDurations (r:0 w:1)
	/// Proof: EncointerScheduler PhaseDurations (max_values: None, max_size: Some(25), added: 2500, mode: MaxEncodedLen)
	fn set_phase_duration() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_814_000 picoseconds.
		Weight::from_parts(6_076_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: EncointerScheduler NextPhaseTimestamp (r:0 w:1)
	/// Proof: EncointerScheduler NextPhaseTimestamp (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	fn set_next_phase_timestamp() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_283_000 picoseconds.
		Weight::from_parts(4_476_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
