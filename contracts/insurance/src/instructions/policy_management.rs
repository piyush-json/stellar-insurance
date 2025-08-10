use soroban_sdk::{Env, Address, String, Vec};
use crate::constant::{EVENT_POL_CRT, EVENT_POL_ARC, EVENT_POL_DEL};
use crate::state::{DataKey, Policy, PolicyStatus, PolicyParams};
use crate::instructions::user_management::is_user_approved;

#[derive(Debug)]
pub enum PolicyManagementError {
    PolicyNotFound,
    PolicyAlreadyExists,
    Unauthorized,
    InvalidPolicyData,
    InsufficientCoverage,
    PolicyNotActive,
    CoverageExceeded,
    InvalidPolicyParams,
}

pub fn create_policy(
    env: &Env,
    creator: Address,
    title: String,
    description: String,
    params: PolicyParams,
) -> Result<u64, PolicyManagementError> {
    
    if !is_user_approved(env, &creator) {
        return Err(PolicyManagementError::Unauthorized);
    }

    
    if params.max_claim_amount <= 0 {
        return Err(PolicyManagementError::InvalidPolicyData);
    }

    let policy_id = env.ledger().sequence() as u64;
    let policy = Policy {
        id: policy_id,
        title: title.clone(),
        description,
        params,
        status: PolicyStatus::Active,
        created_at: env.ledger().timestamp(),
        creator: creator.clone(),
    };

    env.storage().instance().set(&DataKey::Policy(policy_id), &policy);
    
    env.events().publish(
        (EVENT_POL_CRT, policy_id),
        (creator, title)
    );

    Ok(policy_id)
}

pub fn update_policy(
    env: &Env,
    policy_id: u64,
    updater: Address,
    new_title: Option<String>,
    new_description: Option<String>,
    new_params: Option<PolicyParams>,
) -> Result<bool, PolicyManagementError> {
    let policy_key = DataKey::Policy(policy_id);
    let mut policy = env.storage().instance().get::<_, Policy>(&policy_key)
        .ok_or(PolicyManagementError::PolicyNotFound)?;

    
    if policy.creator != updater && !is_user_approved(env, &updater) {
        return Err(PolicyManagementError::Unauthorized);
    }

    
    if policy.status != PolicyStatus::Active {
        return Err(PolicyManagementError::PolicyNotActive);
    }

    
    if let Some(title) = new_title {
        policy.title = title;
    }

    if let Some(description) = new_description {
        policy.description = description;
    }

    if let Some(params) = new_params {
        if params.max_claim_amount > 0 {
            policy.params = params;
        }
    }

    env.storage().instance().set(&policy_key, &policy);

    env.events().publish(
        (EVENT_POL_ARC, policy_id),
        (updater, "policy updated")
    );

    Ok(true)
}

pub fn archive_policy(
    env: &Env,
    policy_id: u64,
    archiver: Address,
) -> Result<bool, PolicyManagementError> {
    let policy_key = DataKey::Policy(policy_id);
    let mut policy = env.storage().instance().get::<_, Policy>(&policy_key)
        .ok_or(PolicyManagementError::PolicyNotFound)?;

    
    if policy.creator != archiver && !is_user_approved(env, &archiver) {
        return Err(PolicyManagementError::Unauthorized);
    }

    
    if policy.status != PolicyStatus::Active {
        return Err(PolicyManagementError::PolicyNotActive);
    }

    policy.status = PolicyStatus::Archived;
    env.storage().instance().set(&policy_key, &policy);

    env.events().publish(
        (EVENT_POL_ARC, policy_id),
        (archiver, "policy archived")
    );

    Ok(true)
}

pub fn delete_policy(
    env: &Env,
    policy_id: u64,
    deleter: Address,
) -> Result<bool, PolicyManagementError> {
    let policy_key = DataKey::Policy(policy_id);
    let policy = env.storage().instance().get::<_, Policy>(&policy_key)
        .ok_or(PolicyManagementError::PolicyNotFound)?;

    
    if policy.creator != deleter && !is_user_approved(env, &deleter) {
        return Err(PolicyManagementError::Unauthorized);
    }

    
    if policy.status != PolicyStatus::Archived {
        return Err(PolicyManagementError::PolicyNotActive);
    }

    
    env.storage().instance().remove(&policy_key);

    env.events().publish(
        (EVENT_POL_DEL, policy_id),
        (deleter, "policy deleted")
    );

    Ok(true)
}

pub fn get_policy(env: &Env, policy_id: u64) -> Result<Policy, PolicyManagementError> {
    env.storage().instance().get(&DataKey::Policy(policy_id))
        .ok_or(PolicyManagementError::PolicyNotFound)
}

pub fn get_user_policies(env: &Env, user: Address) -> Result<Vec<u64>, PolicyManagementError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    //TODO: Implement this
    Ok(Vec::new(&env))
}

pub fn get_active_policies(env: &Env) -> Result<Vec<u64>, PolicyManagementError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    //TODO: Implement this
    Ok(Vec::new(&env))
}

pub fn calculate_premium(
    env: &Env,
    coverage_amount: i128,
    risk_score: u32,
    duration_days: u32,
) -> i128 {
    let base_rate = 50;     let risk_multiplier = 100 + (risk_score as i128 * 2); 
    let duration_multiplier = (duration_days as i128 * 100) / 365; 

    (coverage_amount * base_rate * risk_multiplier * duration_multiplier) / (100 * 100 * 100)
}

pub fn validate_policy_eligibility(
    env: &Env,
    user: Address,
    policy_id: u64,
) -> Result<bool, PolicyManagementError> {
    
    if !is_user_approved(env, &user) {
        return Err(PolicyManagementError::Unauthorized);
    }

    
    let policy = env.storage().instance().get::<_, Policy>(&DataKey::Policy(policy_id))
        .ok_or(PolicyManagementError::PolicyNotFound)?;

    if policy.status != PolicyStatus::Active {
        return Err(PolicyManagementError::PolicyNotActive);
    }

    // Check if user already has an active subscription for this policy
    // This would require maintaining an index in a real implementation
    
    Ok(true)
}


pub fn is_policy_active(env: &Env, policy: &Policy) -> bool {
    policy.status == PolicyStatus::Active
}


pub fn can_use_policy_for_subscription(env: &Env, policy: &Policy) -> bool {
    policy.status == PolicyStatus::Active
}
