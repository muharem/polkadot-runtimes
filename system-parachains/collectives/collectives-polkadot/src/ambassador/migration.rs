// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Migrations.

use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade, weights::Weight};
use log;

/// Ambassador Program Seeding.
pub(crate) mod ambassador_seeding {
	use super::*;
	use frame_support::{parameter_types, traits::RankedMembers};
	use pallet_ranked_collective::{Config, MemberCount, Pallet as RankedCollective, Rank};
	#[cfg(feature = "try-runtime")]
	use sp_std::vec::Vec;

	const TARGET: &str = "runtime::migration::ambassador_seeding";

	parameter_types! {
		// Ambassadors addresses.
		pub const AmbassadorsAddresses: [(Rank, [u8; 32]); 20] = [
			(6, hex_literal::hex!("bc6e12d7ab70abea4c08db7055e84f16bab817b5fb359088ad5190422df9dd1d"),),
			(6, hex_literal::hex!("a65626bf6bd3d70916fea48c93358cbf1e97f4f7f5ce8d5a292719e2555de66b"),),
			(6, hex_literal::hex!("383d052b70007f5382342d61205c50e75336c57eda16b84db766278420ada25a"),),
			(6, hex_literal::hex!("9a5e3760bdedaa34c256e0b28279d358b68a28cfd5967bd0f8cd5555496fa655"),),
			(6, hex_literal::hex!("52582c7985dc8002e718841421ee5059a92ed9a4e38c0c33365ee196c96d5e0a"),),
			(6, hex_literal::hex!("0842fe19dee2029fe63ae78b250fb0ed99ebcc6a72b36e7f56160150c230f237"),),
			(6, hex_literal::hex!("90ebd94a58a07211dcf05c4164ebab8c3abaca45f16f793fbe34072ae7b9ba08"),),
			(6, hex_literal::hex!("0776a6d2dc585e66545d1538c8d8db9a221e7b67591790941d049992973e360d"),),
			(6, hex_literal::hex!("88f28e17671ba1808d7b02cd3caaf80113066a467127666f4d80afc50bfbc127"),),
			(5, hex_literal::hex!("cc10dd1946b0fc65c8993ff7f47052713e9aa4b1cb72c913bd397c34adf4f949"),),
			(5, hex_literal::hex!("2add0af948eba3b1fcd5cacde1f6fcc70f11ef75056f88ca4d11dcc5b080220e"),),
			(5, hex_literal::hex!("d4e8bfb1c924dd64e33ecfbb35d90061bb83b2dde667e58588780068f9fc1471"),),
			(5, hex_literal::hex!("ce2490656709c33bdd50d72dc0ec562bb72db84945ef7a1be45be14bbc6fc877"),),
			(5, hex_literal::hex!("5e97fb5939fdad5bead83976cdea2882d2cb55d9bfc38421ed87aa46711ec625"),),
			(5, hex_literal::hex!("29f107a6abccb303dcdd5d05569389f87a411d192e8463829a9c84a95a5b3ed1"),),
			(5, hex_literal::hex!("568191edc1aaf4bea93b17cf53ea49ab78e2d25d83dec8581854be93d3bc9609"),),
			(5, hex_literal::hex!("5071733a41703f6cf3fe43daa2814861b512205451a2f33a60a8b3d739eab20e"),),
			(5, hex_literal::hex!("0a9e6fb48b9423ae5939f07fe1d7f41a6c1ebe3d2135e94af94b7fd717a47418"),),
			(5, hex_literal::hex!("9681fe005baa3099a7d9c03a46e539b173d2b3de75b11e86cd5fbb7d4e92993c"),),
			(5, hex_literal::hex!("f4f5911143092b1788c3a91f96d192b35e4fb201f05658f47b677e7dc9a1fd5d"),),
		];
	}

	/// Implements `OnRuntimeUpgrade` trait.
	pub struct Migration<T, I = ()>(PhantomData<(T, I)>);

	impl<T: Config<I>, I: 'static> OnRuntimeUpgrade for Migration<T, I>
	where
		<T as frame_system::Config>::AccountId: From<[u8; 32]>,
	{
		#[cfg(feature = "try-runtime")]
		fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
			let onchain_version = RankedCollective::<T, I>::on_chain_storage_version();
			ensure!(onchain_version == 0, "the storage version must be 0.");
			let member_count = MemberCount::<T, I>::get(0);
			ensure!(member_count == 0, "the collective must be uninitialized.");

			Ok(Vec::new())
		}

		fn on_runtime_upgrade() -> Weight {
			let current_version = RankedCollective::<T, I>::current_storage_version();
			let onchain_version = RankedCollective::<T, I>::on_chain_storage_version();
			let mut weight = T::DbWeight::get().reads(1);
			log::info!(
				target: TARGET,
				"running migration with current storage version {:?} / onchain {:?}.",
				current_version,
				onchain_version
			);
			if onchain_version != 0 {
				log::warn!(
					target: TARGET,
					"unsupported storage version, skipping ambassador_seeding migration."
				);
				return weight
			}
			let member_count = MemberCount::<T, I>::get(0);
			weight.saturating_accrue(T::DbWeight::get().reads(1));
			if member_count != 0 {
				log::warn!(
					target: TARGET,
					"the collective already initialized, skipping ambassador_seeding migration."
				);
				return weight
			}

			for (rank, account_id32) in AmbassadorsAddresses::get() {
				let who: T::AccountId = account_id32.into();
				let _ = <RankedCollective<T, I> as RankedMembers>::induct(&who);
				for _ in 0..rank {
					let _ = <RankedCollective<T, I> as RankedMembers>::promote(&who);
					// 1 write to `IdToIndex` and `IndexToId` per member on each rank.
					weight.saturating_accrue(T::DbWeight::get().writes(2));
				}
				// 1 write to `IdToIndex` and `IndexToId` per member on each rank.
				weight.saturating_accrue(T::DbWeight::get().writes(2));
				// 1 read and 1 write to `Members` and `MemberCount` per member.
				weight.saturating_accrue(T::DbWeight::get().reads_writes(2, 2));
			}
			weight
		}

		#[cfg(feature = "try-runtime")]
		fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
			ensure!(MemberCount::<T, I>::get(0) == 20, "invalid members count at rank 0.");
			ensure!(MemberCount::<T, I>::get(1) == 20, "invalid members count at rank 1.");
			ensure!(MemberCount::<T, I>::get(2) == 20, "invalid members count at rank 2.");
			ensure!(MemberCount::<T, I>::get(3) == 20, "invalid members count at rank 3.");
			ensure!(MemberCount::<T, I>::get(4) == 20, "invalid members count at rank 4.");
			ensure!(MemberCount::<T, I>::get(5) == 20, "invalid members count at rank 5.");
			ensure!(MemberCount::<T, I>::get(6) == 9, "invalid members count at rank 6.");
			Ok(())
		}
	}
}

#[cfg(test)]
pub mod tests {
	use super::ambassador_seeding::AmbassadorsAddresses;
	use crate::{ambassador::AmbassadorCollectiveInstance as Ambassador, Runtime, System};
	use frame_support::traits::OnRuntimeUpgrade;
	use pallet_ranked_collective::Rank;
	use parachains_common::AccountId;
	use sp_core::crypto::Ss58Codec;
	use sp_runtime::{AccountId32, BuildStorage};

	#[test]
	fn check_ambassadors_addresses() {
		let ambassadors_addresses = AmbassadorsAddresses::get();
		let ambassadors_addresses_ss58: [(Rank, _); 20] = [
			(6, "15G4hfDNtNhRc82As8Ep2YfvpM5xVdX7De3P9qSdHerGA6wC"),
			(6, "14m6YPTpa64jusibkvvCov4XsxW2ig7Hy3n23GpN9YVTqhUd"),
			(6, "12GjotDNTmRsZHAQrZp9rwwRKRJftMkbD9Xht47Ky6MMibXR"),
			(6, "14VQN1uR6XdyhnyyskbhvE93uYyAnK1roUBNC6Qf7KkUwLap"),
			(6, "12ry7wPEGeBSceWTZXgMTpqLDuP4Kd6rLJYUx3qVwF1gbSNs"),
			(6, "1BqHUUwZ6kuRhFZTdbhPwSmwxVnGdCVJTsUf1FU2DXWtEwc"),
			(6, "14H1xWm7LFz8u9FJ2roq9DDymUMyARz5gY736i5TMaSwvKC3"),
			(6, "1AnamB4ZknHy7knpqevWFjQ7kWtdqLBFWie3AdBZaadEUUV"),
			(6, "146ZZqm2cMHLf3ju7oc8M9JnPaAktuADAKThagKnXqzjPJbZ"),
			(5, "15cZn8K1DaE7qiBWK6mGFJMKYKjFrALTVwe5urpD9PzKSsPY"),
			(5, "1yCg8NSCgjS4K5KDK5DZGhxUCxmgVyhyG6vBPn5wqUmLuYo"),
			(5, "15pAHxckdkyCXJEXZkRtWxw1gbwjATkFejxKDC4q9FUKUTFH"),
			(5, "15fHj7Q7SYxqMgZ38UpjXS8cxdq77rczTP3JgY9JVi5piMPN"),
			(5, "1392eetyWwshYo5xQddbZEV1dNuEeaNXEHAdHDUfEokceUMa"),
			(5, "1wzZiegQfdppPw7Ag3vhhXbYuw47uJLJ5G6eXcKc6feTS5x"),
			(5, "12xRcHjvStkUYgzTh9vyqinN3tUpddgcPnSUcLXZ3ty44Mq1"),
			(5, "12pUXtKj7jD3yaxnK6TWKBsxS1JpLUceBKZKMPMH6RiZvjr4"),
			(5, "1EvYZwJ25wgUzMqJ61AadwTtaHr6BVosPcyK4goohUDqmjY"),
			(5, "14QLoJzkneWvzWcLKXnnr32h9q66Cp8KyWSQvwhfxqpbNRh5"),
			(5, "16YBdgR2NXVfcgVx8vmGimJMo8CFzULktruh1bTDUnY2Ss6Q"),
		];

		for (index, val) in ambassadors_addresses_ss58.iter().enumerate() {
			let account: AccountId32 = <AccountId as Ss58Codec>::from_string(val.1).unwrap();
			let account32: [u8; 32] = account.clone().into();
			// println!("hex and 8 bytes of ss58: {:?}", account);
			assert_eq!(
				ambassadors_addresses[index].0, ambassadors_addresses_ss58[index].0,
				"ranks must be equal at index '{}'.",
				index
			);
			assert_eq!(
				ambassadors_addresses[index].1, account32,
				"accounts must be equal at index '{}'.",
				index
			);
		}
	}

	#[test]
	fn test_ambassador_seeding() {
		use super::ambassador_seeding::Migration;
		use pallet_ranked_collective::{IdToIndex, IndexToId, MemberCount, MemberRecord, Members};

		let t = frame_system::GenesisConfig::<Runtime>::default().build_storage().unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext.execute_with(|| {
			assert_eq!(MemberCount::<Runtime, Ambassador>::get(0), 0);
			Migration::<Runtime, Ambassador>::on_runtime_upgrade();
			assert_eq!(MemberCount::<Runtime, Ambassador>::get(0), 20);
			assert_eq!(MemberCount::<Runtime, Ambassador>::get(5), 20);
			assert_eq!(MemberCount::<Runtime, Ambassador>::get(6), 9);
			for (rank, account_id32) in AmbassadorsAddresses::get() {
				let who = <Runtime as frame_system::Config>::AccountId::from(account_id32);
				assert!(IdToIndex::<Runtime, Ambassador>::get(0, &who).is_some());
				assert!(IdToIndex::<Runtime, Ambassador>::get(rank + 1, &who).is_none());
				let index = IdToIndex::<Runtime, Ambassador>::get(rank, &who).unwrap();
				assert_eq!(IndexToId::<Runtime, Ambassador>::get(rank, index).unwrap(), who);
				assert_eq!(
					Members::<Runtime, Ambassador>::get(&who).unwrap(),
					MemberRecord::new(rank)
				);
			}
		});
	}
}
