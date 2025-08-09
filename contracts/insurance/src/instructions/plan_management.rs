use soroban_sdk::{symbol_short, Address, Env, Map, String, Vec};
use crate::state::{DataKey, Policy, PolicyParams, PolicyStatus, Proposal, ProposalStatus, ProposalType};
use crate::instructions::user_management::check_dao_member;
use crate::constant::*;

enum PlanManagementError {
    Unauthorized,
    PolicyNotFound
}

pub fn create_plan_proposal(
    env: &Env,
    creator: Address,
    name: String,
    description: String,
    params:PolicyParams
) ->  Result<bool, PlanManagementError>  {
    // Only council members can create plans
    if !check_dao_member(env, &creator)? {
        return Err(PlanManagementError::Unauthorized);
    }

    let proposal = Proposal {
        id: env.ledger().sequence(),
        proposer: creator.clone(),
        proposal_type: ProposalType::PolicyCreation,
        title: String::from_str(env, "User Ban"),
        description: description,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + 604800, // 7 days
        yes_votes: 0,     // Each vote counts as 1
        no_votes: 0,      // Each vote counts as 1
        status: ProposalStatus::Open,
        //TODO correct thi
        execution_data: user.clone().to_object().into(),
        required_quorum: CONFIG_MIN_QUORUM, // Standard quorum for bans
        voters: Vec::new(&env),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal.id as u64), &proposal);
    
    env.events().publish(
        (symbol_short!("polc"), &proposal.id),
        (creator, proposal.id)
    );
    Ok(true)
}

pub fn archive_plan_proposal(env: &Env, archiver: Address, policy_id: u64, reason: String) -> bool {
    if !check_dao_member(env, &archiver) {
        return false;
    }

    let mut policy = env.storage().instance().get::<_,Policy>(DataKey::Policy(policy_id)).ok_or(PlanManagementError::PolicyNotFound)?;

        if plan.status != PlanStatus::Active {
            return false;
        }

        plan.status = PlanStatus::Archived;
        plan.archived_date = Some(env.ledger().timestamp());

        plans.set(plan_id, plan);
        env.storage().persistent().set(&PLANS, &plans);

        env.events().publish((PLAN_ARCHIVED, plan_id), reason);
        true
    } else {
        false
    }
}

pub fn deprecate_plan(env: &Env, deprecator: Address, plan_id: u64, reason: String) -> bool {
    // Only council members can deprecate plans
    if !is_council_member(env, &deprecator) {
        return false;
    }

    let mut plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    
    if let Some(mut plan) = plans.get(plan_id) {
        plan.status = PlanStatus::Deprecated;
        plan.archived_date = Some(env.ledger().timestamp());

        plans.set(plan_id, plan);
        env.storage().persistent().set(&PLANS, &plans);

        // Cancel all active subscriptions for this plan
        cancel_all_subscriptions_for_plan(env, plan_id);

        env.events().publish((PLAN_DEPRECATED, plan_id), reason);
        true
    } else {
        false
    }
}

pub fn get_available_plans(env: &Env) -> Vec<Plan> {
    let plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    let mut available_plans = Vec::new(env);

    for (_, plan) in plans.iter() {
        if plan.status == PlanStatus::Active {
            available_plans.push_back(plan);
        }
    }

    available_plans
}

pub fn get_plan(env: &Env, plan_id: u64) -> Option<Plan> {
    let plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    plans.get(plan_id)
}

pub fn get_all_plans(env: &Env) -> Vec<Plan> {
    let plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    let mut all_plans = Vec::new(env);

    for (_, plan) in plans.iter() {
        all_plans.push_back(plan);
    }

    all_plans
}

pub fn is_plan_available_for_subscription(env: &Env, plan_id: u64) -> bool {
    if let Some(plan) = get_plan(env, plan_id) {
        plan.status == PlanStatus::Active
    } else {
        false
    }
}

pub fn increment_plan_subscriber_count(env: &Env, plan_id: u64) -> bool {
    let mut plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    
    if let Some(mut plan) = plans.get(plan_id) {
        plan.subscriber_count += 1;
        plans.set(plan_id, plan);
        env.storage().persistent().set(&PLANS, &plans);
        true
    } else {
        false
    }
}

pub fn decrement_plan_subscriber_count(env: &Env, plan_id: u64) -> bool {
    let mut plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    
    if let Some(mut plan) = plans.get(plan_id) {
        if plan.subscriber_count > 0 {
            plan.subscriber_count -= 1;
        }
        plans.set(plan_id, plan);
        env.storage().persistent().set(&PLANS, &plans);
        true
    } else {
        false
    }
}

fn cancel_all_subscriptions_for_plan(env: &Env, plan_id: u64) {
    use crate::state::{Subscription, SubscriptionStatus};
    
    let mut subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let mut updated = false;

    for (sub_id, mut subscription) in subscriptions.iter() {
        if subscription.plan_id == plan_id && subscription.status == SubscriptionStatus::Active {
            subscription.status = SubscriptionStatus::Cancelled;
            subscriptions.set(sub_id, subscription.clone());
            
            env.events().publish(
                (SUBSCRIPTION_CANCELLED, subscription.user.clone()),
                plan_id
            );
            updated = true;
        }
    }

    if updated {
        env.storage().persistent().set(&SUBSCRIPTIONS, &subscriptions);
    }
}

pub fn get_plans_by_status(env: &Env, status: PlanStatus) -> Vec<Plan> {
    let plans: Map<u64, Plan> = env.storage().persistent().get(&PLANS).unwrap_or_else(|| Map::new(env));
    let mut filtered_plans = Vec::new(env);

    for (_, plan) in plans.iter() {
        if plan.status == status {
            filtered_plans.push_back(plan);
        }
    }

    filtered_plans
}

pub fn get_plan_statistics(env: &Env, plan_id: u64) -> Option<(u64, i128, i128)> {
    // Returns (subscriber_count, total_premiums_collected, total_claims_paid)
    if let Some(plan) = get_plan(env, plan_id) {
        let total_premiums = calculate_total_premiums_for_plan(env, plan_id);
        let total_claims = calculate_total_claims_for_plan(env, plan_id);
        Some((plan.subscriber_count, total_premiums, total_claims))
    } else {
        None
    }
}

fn calculate_total_premiums_for_plan(env: &Env, plan_id: u64) -> i128 {
    use crate::state::Payment;
    
    let payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PAYMENTS).unwrap_or_else(|| Map::new(env));
    let mut total = 0i128;

    for (_, payment_list) in payments.iter() {
        for payment in payment_list.iter() {
            if payment.plan_id == plan_id {
                total += payment.amount;
            }
        }
    }

    total
}

fn calculate_total_claims_for_plan(env: &Env, plan_id: u64) -> i128 {
    use crate::state::{Claim, ClaimStatus};
    
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let mut total = 0i128;

    for (_, claim) in claims.iter() {
        if claim.plan_id == plan_id && claim.status == ClaimStatus::Paid {
            total += claim.amount;
        }
    }

    total
}
