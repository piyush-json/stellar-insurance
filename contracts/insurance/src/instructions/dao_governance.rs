use soroban_sdk::{Env, Address, String, Vec as SdkVec};
use crate::constant::{PROPOSAL_CREATED, VOTE_CAST, PROPOSAL_EXECUTED};
use crate::state::{DataKey, Proposal, ProposalType, ProposalStatus, User, DAOVote};

#[derive(Debug)]
pub enum DAOGovernanceError {
    ProposalNotFound,
    ProposalAlreadyExecuted,
    ProposalNotOpen,
    VotingPeriodEnded,
    AlreadyVoted,
    Unauthorized,
    InsufficientQuorum,
    InvalidVoteType,
}

pub fn create_proposal(
    env: &Env,
    proposer: Address,
    proposal_type: ProposalType,
    title: String,
    description: String,
    execution_data: soroban_sdk::Bytes,
    required_quorum: u32,
) -> Result<u64, DAOGovernanceError> {
    // Check if proposer is DAO member
    let user = env.storage().instance().get::<_, User>(&DataKey::User(proposer.clone()))
        .ok_or(DAOGovernanceError::Unauthorized)?;
    
    if !user.is_dao_member {
        return Err(DAOGovernanceError::Unauthorized);
    }

    let proposal_id = env.ledger().sequence() as u64;
    let voting_period = 604800; // 7 days in seconds

    let proposal = Proposal {
        id: proposal_id,
        proposer: proposer.clone(),
        proposal_type: proposal_type.clone(),
        title,
        description,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + voting_period,
        yes_votes: 0,
        no_votes: 0,
        status: ProposalStatus::Open,
        execution_data,
        required_quorum,
        voters: SdkVec::new(&env),
        voting_period_end: env.ledger().timestamp() + voting_period,
        votes_for: 0,
        votes_against: 0,
        quorum_required: required_quorum as i128,
        created_date: env.ledger().timestamp(),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal_id), &proposal);
    
    env.events().publish(
        (PROPOSAL_CREATED, proposal_id),
        (proposer, proposal_type)
    );
    
    Ok(proposal_id)
}

pub fn vote_on_proposal(
    env: &Env,
    proposal_id: u64,
    voter: Address,
    vote_direction: bool,
    vote_weight: i128,
) -> Result<bool, DAOGovernanceError> {
    let proposal_key = DataKey::DAOProposal(proposal_id);
    let mut proposal = env.storage().instance().get::<_, Proposal>(&proposal_key)
        .ok_or(DAOGovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Open {
        return Err(DAOGovernanceError::ProposalNotOpen);
    }

    if env.ledger().timestamp() > proposal.voting_period_end {
        return Err(DAOGovernanceError::VotingPeriodEnded);
    }

    // Check if user has already voted
    if proposal.voters.contains(&voter) {
        return Err(DAOGovernanceError::AlreadyVoted);
    }

    // Check if voter is DAO member
    let user = env.storage().instance().get::<_, User>(&DataKey::User(voter.clone()))
        .ok_or(DAOGovernanceError::Unauthorized)?;
    
    if !user.is_dao_member {
        return Err(DAOGovernanceError::Unauthorized);
    }

    // Record the vote
    let vote = DAOVote {
        proposal_id,
        voter: voter.clone(),
        vote_weight,
        vote_direction,
        timestamp: env.ledger().timestamp(),
    };

    // Update proposal vote counts
    if vote_direction {
        proposal.votes_for += vote_weight as u32;
        proposal.yes_votes += 1;
    } else {
        proposal.votes_against += vote_weight as u32;
        proposal.no_votes += 1;
    }

    proposal.voters.push_back(voter.clone());
    
    // Store the vote
    env.storage().instance().set(&DataKey::DAOVote(proposal_id), &vote);
    
    // Update proposal
    env.storage().instance().set(&proposal_key, &proposal);

    env.events().publish(
        (VOTE_CAST, proposal_id),
        (voter, vote_direction, vote_weight)
    );

    Ok(true)
}

pub fn finalize_proposal(env: &Env, proposal_id: u64) -> Result<bool, DAOGovernanceError> {
    let proposal_key = DataKey::DAOProposal(proposal_id);
    let mut proposal = env.storage().instance().get::<_, Proposal>(&proposal_key)
        .ok_or(DAOGovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Open {
        return Err(DAOGovernanceError::ProposalAlreadyExecuted);
    }

    if env.ledger().timestamp() <= proposal.voting_period_end {
        return Err(DAOGovernanceError::VotingPeriodEnded);
    }

    let total_votes = proposal.votes_for + proposal.votes_against;
    let quorum_met = total_votes >= proposal.quorum_required as u32;

    if !quorum_met {
        proposal.status = ProposalStatus::Failed;
        env.storage().instance().set(&proposal_key, &proposal);
        return Ok(false);
    }

    // Determine if proposal passed
    if proposal.votes_for > proposal.votes_against {
        proposal.status = ProposalStatus::Passed;
    } else {
        proposal.status = ProposalStatus::Rejected;
    }

    env.storage().instance().set(&proposal_key, &proposal);

    env.events().publish(
        (PROPOSAL_EXECUTED, proposal_id),
        (proposal.status, total_votes)
    );

    Ok(true)
}

pub fn execute_proposal(env: &Env, proposal_id: u64, executor: Address) -> Result<bool, DAOGovernanceError> {
    let proposal_key = DataKey::DAOProposal(proposal_id);
    let proposal = env.storage().instance().get::<_, Proposal>(&proposal_key)
        .ok_or(DAOGovernanceError::ProposalNotFound)?;

    if proposal.status != ProposalStatus::Passed {
        return Err(DAOGovernanceError::ProposalNotOpen);
    }

    // Check if executor is DAO member
    let user = env.storage().instance().get::<_, User>(&DataKey::User(executor.clone()))
        .ok_or(DAOGovernanceError::Unauthorized)?;
    
    if !user.is_dao_member {
        return Err(DAOGovernanceError::Unauthorized);
    }

    // Mark proposal as active (since Executed doesn't exist)
    let mut updated_proposal = proposal.clone();
    updated_proposal.status = ProposalStatus::Active;
    env.storage().instance().set(&proposal_key, &updated_proposal);

    env.events().publish(
        (PROPOSAL_EXECUTED, proposal_id),
        (executor, "executed")
    );

    Ok(true)
}

pub fn get_proposal(env: &Env, proposal_id: u64) -> Result<Proposal, DAOGovernanceError> {
    env.storage().instance().get(&DataKey::DAOProposal(proposal_id))
        .ok_or(DAOGovernanceError::ProposalNotFound)
}

pub fn get_vote(env: &Env, proposal_id: u64, voter: Address) -> Result<DAOVote, DAOGovernanceError> {
    env.storage().instance().get(&DataKey::DAOVote(proposal_id))
        .ok_or(DAOGovernanceError::Unauthorized)
}

pub fn get_proposals_by_type(env: &Env, proposal_type: ProposalType) -> Result<SdkVec<u64>, DAOGovernanceError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    Ok(SdkVec::new(&env))
}

pub fn get_active_proposals(env: &Env) -> Result<SdkVec<u64>, DAOGovernanceError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    Ok(SdkVec::new(&env))
}

// Helper function to check if a proposal can be voted on
pub fn can_vote_on_proposal(env: &Env, proposal_id: u64, voter: Address) -> bool {
    if let Ok(proposal) = get_proposal(env, proposal_id) {
        if proposal.status != ProposalStatus::Open {
            return false;
        }
        
        if env.ledger().timestamp() > proposal.voting_period_end {
            return false;
        }
        
        if proposal.voters.contains(&voter) {
            return false;
        }
        
        // Check if voter is DAO member
        if let Some(user) = env.storage().instance().get::<_, User>(&DataKey::User(voter)) {
            return user.is_dao_member;
        }
    }
    false
}
