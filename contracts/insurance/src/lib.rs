#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String};

mod state;
mod instructions;
mod constant;

use state::*;
use instructions::*;

#[contract]
pub struct VillageMicroInsuranceContract;


#[contractimpl]
impl VillageMicroInsuranceContract {
    
    
    pub fn register_user(env: Env, user: Address, name: Option<String>) -> bool {
        user_management::register_user(&env, user, name).unwrap()
    }

    pub fn approve_user(env: Env, user: Address, approver: Address) -> bool {
        user_management::approve_user(&env, user, approver).unwrap()
    }

    pub fn suspend_user(env: Env, user: Address, admin: Address, reason: String) -> bool {
        user_management::suspend_user(&env, user, admin, reason).unwrap()
    }

    pub fn ban_user(env: Env, user: Address, admin: Address, reason: String) -> bool {
        user_management::ban_user(&env, user, admin, reason).unwrap()
    }

    pub fn add_council_member(env: Env, new_member: Address, appointer: Address) -> bool {
        user_management::add_council_member(&env, new_member, appointer).unwrap()
    }

    
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        proposal_type: ProposalType,
        title: String,
        description: String,
        execution_data: Bytes,
        required_quorum: u32,
    ) -> u64 {
        dao_governance::create_proposal(&env, proposer, proposal_type, title, description, execution_data, required_quorum).unwrap()
    }

    pub fn vote_on_proposal(env: Env, proposal_id: u64, voter: Address, vote_for: bool, vote_weight: i128) -> bool {
        dao_governance::vote_on_proposal(&env, proposal_id, voter, vote_for, vote_weight).unwrap()
    }

    pub fn execute_proposal(env: Env, proposal_id: u64, executor: Address) -> bool {
        dao_governance::execute_proposal(&env, proposal_id, executor).unwrap()
    }

    
    pub fn get_user(env: Env, user: Address) -> Option<User> {
        user_management::get_user(&env, user).ok()
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<Proposal> {
        dao_governance::get_proposal(&env, proposal_id).ok()
    }

    
    pub fn get_safety_pool_balance(env: Env) -> i128 {
        financial_management::get_safety_pool_balance(&env)
    }

    pub fn add_external_funding(env: Env, funder: Address, amount: i128) -> bool {
        financial_management::add_external_funding(&env, funder, amount)
    }

    pub fn withdraw_reserve_funds(env: Env, withdrawer: Address, amount: i128, purpose: String) -> bool {
        financial_management::withdraw_reserve_funds(&env, withdrawer, amount, purpose)
    }

    pub fn conduct_financial_audit(env: Env, auditor: Address) -> bool {
        financial_management::conduct_financial_audit(&env, auditor)
    }

    pub fn emergency_fund_freeze(env: Env, freezer: Address, reason: String) -> bool {
        financial_management::emergency_fund_freeze(&env, freezer, reason)
    }

    pub fn emergency_fund_unfreeze(env: Env, unfreezer: Address) -> bool {
        financial_management::emergency_fund_unfreeze(&env, unfreezer)
    }

    pub fn get_financial_summary(env: Env) -> (i128, i128, i128, i128) {
        financial_management::get_financial_summary(&env)
    }

    
    pub fn is_fund_frozen(env: Env) -> bool {
        financial_management::is_fund_frozen(&env)
    }

    pub fn check_reserve_health(env: Env) -> (bool, i128, i128) {
        financial_management::check_reserve_health(&env)
    }
}
