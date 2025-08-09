use soroban_sdk::{Env, Address, Map, Vec, String, Bytes};
use crate::state::{Proposal, ProposalType, ProposalStatus, DAOVote, PlatformConfig,DataKey};
use crate::constant::{PROPOSALS, DAO_VOTES, VOTE_DELEGATIONS, PLATFORM_CONFIG, PROPOSAL_CREATED, VOTE_CAST, PROPOSAL_EXECUTED, VOTE_DELEGATED};
use crate::instructions::user_management::{is_council_member, is_user_approved};

pub fn create_proposal(
    env: &Env,
    proposer: Address,
    proposal_type: ProposalType,
    title: String,
    description: String,
    execution_data: Bytes,
) -> u64 {
    if !can_create_proposal(env, &proposer, &proposal_type) {
        panic!("Insufficient permissions to create proposal");
    }
    let payload  = (proposer, env.ledger().timestamp(), proposal_type);
    let proposal_id = env.crypto().sha256(&payload).to_u64();
    if let Some(_) = env.storage().instance().get::<_, Proposal>(&DataKey::Proposal(proposal_id)) {
        panic!("Proposal with this ID already exists");
    }
    
    let config = get_platform_config(env);
    let voting_period = config.proposal_duration_days * 24 * 60 * 60; // Convert days to seconds
    
    let proposal = Proposal {
        id: proposal_id,
        proposer: proposer.clone(),
        proposal_type: proposal_type.clone(),
        title: title.clone(),
        description,
        voting_period_end: env.ledger().timestamp() + voting_period,
        votes_for: 0,
        votes_against: 0,
        status: ProposalStatus::Active,
        execution_data,
        quorum_required: calculate_quorum_required(env, &proposal_type),
        created_date: env.ledger().timestamp(),
    };

    env.storage().instance().set(&DataKey::Proposal(&proposal_id), &proposal);
    env.events().publish((PROPOSAL_CREATED, &proposal_id), proposer);
    proposal_id
}

pub fn vote_on_proposal(env: &Env, proposal_id: u64, voter: Address, vote_for: bool) -> bool {
    // Check if voter has voting rights
    let vote_weight = calculate_voting_weight(env, &voter);
    if vote_weight == 0 {
        return false;
    }

    // Check if proposal exists and is active
    let mut proposal = match env.storage().instance().get::<_, Proposal>(&DataKey::Proposal(proposal_id)) {
        Some(p) => p,
        None => return false, 
    };

    if proposal.status != ProposalStatus::Active || env.ledger().timestamp() > proposal.voting_period_end {
        return false;
    }

    // Check if user already voted
    let mut votes: Map<u64, Vec<DAOVote>> = env.storage().instance().get(&DAO_VOTES).unwrap_or_else(|| Map::new(env));
    let proposal_votes = votes.get(proposal_id).unwrap_or(Vec::new(env));
    
    for vote in proposal_votes.iter() {
        if vote.voter == voter {
            return false; // Already voted
        }
    }

    // Record the vote
    let new_vote = DAOVote {
        proposal_id,
        voter: voter.clone(),
        vote_weight,
        vote_direction: vote_for,
        timestamp: env.ledger().timestamp(),
    };

    let mut updated_votes = proposal_votes;
    updated_votes.push_back(new_vote);
    votes.set(proposal_id, updated_votes);
    env.storage().instance().set(&DAO_VOTES, &votes);

    // Update proposal vote counts
    if vote_for {
        proposal.votes_for += vote_weight;
    } else {
        proposal.votes_against += vote_weight;
    }

    env.storage().instance().set(&DataKey::Proposal(proposal_id), &proposal);
    env.events().publish((VOTE_CAST, proposal_id), voter);
    
    // Check if proposal should be finalized
    try_finalize_proposal(env, proposal_id);
    
    true
}

pub fn execute_proposal(env: &Env, proposal_id: u64, executor: Address) -> bool {
    let mut proposals: Map<u64, Proposal> = env.storage().instance().get(&PROPOSALS).unwrap_or_else(|| Map::new(env));
    let mut proposal = match proposals.get(proposal_id) {
        Some(p) => p,
        None => return false,
    };

    // Check if proposal is ready for execution
    if proposal.status != ProposalStatus::Passed {
        return false;
    }

    // Check if executor has permission
    if !is_council_member(env, &executor) {
        return false;
    }

    // Execute based on proposal type
    let success = match proposal.proposal_type {
        ProposalType::UserApproval => execute_user_approval(env, &proposal),
        ProposalType::PlanManagement => execute_plan_management(env, &proposal),
        ProposalType::Financial => execute_financial_proposal(env, &proposal),
        ProposalType::Governance => execute_governance_proposal(env, &proposal),
    };

    if success {
        proposal.status = ProposalStatus::Executed;
        proposals.set(proposal_id, proposal);
        env.storage().instance().set(&PROPOSALS, &proposals);
        env.events().publish((PROPOSAL_EXECUTED, proposal_id), executor);
        true
    } else {
        false
    }
}

pub fn delegate_vote(env: &Env, delegator: Address, delegate: Address) -> bool {
    // Check if both users are approved
    if !is_user_approved(env, &delegator) || !is_user_approved(env, &delegate) {
        return false;
    }

    let mut delegations: Map<Address, Address> = env.storage().persistent().get(&VOTE_DELEGATIONS).unwrap_or_else(|| Map::new(env));
    delegations.set(delegator.clone(), delegate.clone());
    env.storage().persistent().set(&VOTE_DELEGATIONS, &delegations);
    env.events().publish((VOTE_DELEGATED, delegator), delegate);
    true
}

fn can_create_proposal(env: &Env, proposer: &Address, proposal_type: &ProposalType) -> bool {
    match proposal_type {
        ProposalType::UserApproval => true, // Anyone can request approval
        _ => is_council_member(env, proposer), // Only council members for other types
    }
}

fn calculate_voting_weight(env: &Env, voter: &Address) -> i128 {
    let mut weight = 0i128;

    // Base weight for approved users
    if is_user_approved(env, voter) {
        weight += 1;
    }

    // Additional weight for council members
    if is_council_member(env, voter) {
        weight += 1;
    }

    // Check for delegated votes
    let delegations: Map<Address, Address> = env.storage().persistent().get(&VOTE_DELEGATIONS).unwrap_or_else(|| Map::new(env));
    for (delegator, delegate) in delegations.iter() {
        if delegate == voter.clone() && is_user_approved(env, &delegator) {
            weight += 1;
        }
    }

    // Bonus weight based on reputation and contributions
    if let Some(user) = crate::instructions::user_management::get_user(env, voter.clone()) {
        // Reputation bonus (0-50 basis points)
        weight += (user.reputation_score as i128) / 2;
        
        // Contribution bonus
        if user.village_contributions > 1000 {
            weight += 1;
        }
    }

    weight
}

fn calculate_quorum_required(env: &Env, proposal_type: &ProposalType) -> i128 {
    let config = get_platform_config(env);
    let base_quorum = config.minimum_quorum;

    match proposal_type {
        ProposalType::UserApproval => base_quorum / 2, // Lower threshold for user approvals
        ProposalType::Financial => base_quorum * 2,    // Higher threshold for financial decisions
        ProposalType::Governance => base_quorum * 3,   // Highest threshold for governance changes
        _ => base_quorum,
    }
}

fn try_finalize_proposal(env: &Env, proposal_id: u64) {
    let mut proposals: Map<u64, Proposal> = env.storage().persistent().get(&PROPOSALS).unwrap_or_else(|| Map::new(env));
    let mut proposal = match proposals.get(proposal_id) {
        Some(p) => p,
        None => return,
    };

    if proposal.status != ProposalStatus::Active {
        return;
    }

    let total_votes = proposal.votes_for + proposal.votes_against;
    let voting_ended = env.ledger().timestamp() > proposal.voting_period_end;
    let quorum_met = total_votes >= proposal.quorum_required;

    if voting_ended || quorum_met {
        if proposal.votes_for > proposal.votes_against && quorum_met {
            proposal.status = ProposalStatus::Passed;
        } else {
            proposal.status = ProposalStatus::Rejected;
        }

        proposals.set(proposal_id, proposal);
        env.storage().persistent().set(&PROPOSALS, &proposals);
    }
}

fn execute_user_approval(env: &Env, proposal: &Proposal) -> bool {
    // Parse user address from execution_data
    // let user_address_str = String::from_slice(env, &proposal.execution_data);
    // In a real implementation, you'd need proper address parsing
    // For now, we'll assume the execution_data contains the user address
    true
}

fn execute_plan_management(_env: &Env, _proposal: &Proposal) -> bool {
    // Implementation for plan management proposals
    true
}

fn execute_financial_proposal(_env: &Env, _proposal: &Proposal) -> bool {
    // Implementation for financial proposals
    true
}

fn execute_governance_proposal(_env: &Env, _proposal: &Proposal) -> bool {
    // Implementation for governance proposals
    true
}

fn get_platform_config(env: &Env) -> PlatformConfig {
    env.storage().persistent().get(&PLATFORM_CONFIG).unwrap_or(PlatformConfig {
        grace_period_weeks: 2,
        minimum_quorum: 3,
        proposal_duration_days: 7,
        max_claim_amount_ratio: 80, // 80% of max coverage
        penalty_rate: 500, // 5% in basis points
        council_size: 5,
    })
}

pub fn get_proposal(env: &Env, proposal_id: u64) -> Option<Proposal> {
    let proposals: Map<u64, Proposal> = env.storage().persistent().get(&PROPOSALS).unwrap_or_else(|| Map::new(env));
    proposals.get(proposal_id)
}

pub fn get_proposal_votes(env: &Env, proposal_id: u64) -> Vec<DAOVote> {
    let votes: Map<u64, Vec<DAOVote>> = env.storage().persistent().get(&DAO_VOTES).unwrap_or_else(|| Map::new(env));
    votes.get(proposal_id).unwrap_or(Vec::new(env))
}
