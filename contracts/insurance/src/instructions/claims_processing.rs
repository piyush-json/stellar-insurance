use soroban_sdk::{Env, Address, Map, Vec, String, Bytes};
use crate::constant::{CLAIMS, CLAIM_ASSESSMENTS, CLAIM_VOTES, SAFETY_POOL, CLAIM_SUBMITTED, CLAIM_ASSESSED, CLAIM_VOTED, CLAIM_APPROVED, CLAIM_REJECTED, CLAIM_DISPUTED, CLAIM_PAID};
use crate::state::{Claim, ClaimStatus, ClaimType, ClaimVote, Assessment, SafetyPool};
use crate::instructions::user_management::{is_user_approved, get_user, update_user_reputation};
use crate::instructions::subscription_management::{get_active_subscription_for_user};
use crate::instructions::plan_management::get_plan;

pub fn submit_claim(
    env: &Env,
    user: Address,
    amount: i128,
    claim_type: ClaimType,
    description: String,
    evidence_hash: Bytes,
) -> u64 {
    // Check if user has active subscription
    let subscription = match get_active_subscription_for_user(env, &user) {
        Some(sub) => sub,
        None => panic!("User must have active subscription to submit claims"),
    };

    // Check if plan exists and get coverage limits
    let plan = match get_plan(env, subscription.plan_id) {
        Some(p) => p,
        None => panic!("Invalid plan"),
    };

    // Validate claim amount
    if amount <= 0 || amount > plan.max_coverage {
        panic!("Invalid claim amount");
    }

    // Check user's claim eligibility (not suspended, good standing, etc.)
    if !is_user_approved(env, &user) {
        panic!("User not approved for claims");
    }

    let mut claims: Map<u64, Claim> = env.storage().instance().get(&CLAIMS).unwrap_or_else(
        || Map::new(env)
    );
    let claim_id = claims.len() as u64 + 1;

    let new_claim = Claim {
        id: claim_id,
        user: user.clone(),
        plan_id: subscription.plan_id,
        amount,
        claim_type: claim_type.clone(),
        description: description.clone(),
        evidence_hash,
        submission_date: env.ledger().timestamp(),
        status: ClaimStatus::Submitted,
        assessor_notes: String::from_str(env, ""),
        payout_date: None,
    };

    claims.set(claim_id, new_claim);
    env.storage().instance().set(&CLAIMS, &claims);

    // Auto-advance to review if it's an emergency claim
    if claim_type == ClaimType::Emergency {
        advance_claim_to_review(env, claim_id);
    }

    env.events().publish((CLAIM_SUBMITTED, claim_id), user);
    claim_id
}

pub fn assess_claim(env: &Env, claim_id: u64, assessor: Address, decision: bool, reasoning: String) -> bool {
    // Check if assessor is approved user
    if !is_user_approved(env, &assessor) {
        return false;
    }

    let mut assessments: Map<u64, Vec<Assessment>> = env.storage().instance().get(&CLAIM_ASSESSMENTS).unwrap_or_else(|| Map::new(env));
    let mut claim_assessments = assessments.get(claim_id).unwrap_or(Vec::new(env));

    // Check if assessor already assessed this claim
    for assessment in claim_assessments.iter() {
        if assessment.assessor == assessor {
            return false; // Already assessed
        }
    }

    // Calculate assessor's voting weight
    let vote_weight = calculate_assessor_weight(env, &assessor);

    let assessment = Assessment {
        assessor: assessor.clone(),
        claim_id,
        decision,
        reasoning: reasoning.clone(),
        assessment_date: env.ledger().timestamp(),
        weight: vote_weight,
    };

    claim_assessments.push_back(assessment);
    assessments.set(claim_id, claim_assessments);
    env.storage().persistent().set(&CLAIM_ASSESSMENTS, &assessments);

    env.events().publish((CLAIM_ASSESSED, claim_id), assessor);

    // Check if we should finalize the claim
    try_finalize_claim(env, claim_id);

    true
}

pub fn vote_on_claim(env: &Env, claim_id: u64, voter: Address, approve: bool) -> bool {
    // Check if voter is approved
    if !is_user_approved(env, &voter) {
        return false;
    }

    // Check if claim exists and is in the right status
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(
        || Map::new(env)
    );
    let claim = match claims.get(claim_id) {
        Some(c) => c,
        None => return false,
    };

    if claim.status != ClaimStatus::UnderReview {
        return false;
    }

    let mut votes: Map<u64, Vec<ClaimVote>> = env.storage().persistent().get(&CLAIM_VOTES).unwrap_or_else(|| Map::new(env));
    let mut claim_votes = votes.get(claim_id).unwrap_or(Vec::new(env));

    // Check if user already voted
    for vote in claim_votes.iter() {
        if vote.voter == voter {
            return false; // Already voted
        }
    }

    let vote_weight = calculate_assessor_weight(env, &voter);

    let vote = ClaimVote {
        voter: voter.clone(),
        approve,
        weight: vote_weight,
        timestamp: env.ledger().timestamp(),
    };

    claim_votes.push_back(vote);
    votes.set(claim_id, claim_votes);
    env.storage().persistent().set(&CLAIM_VOTES, &votes);

    env.events().publish((CLAIM_VOTED, claim_id), voter);

    // Check if voting is complete
    try_finalize_claim_vote(env, claim_id);

    true
}

pub fn approve_claim(env: &Env, claim_id: u64, approver: Address) -> bool {
    use crate::instructions::user_management::is_council_member;
    // Only council members can directly approve claims
    if !is_council_member(env, &approver) {
        return false;
    }
    let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let claim = match claims.get(claim_id) {
        Some(c) => c,
        None => return false,
    };
    let mut claim_mut = claim.clone();
    // Can only approve submitted or under review claims
    if claim_mut.status != ClaimStatus::Submitted && claim_mut.status != ClaimStatus::UnderReview {
        return false;
    }
    // Check safety pool has sufficient funds
    let safety_pool: SafetyPool = env.storage().persistent().get(&SAFETY_POOL).unwrap_or_default();
    if safety_pool.total_balance < claim_mut.amount {
        return false; // Insufficient funds
    }
    claim_mut.status = ClaimStatus::Approved;
    claims.set(claim_id, claim_mut.clone());
    env.storage().persistent().set(&CLAIMS, &claims);
    // Process payout
    process_claim_payout(env, claim_id);
    env.events().publish((CLAIM_APPROVED, claim_id), approver);
    true
}

pub fn reject_claim(env: &Env, claim_id: u64, rejector: Address, reason: String) -> bool {
    use crate::instructions::user_management::is_council_member;
    
    // Only council members can directly reject claims
    if !is_council_member(env, &rejector) {
        return false;
    }

    let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let claim = match claims.get(claim_id) {
        Some(c) => c,
        None => return false,
    };
    let mut claim_mut = claim.clone();

    claim_mut.status = ClaimStatus::Rejected;
    claim_mut.assessor_notes = reason.clone();
    claims.set(claim_id, claim_mut.clone());
    env.storage().persistent().set(&CLAIMS, &claims);

    // Update user reputation negatively for rejected claims
    update_user_reputation(env, claim_mut.user.clone(), -5);

    env.events().publish((CLAIM_REJECTED, claim_id), reason);
    true
}

pub fn dispute_claim(env: &Env, claim_id: u64, disputer: Address, reason: String) -> bool {
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let claim = match claims.get(claim_id) {
        Some(c) => c,
        None => return false,
    };

    // Only the claim owner can dispute
    if claim.user != disputer {
        return false;
    }

    // Can only dispute rejected claims
    if claim.status != ClaimStatus::Rejected {
        return false;
    }

    let mut claims = claims;
    let mut claim = claim;
    claim.status = ClaimStatus::Disputed;
    claims.set(claim_id, claim);
    env.storage().persistent().set(&CLAIMS, &claims);

    env.events().publish((CLAIM_DISPUTED, claim_id), reason);
    true
}

pub fn get_claim(env: &Env, claim_id: u64) -> Option<Claim> {
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    claims.get(claim_id)
}

pub fn get_claims_by_user(env: &Env, user: &Address) -> Vec<Claim> {
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let mut user_claims = Vec::new(env);

    for (_, claim) in claims.iter() {
        if claim.user == user.clone() {
            user_claims.push_back(claim);
        }
    }

    user_claims
}

pub fn get_claims_by_status(env: &Env, status: ClaimStatus) -> Vec<Claim> {
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let mut filtered_claims = Vec::new(env);

    for (_, claim) in claims.iter() {
        if claim.status == status {
            filtered_claims.push_back(claim);
        }
    }

    filtered_claims
}

fn advance_claim_to_review(env: &Env, claim_id: u64) {
    let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    if let Some(mut claim) = claims.get(claim_id) {
        claim.status = ClaimStatus::UnderReview;
        claims.set(claim_id, claim);
        env.storage().persistent().set(&CLAIMS, &claims);
    }
}

fn calculate_assessor_weight(env: &Env, assessor: &Address) -> i128 {
    let mut weight = 1i128; // Base weight

    if let Some(user) = get_user(env, assessor.clone()) {
        // Reputation-based weight (0-50 additional points)
        weight += (user.reputation_score as i128) / 2;
        
        // Experience-based weight
        let user_claims = get_claims_by_user(env, assessor);
        weight += (user_claims.len() as i128) / 5; // Bonus for assessment experience
    }

    // Council member bonus
    use crate::instructions::user_management::is_council_member;
    if is_council_member(env, assessor) {
        weight += 2;
    }

    weight
}

fn try_finalize_claim(env: &Env, claim_id: u64) {
    let assessments: Map<u64, Vec<Assessment>> = env.storage().persistent().get(&CLAIM_ASSESSMENTS).unwrap_or_else(|| Map::new(env));
    let claim_assessments = assessments.get(claim_id).unwrap_or(Vec::new(env));

    if claim_assessments.len() >= 3 { // Minimum 3 assessments
        let mut total_weight_for = 0i128;
        let mut total_weight_against = 0i128;

        for assessment in claim_assessments.iter() {
            if assessment.decision {
                total_weight_for += assessment.weight;
            } else {
                total_weight_against += assessment.weight;
            }
        }

        let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
        if let Some(mut claim) = claims.get(claim_id) {
            if total_weight_for > total_weight_against {
                claim.status = ClaimStatus::Approved;
                process_claim_payout(env, claim_id);
            } else {
                claim.status = ClaimStatus::Rejected;
                update_user_reputation(env, claim.user.clone(), -3);
            }

            claims.set(claim_id, claim);
            env.storage().persistent().set(&CLAIMS, &claims);
        }
    }
}

fn try_finalize_claim_vote(env: &Env, claim_id: u64) {
    let votes: Map<u64, Vec<ClaimVote>> = env.storage().persistent().get(&CLAIM_VOTES).unwrap_or_else(|| Map::new(env)  );
    let claim_votes = votes.get(claim_id).unwrap_or(Vec::new(env));

    if claim_votes.len() >= 5 { // Minimum 5 votes
        let mut total_weight_for = 0i128;
        let mut total_weight_against = 0i128;

        for vote in claim_votes.iter() {
            if vote.approve {
                total_weight_for += vote.weight;
            } else {
                total_weight_against += vote.weight;
            }
        }

        let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
        if let Some(mut claim) = claims.get(claim_id) {
            if total_weight_for > total_weight_against {
                claim.status = ClaimStatus::Approved;
                process_claim_payout(env, claim_id);
            } else {
                claim.status = ClaimStatus::Rejected;
                update_user_reputation(env, claim.user.clone(), -3);
            }

            claims.set(claim_id, claim);
            env.storage().persistent().set(&CLAIMS, &claims);
        }
    }
}

fn process_claim_payout(env: &Env, claim_id: u64) {
    let mut claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let mut safety_pool: SafetyPool = env.storage().persistent().get(&SAFETY_POOL).unwrap_or_default();

    if let Some(mut claim) = claims.get(claim_id) {
        // Deduct from safety pool
        safety_pool.total_balance -= claim.amount;
        safety_pool.claim_payouts += claim.amount;

        // Update claim status
        claim.status = ClaimStatus::Paid;
        claim.payout_date = Some(env.ledger().timestamp());

        // Save changes
        claims.set(claim_id, claim.clone());
        env.storage().persistent().set(&CLAIMS, &claims);
        env.storage().persistent().set(&SAFETY_POOL, &safety_pool);

        // Update user reputation positively for successful claims
        update_user_reputation(env, claim.user.clone(), 2);

        env.events().publish((CLAIM_PAID, claim_id), claim.amount);
    }
}

pub fn get_claim_statistics(env: &Env) -> (u64, i128, i128, u64) {
    // Returns (total_claims, total_paid_amount, total_pending_amount, approval_rate_percent)
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    
    let mut total_claims = 0u64;
    let mut total_paid = 0i128;
    let mut total_pending = 0i128;
    let mut approved_claims = 0u64;

    for (_, claim) in claims.iter() {
        total_claims += 1;
        
        match claim.status {
            ClaimStatus::Paid => {
                total_paid += claim.amount;
                approved_claims += 1;
            },
            ClaimStatus::Approved => approved_claims += 1,
            ClaimStatus::Submitted | ClaimStatus::UnderReview => {
                total_pending += claim.amount;
            },
            _ => {},
        }
    }

    let approval_rate = if total_claims > 0 {
        (approved_claims * 100) / total_claims
    } else {
        0
    };

    (total_claims, total_paid, total_pending, approval_rate)
}
