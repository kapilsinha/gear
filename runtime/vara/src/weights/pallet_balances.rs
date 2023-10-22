// This file is part of Gear.

// Copyright (C) 2022-2023 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_balances
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-10-20, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("vara-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/gear benchmark pallet --chain=vara-dev --steps=50 --repeat=20 --pallet=pallet_balances --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./scripts/benchmarking/weights-output/pallet_balances.rs --template=.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_balances.
pub trait WeightInfo {
    fn transfer() -> Weight;
    fn transfer_keep_alive() -> Weight;
    fn set_balance_creating() -> Weight;
    fn set_balance_killing() -> Weight;
    fn force_transfer() -> Weight;
    fn transfer_all() -> Weight;
    fn force_unreserve() -> Weight;
}

/// Weights for pallet_balances using the Gear node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_balances::WeightInfo for SubstrateWeight<T> {
    fn transfer() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `6196`
        // Minimum execution time: 41_686_000 picoseconds.
        Weight::from_parts(42_906_000, 6196)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    fn transfer_keep_alive() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `3593`
        // Minimum execution time: 22_118_000 picoseconds.
        Weight::from_parts(23_031_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn set_balance_creating() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 12_641_000 picoseconds.
        Weight::from_parts(13_030_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn set_balance_killing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 15_587_000 picoseconds.
        Weight::from_parts(16_204_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn force_transfer() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `8799`
        // Minimum execution time: 43_554_000 picoseconds.
        Weight::from_parts(44_259_000, 8799)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    fn transfer_all() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `3593`
        // Minimum execution time: 29_622_000 picoseconds.
        Weight::from_parts(30_249_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    fn force_unreserve() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 12_304_000 picoseconds.
        Weight::from_parts(12_677_000, 3593)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn transfer() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `6196`
        // Minimum execution time: 41_686_000 picoseconds.
        Weight::from_parts(42_906_000, 6196)
            .saturating_add(RocksDbWeight::get().reads(2_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    fn transfer_keep_alive() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `3593`
        // Minimum execution time: 22_118_000 picoseconds.
        Weight::from_parts(23_031_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn set_balance_creating() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 12_641_000 picoseconds.
        Weight::from_parts(13_030_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn set_balance_killing() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 15_587_000 picoseconds.
        Weight::from_parts(16_204_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn force_transfer() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `206`
        //  Estimated: `8799`
        // Minimum execution time: 43_554_000 picoseconds.
        Weight::from_parts(44_259_000, 8799)
            .saturating_add(RocksDbWeight::get().reads(3_u64))
            .saturating_add(RocksDbWeight::get().writes(3_u64))
    }
    fn transfer_all() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `3593`
        // Minimum execution time: 29_622_000 picoseconds.
        Weight::from_parts(30_249_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
    fn force_unreserve() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `103`
        //  Estimated: `3593`
        // Minimum execution time: 12_304_000 picoseconds.
        Weight::from_parts(12_677_000, 3593)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}
