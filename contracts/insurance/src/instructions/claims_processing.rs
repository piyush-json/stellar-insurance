use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{Env, Address, String, Vec as SdkVec, symbol_short, BytesN};
use crate::constant::{CLAIM_SUBMITTED, CLAIM_APPROVED, CLAIM_REJECTED, CLAIM_PAID};
use crate::state::{DataKey, Claim, ClaimStatus, Policy, User, ClaimType};
use crate::instructions::user_management::is_user_approved;
use crate::instructions::policy_management::is_policy_active;

#[derive(Debug)]
pub enum ClaimsProcessingError {
    ClaimNotFound,
    PolicyNotFound,
    Unauthorized,
    InvalidClaimData,
    PolicyNotActive,
    InsufficientCoverage,
    ClaimAlreadyProcessed,
    InvalidEvidence,
    ClaimAmountExceeded,
}

pub fn create_claim(
    env: &Env,
    claimer: Address,
    subscription_id: u64,
    amount: i128,
    image_hash: BytesN<32>,
    description: String,
) -> Result<u64, ClaimsProcessingError> {
    // Check if claimer is approved
    if !is_user_approved(env, &claimer) {
        return Err(ClaimsProcessingError::Unauthorized);
    }

    // Get the subscription to validate it exists and is active
    let subscription_key = DataKey::Subscription(subscription_id);
    let subscription = env.storage().instance().get::<_, crate::state::Subscription>(&subscription_key)
        .ok_or(ClaimsProcessingError::PolicyNotFound)?;

    // Check if subscription is active
    if subscription.status != crate::state::SubscriptionStatus::Active {
        return Err(ClaimsProcessingError::PolicyNotActive);
    }

    // Validate claim amount
    if amount <= 0 {
        return Err(ClaimsProcessingError::InvalidClaimData);
    }

    let claim_id = env.ledger().sequence() as u64;
    let claim = Claim {
        id: claim_id,
        subscription_id,
        claimer: claimer.clone(),
        amount,
        image_hash,
        created_at: env.ledger().timestamp(),
        plan_id: subscription.policy_id,
        assessor_notes: String::from_str(&env, ""), // Empty string initially
        payout_date: None,
        status: ClaimStatus::Submitted,
        description,
        claim_type: ClaimType::Standard, // Default to Standard type
    };

    env.storage().instance().set(&DataKey::Claim(claim_id), &claim);

    env.events().publish(
        (CLAIM_SUBMITTED, claim_id),
        (claimer, subscription_id, amount)
    );

    Ok(claim_id)
}

pub fn review_claim(
    env: &Env,
    claim_id: u64,
    reviewer: Address,
    status: ClaimStatus,
    notes: String,
) -> Result<bool, ClaimsProcessingError> {
    let claim_key = DataKey::Claim(claim_id);
    let mut claim = env.storage().instance().get::<_, Claim>(&claim_key)
        .ok_or(ClaimsProcessingError::ClaimNotFound)?;

    // Check if reviewer is authorized (DAO member or admin)
    if !is_user_approved(env, &reviewer) {
        return Err(ClaimsProcessingError::Unauthorized);
    }

    // Check if claim can be reviewed
    if claim.status != ClaimStatus::Submitted {
        return Err(ClaimsProcessingError::ClaimAlreadyProcessed);
    }

    // Update claim status
    claim.status = status.clone();
    claim.assessor_notes = notes;

    env.storage().instance().set(&claim_key, &claim);

    let event_type = match status {
        ClaimStatus::Approved => CLAIM_APPROVED,
        ClaimStatus::Rejected => CLAIM_REJECTED,
        _ => CLAIM_SUBMITTED,
    };

    env.events().publish(
        (event_type, claim_id),
        (reviewer, status, claim.amount)
    );

    Ok(true)
}

pub fn process_claim_payout(
    env: &Env,
    claim_id: u64,
    processor: Address,
) -> Result<bool, ClaimsProcessingError> {
    let claim_key = DataKey::Claim(claim_id);
    let mut claim = env.storage().instance().get::<_, Claim>(&claim_key)
        .ok_or(ClaimsProcessingError::ClaimNotFound)?;

    // Check if claim is approved
    if claim.status != ClaimStatus::Approved {
        return Err(ClaimsProcessingError::ClaimAlreadyProcessed);
    }

    // Check if processor is authorized
    if !is_user_approved(env, &processor) {
        return Err(ClaimsProcessingError::Unauthorized);
    }

    // Get the policy to validate coverage
    let policy = env.storage().instance().get::<_, Policy>(&DataKey::Policy(claim.plan_id))
        .ok_or(ClaimsProcessingError::PolicyNotFound)?;

    // Check if policy is active
    if !is_policy_active(env, &policy) {
        return Err(ClaimsProcessingError::PolicyNotActive);
    }

    // Check if policy has sufficient coverage
    if policy.params.max_claim_amount < claim.amount {
        return Err(ClaimsProcessingError::InsufficientCoverage);
    }

    // Process payout
    claim.status = ClaimStatus::Paid;
    claim.payout_date = Some(env.ledger().timestamp());

    // Update claim
    env.storage().instance().set(&claim_key, &claim);

    env.events().publish(
        (CLAIM_PAID, claim_id),
        (processor, claim.amount, "payout processed")
    );

    Ok(true)
}

pub fn update_claim(
    env: &Env,
    claim_id: u64,
    updater: Address,
    description: Option<String>,
    new_image_hash: Option<String>,
) -> Result<bool, ClaimsProcessingError> {
    let claim_key = DataKey::Claim(claim_id);
    let mut claim = env.storage().instance().get::<_, Claim>(&claim_key)
        .ok_or(ClaimsProcessingError::ClaimNotFound)?;

    // Check if updater is the claimer
    if claim.claimer != updater {
        return Err(ClaimsProcessingError::Unauthorized);
    }

    // Check if claim can be updated
    if claim.status != ClaimStatus::Submitted {
        return Err(ClaimsProcessingError::ClaimAlreadyProcessed);
    }

    // Update fields if provided
    if let Some(desc) = description {
        // Note: The current Claim struct doesn't have a description field
        // This would need to be added to the state if needed
    }

    if let Some(image_hash) = new_image_hash {
        claim.image_hash = image_hash.as_object().to_xdr(&env).try_into().unwrap();
    }

    env.storage().instance().set(&claim_key, &claim);

    env.events().publish(
        (CLAIM_SUBMITTED, claim_id),
        (updater, "claim updated")
    );

    Ok(true)
}

pub fn get_claim(env: &Env, claim_id: u64) -> Result<Claim, ClaimsProcessingError> {
    env.storage().instance().get(&DataKey::Claim(claim_id))
        .ok_or(ClaimsProcessingError::ClaimNotFound)
}

pub fn get_user_claims(env: &Env, user: Address) -> Result<SdkVec<u64>, ClaimsProcessingError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    Ok(SdkVec::new(&env))
}

pub fn get_subscription_claims(env: &Env, subscription_id: u64) -> Result<SdkVec<u64>, ClaimsProcessingError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    Ok(SdkVec::new(&env))
}

pub fn get_pending_claims(env: &Env) -> Result<SdkVec<u64>, ClaimsProcessingError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    Ok(SdkVec::new(&env))
}

// Helper function to calculate risk score for a claim
pub fn calculate_claim_risk_score(
    claim_amount: i128,
    policy_max_claim: i128,
    claimant_history: u32,
) -> u32 {
    let mut risk_score = 0;

    // Amount risk (higher amount = higher risk)
    let amount_ratio = (claim_amount * 100) / policy_max_claim;
    if amount_ratio > 80 {
        risk_score += 30;
    } else if amount_ratio > 50 {
        risk_score += 20;
    } else if amount_ratio > 20 {
        risk_score += 10;
    }

    // Claimant history risk
    if claimant_history > 5 {
        risk_score += 25;
    } else if claimant_history > 2 {
        risk_score += 15;
    }

    risk_score.min(100) // Cap at 100
}

// Helper function to check if a claim can be appealed
pub fn can_appeal_claim(env: &Env, claim: &Claim) -> bool {
    claim.status == ClaimStatus::Rejected
}

// Helper function to check if a claim is overdue for review
pub fn is_claim_overdue(env: &Env, claim: &Claim) -> bool {
    claim.status == ClaimStatus::UnderReview && 
    env.ledger().timestamp() > claim.created_at + (30 * 24 * 60 * 60) // 30 days overdue
}
