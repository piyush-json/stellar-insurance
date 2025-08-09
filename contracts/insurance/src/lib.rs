#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Bytes, Env, String, Vec};

mod state;
mod instructions;
mod constant;

use state::*;
use instructions::*;

#[contract]
pub struct VillageMicroInsuranceContract;


#[contractimpl]
impl VillageMicroInsuranceContract {
    
    // User Management Functions
    pub fn register_user(env: Env, user: Address,) -> bool {
        user_management::register_user(&env, user)
    }

    pub fn approve_user(env: Env, user: Address, approver: Address) -> bool {
        user_management::approve_user(&env, user, approver)
    }

    pub fn suspend_user(env: Env, user: Address, admin: Address, reason: String) -> bool {
        user_management::suspend_user(&env, user, admin, reason)
    }

    pub fn ban_user(env: Env, user: Address, admin: Address, reason: String) -> bool {
        user_management::ban_user(&env, user, admin, reason)
    }

    pub fn add_council_member(env: Env, new_member: Address, appointer: Address) -> bool {
        user_management::add_council_member(&env, new_member, appointer)
    }

    // DAO Governance Functions
    pub fn create_proposal(
        env: Env,
        proposer: Address,
        proposal_type: ProposalType,
        title: String,
        description: String,
        execution_data: Bytes,
    ) -> u64 {
        dao_governance::create_proposal(&env, proposer, proposal_type, title, description, execution_data)
    }

    pub fn vote_on_proposal(env: Env, proposal_id: u64, voter: Address, vote_for: bool) -> bool {
        dao_governance::vote_on_proposal(&env, proposal_id, voter, vote_for)
    }

    pub fn execute_proposal(env: Env, proposal_id: u64, executor: Address) -> bool {
        dao_governance::execute_proposal(&env, proposal_id, executor)
    }

    pub fn delegate_vote(env: Env, delegator: Address, delegate: Address) -> bool {
        dao_governance::delegate_vote(&env, delegator, delegate)
    }

    // Plan Management Functions
    pub fn create_plan(
        env: Env,
        creator: Address,
        name: String,
        description: String,
        weekly_premium: i128,
        max_coverage: i128,
    ) -> u64 {
        plan_management::create_plan(&env, creator, name, description, weekly_premium, max_coverage)
    }

    pub fn modify_plan(
        env: Env,
        modifier: Address,
        plan_id: u64,
        new_weekly_premium: Option<i128>,
        new_max_coverage: Option<i128>,
        new_description: Option<String>,
    ) -> bool {
        plan_management::modify_plan(&env, modifier, plan_id, new_weekly_premium, new_max_coverage, new_description)
    }

    pub fn archive_plan(env: Env, archiver: Address, plan_id: u64, reason: String) -> bool {
        plan_management::archive_plan(&env, archiver, plan_id, reason)
    }

    pub fn get_available_plans(env: Env) -> Vec<Plan> {
        plan_management::get_available_plans(&env)
    }

    // Subscription Management Functions
    pub fn subscribe_to_plan(env: Env, user: Address, plan_id: u64) -> bool {
        subscription_management::subscribe_to_plan(&env, user, plan_id)
    }

    pub fn pay_premium(env: Env, user: Address) -> bool {
        subscription_management::pay_premium(&env, user)
    }

    pub fn cancel_subscription(env: Env, user: Address) -> bool {
        subscription_management::cancel_subscription(&env, user)
    }

    pub fn reactivate_subscription(env: Env, user: Address) -> bool {
        subscription_management::reactivate_subscription(&env, user)
    }

    // Claims Processing Functions
    pub fn submit_claim(
        env: Env,
        user: Address,
        amount: i128,
        claim_type: ClaimType,
        description: String,
        evidence_hash: Bytes,
    ) -> u64 {
        claims_processing::submit_claim(&env, user, amount, claim_type, description, evidence_hash)
    }

    pub fn assess_claim(env: Env, claim_id: u64, assessor: Address, decision: bool, reasoning: String) -> bool {
        claims_processing::assess_claim(&env, claim_id, assessor, decision, reasoning)
    }

    pub fn vote_on_claim(env: Env, claim_id: u64, voter: Address, approve: bool) -> bool {
        claims_processing::vote_on_claim(&env, claim_id, voter, approve)
    }

    pub fn approve_claim(env: Env, claim_id: u64, approver: Address) -> bool {
        claims_processing::approve_claim(&env, claim_id, approver)
    }

    pub fn reject_claim(env: Env, claim_id: u64, rejector: Address, reason: String) -> bool {
        claims_processing::reject_claim(&env, claim_id, rejector, reason)
    }

    pub fn dispute_claim(env: Env, claim_id: u64, disputer: Address, reason: String) -> bool {
        claims_processing::dispute_claim(&env, claim_id, disputer, reason)
    }

    // Financial Management Functions
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

    // View Functions
    pub fn get_user(env: Env, user: Address) -> Option<User> {
        user_management::get_user(&env, user)
    }

    pub fn get_proposal(env: Env, proposal_id: u64) -> Option<Proposal> {
        dao_governance::get_proposal(&env, proposal_id)
    }

    pub fn get_plan(env: Env, plan_id: u64) -> Option<Plan> {
        plan_management::get_plan(&env, plan_id)
    }

    pub fn get_subscription_for_user(env: Env, user: Address) -> Option<Subscription> {
        subscription_management::get_subscription_for_user(&env, &user)
    }

    pub fn get_claim(env: Env, claim_id: u64) -> Option<Claim> {
        claims_processing::get_claim(&env, claim_id)
    }

    pub fn get_claims_by_user(env: Env, user: Address) -> Vec<Claim> {
        claims_processing::get_claims_by_user(&env, &user)
    }

    pub fn get_financial_summary(env: Env) -> (i128, i128, i128, i128) {
        financial_management::get_financial_summary(&env)
    }

    pub fn get_claim_statistics(env: Env) -> (u64, i128, i128, u64) {
        claims_processing::get_claim_statistics(&env)
    }

    // Administrative Functions
    pub fn update_subscription_status(env: Env) {
        subscription_management::update_subscription_status(&env)
    }

    pub fn is_fund_frozen(env: Env) -> bool {
        financial_management::is_fund_frozen(&env)
    }

    pub fn check_reserve_health(env: Env) -> (bool, i128, i128) {
        financial_management::check_reserve_health(&env)
    }
}
