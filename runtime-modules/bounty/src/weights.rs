// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for bounty
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-31, STEPS: `5`, REPEAT: 2, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/joystream-node
// benchmark
// pallet
// --pallet=bounty
// --extrinsic=*
// --chain=dev
// --steps=5
// --repeat=2
// --execution=wasm
// --template=./scripts/../devops/joystream-pallet-weight-template.hbs
// --output=.

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for bounty.
pub trait WeightInfo {
	fn create_bounty_by_council(_i: u32, _j: u32, ) -> Weight;
	fn create_bounty_by_member(_i: u32, _j: u32, ) -> Weight;
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight;
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight;
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight;
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight;
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight;
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight;
	fn terminate_bounty_work_or_judging_period() -> Weight;
	fn fund_bounty_by_member() -> Weight;
	fn fund_bounty_by_council() -> Weight;
	fn withdraw_funding_by_member() -> Weight;
	fn withdraw_funding_by_council() -> Weight;
	fn announce_work_entry(_i: u32, _j: u32, ) -> Weight;
	fn submit_work(_i: u32, ) -> Weight;
	fn submit_oracle_judgment_by_council_all_winners(_i: u32, _j: u32, ) -> Weight;
	fn submit_oracle_judgment_by_council_all_rejected(_i: u32, _j: u32, _k: u32, ) -> Weight;
	fn submit_oracle_judgment_by_member_all_winners(_i: u32, _j: u32, ) -> Weight;
	fn submit_oracle_judgment_by_member_all_rejected(_i: u32, _j: u32, _k: u32, ) -> Weight;
	fn switch_oracle_to_council_by_council_successful() -> Weight;
	fn switch_oracle_to_member_by_oracle_council() -> Weight;
	fn switch_oracle_to_member_by_council() -> Weight;
	fn switch_oracle_to_member_by_oracle_member() -> Weight;
	fn switch_oracle_to_council_by_oracle_member() -> Weight;
	fn end_working_period() -> Weight;
	fn withdraw_entrant_stake() -> Weight;
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight;
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight;
	fn withdraw_oracle_reward_by_oracle_council() -> Weight;
	fn withdraw_oracle_reward_by_oracle_member() -> Weight;
	fn entrant_remark(_i: u32, ) -> Weight;
	fn contributor_remark(_i: u32, ) -> Weight;
	fn oracle_remark(_i: u32, ) -> Weight;
	fn creator_remark(_i: u32, ) -> Weight;
}

/// Weights for bounty using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:50 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbee37921d32604f381943d4150ed81c7eb] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:0 w:1)
	fn create_bounty_by_council(i: u32, j: u32, ) -> Weight {
		(68_882_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 344_000
			.saturating_add((6_506_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:51 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbee37921d32604f381943d4150ed81c7eb] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:0 w:1)
	fn create_bounty_by_member(i: u32, j: u32, ) -> Weight {
		(86_950_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 469_000
			.saturating_add((6_618_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(j as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight {
		(84_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight {
		(112_430_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight {
		(83_334_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight {
		(112_015_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:2 w:0)
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight {
		(51_150_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:2 w:0)
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight {
		(60_390_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:2 w:0)
	fn terminate_bounty_work_or_judging_period() -> Weight {
		(51_717_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn fund_bounty_by_member() -> Weight {
		(92_454_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn fund_bounty_by_council() -> Weight {
		(85_789_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn withdraw_funding_by_member() -> Weight {
		(89_777_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn withdraw_funding_by_council() -> Weight {
		(85_749_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc8b18453086fa74ddec96f7b48109d8f4] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe3a91370312d2084e43ce7f10bf343152] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:0 w:1)
	fn announce_work_entry(i: u32, j: u32, ) -> Weight {
		(81_567_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 322_000
			.saturating_add((822_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	fn submit_work(i: u32, ) -> Weight {
		(52_762_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	fn submit_oracle_judgment_by_council_all_winners(i: u32, j: u32, ) -> Weight {
		(90_198_000 as Weight)
			// Standard Error: 638_000
			.saturating_add((72_534_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 1_000
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn submit_oracle_judgment_by_council_all_rejected(i: u32, _j: u32, k: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 4_332_000
			.saturating_add((161_759_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 8_000
			.saturating_add((142_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	fn submit_oracle_judgment_by_member_all_winners(i: u32, j: u32, ) -> Weight {
		(99_620_000 as Weight)
			// Standard Error: 501_000
			.saturating_add((72_442_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(j as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn submit_oracle_judgment_by_member_all_rejected(i: u32, j: u32, k: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 4_744_000
			.saturating_add((162_034_000 as Weight).saturating_mul(i as Weight))
			// Standard Error: 9_000
			.saturating_add((3_000 as Weight).saturating_mul(j as Weight))
			// Standard Error: 9_000
			.saturating_add((141_000 as Weight).saturating_mul(k as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((4 as Weight).saturating_mul(i as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(i as Weight)))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	fn switch_oracle_to_council_by_council_successful() -> Weight {
		(41_653_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn switch_oracle_to_member_by_oracle_council() -> Weight {
		(46_669_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn switch_oracle_to_member_by_council() -> Weight {
		(48_013_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:2 w:0)
	fn switch_oracle_to_member_by_oracle_member() -> Weight {
		(52_364_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn switch_oracle_to_council_by_oracle_member() -> Weight {
		(46_570_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	fn end_working_period() -> Weight {
		(46_226_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:1)
	// Storage: unknown [0xc2261276cc9d1f8598ea4b6a74b15c2f218f26c73add634897550b4003b26bc6] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	fn withdraw_entrant_stake() -> Weight {
		(76_201_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight {
		(94_766_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight {
		(88_289_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:1 w:1)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	fn withdraw_oracle_reward_by_oracle_council() -> Weight {
		(119_917_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:1)
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9] (r:2 w:2)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	// Storage: unknown [0xaebd463ed9925c488c112434d61debc0f3928fc443e8d9cca27b4e39e5c29cac] (r:1 w:1)
	fn withdraw_oracle_reward_by_oracle_member() -> Weight {
		(131_494_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbef2c528f439cb4e3ed0c3631044c7e5de] (r:1 w:0)
	fn entrant_remark(i: u32, ) -> Weight {
		(37_632_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: unknown [0x2ce461329fdf4be12bce01afc0af09bc13020dc69e85870ac7b4c755bb8753c2] (r:1 w:0)
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbe8514702e239474a33b476e87a78d382d] (r:1 w:0)
	fn contributor_remark(i: u32, ) -> Weight {
		(37_112_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	fn oracle_remark(i: u32, ) -> Weight {
		(32_387_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	// Storage: unknown [0x43a64b3f1b3826a8520d6a2635c4cdbea37f719efab16103103a0c8c2c784ce1] (r:1 w:0)
	fn creator_remark(i: u32, ) -> Weight {
		(39_423_000 as Weight)
			// Standard Error: 0
			.saturating_add((1_000 as Weight).saturating_mul(i as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
}

// Default implementation for tests
impl WeightInfo for () {
	fn create_bounty_by_council(i: u32, j: u32, ) -> Weight {
		0
	}
	fn create_bounty_by_member(i: u32, j: u32, ) -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_funding_expired() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_funding_expired() -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_wo_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_wo_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_w_oracle_reward_w_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_wo_oracle_reward_w_funds_funding() -> Weight {
		0
	}
	fn terminate_bounty_work_or_judging_period() -> Weight {
		0
	}
	fn fund_bounty_by_member() -> Weight {
		0
	}
	fn fund_bounty_by_council() -> Weight {
		0
	}
	fn withdraw_funding_by_member() -> Weight {
		0
	}
	fn withdraw_funding_by_council() -> Weight {
		0
	}
	fn announce_work_entry(i: u32, j: u32, ) -> Weight {
		0
	}
	fn submit_work(i: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_council_all_winners(i: u32, j: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_council_all_rejected(i: u32, _j: u32, k: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_member_all_winners(i: u32, j: u32, ) -> Weight {
		0
	}
	fn submit_oracle_judgment_by_member_all_rejected(i: u32, j: u32, k: u32, ) -> Weight {
		0
	}
	fn switch_oracle_to_council_by_council_successful() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_oracle_council() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_council() -> Weight {
		0
	}
	fn switch_oracle_to_member_by_oracle_member() -> Weight {
		0
	}
	fn switch_oracle_to_council_by_oracle_member() -> Weight {
		0
	}
	fn end_working_period() -> Weight {
		0
	}
	fn withdraw_entrant_stake() -> Weight {
		0
	}
	fn withdraw_funding_state_bloat_bond_by_council() -> Weight {
		0
	}
	fn withdraw_funding_state_bloat_bond_by_member() -> Weight {
		0
	}
	fn withdraw_oracle_reward_by_oracle_council() -> Weight {
		0
	}
	fn withdraw_oracle_reward_by_oracle_member() -> Weight {
		0
	}
	fn entrant_remark(i: u32, ) -> Weight {
		0
	}
	fn contributor_remark(i: u32, ) -> Weight {
		0
	}
	fn oracle_remark(i: u32, ) -> Weight {
		0
	}
	fn creator_remark(i: u32, ) -> Weight {
		0
	}
}
