multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use common_structs::*;

use crate::attr_ex_helper::{self, PRECISION_EX_INCREASE};

pub const ONE_MILLION: u64 = 1_000_000u64;
pub const TEN_THOUSAND: u64 = 10_000u64;
pub const PERCENTAGE_TOTAL_EX: u64 = 100_000u64;
pub const MAX_MILESTONES_IN_SCHEDULE: usize = 64;
pub const DOUBLE_MAX_MILESTONES_IN_SCHEDULE: usize = 2 * MAX_MILESTONES_IN_SCHEDULE;

use core::fmt::Debug;

#[derive(ManagedVecItem)]
pub struct LockedTokenEx<M: ManagedTypeApi> {
    pub token_amount: EsdtTokenPayment<M>,
    pub attributes: LockedAssetTokenAttributesEx<M>,
}

#[derive(ManagedVecItem, Clone, Debug)]
pub struct EpochAmountPair<M: ManagedTypeApi> {
    pub epoch: u64,
    pub amount: BigUint<M>,
}

#[multiversx_sc::module]
pub trait LockedAssetModule: attr_ex_helper::AttrExHelper {
    fn create_and_send_locked_assets(
        &self,
        amount: &BigUint,
        additional_amount_to_create: &BigUint,
        address: &ManagedAddress,
        attributes: &LockedAssetTokenAttributesEx<Self::Api>,
    ) -> Nonce {
        let token_id = self.locked_asset_token_id().get();
        let last_created_nonce = self.send().esdt_nft_create_compact(
            &token_id,
            &(amount + additional_amount_to_create),
            attributes,
        );
        self.send()
            .direct_esdt(address, &token_id, last_created_nonce, amount);

        last_created_nonce
    }

    fn add_quantity_and_send_locked_assets(
        &self,
        amount: &BigUint,
        sft_nonce: Nonce,
        address: &ManagedAddress,
    ) {
        let token_id = self.locked_asset_token_id().get();
        self.send().esdt_local_mint(&token_id, sft_nonce, amount);

        self.send()
            .direct_esdt(address, &token_id, sft_nonce, amount);
    }

    fn get_unlock_amount(
        &self,
        amount: &BigUint,
        current_epoch: Epoch,
        unlock_milestones: &ManagedVec<UnlockMilestoneEx>,
    ) -> BigUint {
        amount * &BigUint::from(self.get_unlock_percent(current_epoch, unlock_milestones))
            / PERCENTAGE_TOTAL_EX
    }

    fn get_unlock_percent(
        &self,
        current_epoch: Epoch,
        unlock_milestones: &ManagedVec<UnlockMilestoneEx>,
    ) -> u64 {
        let mut unlock_percent = 0u64;

        for milestone in unlock_milestones.into_iter() {
            if milestone.unlock_epoch <= current_epoch {
                unlock_percent += milestone.unlock_percent;
            }
        }

        require!(
            unlock_percent <= PERCENTAGE_TOTAL_EX,
            "unlock percent greater than max"
        );

        unlock_percent
    }

    fn create_new_unlock_milestones(
        &self,
        current_epoch: Epoch,
        old_unlock_milestones: &ManagedVec<UnlockMilestoneEx>,
    ) -> ManagedVec<UnlockMilestoneEx> {
        let unlock_percent = self.get_unlock_percent(current_epoch, old_unlock_milestones);
        let unlock_percent_remaining = PERCENTAGE_TOTAL_EX - unlock_percent;

        if unlock_percent_remaining == 0 {
            return ManagedVec::new();
        }

        let mut unlock_milestones_merged =
            ArrayVec::<UnlockMilestoneEx, MAX_MILESTONES_IN_SCHEDULE>::new();
        for old_milestone in old_unlock_milestones.iter() {
            if old_milestone.unlock_epoch > current_epoch {
                let new_unlock_percent =
                    old_milestone.unlock_percent * PRECISION_EX_INCREASE * ONE_MILLION
                        / unlock_percent_remaining;
                unlock_milestones_merged.push(UnlockMilestoneEx {
                    unlock_epoch: old_milestone.unlock_epoch,
                    unlock_percent: new_unlock_percent,
                });
            }
        }

        self.distribute_leftover(&mut unlock_milestones_merged);
        self.get_non_zero_percent_milestones_as_vec(&unlock_milestones_merged)
    }

    fn distribute_leftover(
        &self,
        unlock_milestones_merged: &mut ArrayVec<UnlockMilestoneEx, MAX_MILESTONES_IN_SCHEDULE>,
    ) {
        let mut sum_of_new_percents = 0u64;
        for milestone in unlock_milestones_merged.iter() {
            sum_of_new_percents += milestone.unlock_percent / TEN_THOUSAND;
        }
        let mut leftover = PERCENTAGE_TOTAL_EX - sum_of_new_percents;

        while leftover != 0 {
            let mut max_rounding_error = 0;
            let mut max_rounding_error_index = 0;
            for index in 0..unlock_milestones_merged.len() {
                let rounding_error = unlock_milestones_merged[index].unlock_percent % TEN_THOUSAND;
                if rounding_error >= max_rounding_error {
                    max_rounding_error = rounding_error;
                    max_rounding_error_index = index;
                }
            }

            leftover -= 1;
            unlock_milestones_merged[max_rounding_error_index].unlock_percent =
                ((unlock_milestones_merged[max_rounding_error_index].unlock_percent
                    / TEN_THOUSAND)
                    + 1)
                    * TEN_THOUSAND;
        }
    }

    fn get_non_zero_percent_milestones_as_vec(
        &self,
        unlock_milestones_merged: &ArrayVec<UnlockMilestoneEx, MAX_MILESTONES_IN_SCHEDULE>,
    ) -> ManagedVec<UnlockMilestoneEx> {
        let mut new_unlock_milestones = ManagedVec::new();

        for el in unlock_milestones_merged.iter() {
            let percent_rounded = el.unlock_percent / TEN_THOUSAND;
            if percent_rounded != 0 {
                new_unlock_milestones.push(UnlockMilestoneEx {
                    unlock_epoch: el.unlock_epoch,
                    unlock_percent: percent_rounded,
                });
            }
        }

        new_unlock_milestones
    }

    fn validate_unlock_milestones(&self, unlock_milestones: &ManagedVec<UnlockMilestone>) {
        require!(!unlock_milestones.is_empty(), "Empty param");

        let mut percents_sum: u8 = 0;
        let mut last_milestone_unlock_epoch: u64 = 0;

        for milestone in unlock_milestones.into_iter() {
            require!(
                milestone.unlock_epoch >= last_milestone_unlock_epoch,
                "Unlock epochs not in order"
            );
            require!(
                milestone.unlock_percent <= 100,
                "Unlock percent more than 100"
            );
            last_milestone_unlock_epoch = milestone.unlock_epoch;
            percents_sum += milestone.unlock_percent;
        }

        require!(percents_sum == 100, "Percents do not sum up to 100");
    }

    fn mint_and_send_assets(&self, dest: &ManagedAddress, amount: &BigUint) {
        if amount > &0 {
            let asset_token_id = self.asset_token_id().get();
            self.send().esdt_local_mint(&asset_token_id, 0, amount);
            self.send().direct_esdt(dest, &asset_token_id, 0, amount);
        }
    }

    #[view(getLockedAssetTokenId)]
    #[storage_mapper("locked_asset_token_id")]
    fn locked_asset_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getAssetTokenId)]
    #[storage_mapper("asset_token_id")]
    fn asset_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
}
