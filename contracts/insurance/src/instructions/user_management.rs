use soroban_sdk::{Env, Address, String, Vec as SdkVec, symbol_short};
use crate::constant::CONFIG_MIN_QUORUM;
use crate::state::{DataKey, User, UserStatus, ProposalType, Proposal, ProposalStatus};

const EVENT_USER_REGISTERED: &str = "USER_REGISTERED";
const EVENT_USER_APPROVED: &str = "USER_APPROVED";
const EVENT_USER_BANNED: &str = "USER_BANNED";
const EVENT_USER_SUSPENDED: &str = "USER_SUSPENDED";
const EVENT_COUNCIL_MEMBER_ADDED: &str = "COUNCIL_MEMBER_ADDED";

#[derive(Debug)]
pub enum Error {
    UserAlreadyExists,
    UserNotFound,
    Unauthorized,
    InsufficientReputation,
    VotingPeriodEnded,
    AlreadyVoted,
    InvalidVoteWeight,
}


#[derive(Debug)]
pub enum UserManagementError {
    UserAlreadyExists,
    UserNotFound,
    Unauthorized,
    VotingPeriodEnded,
    AlreadyVoted,
}

pub fn register_user(env: &Env, user: Address,name: Option<String>) -> Result<bool, UserManagementError> {
    let user_key = DataKey::User(user.clone());

    if env.storage().instance().has(&user_key) {
        return Err(UserManagementError::UserAlreadyExists);
    }   

    let new_user = User {
        address: user.clone(),
        name: name.unwrap_or(String::from_str(&env, "")),
        credit_score: 100,         // Initial credit score
        status: UserStatus::Active,
        join_date: env.ledger().timestamp(),
        is_dao_member: false,
        reputation_score: 50,      // Initial reputation score
        staked_amount: 0,         // No initial stake
        last_vote_timestamp: 0,   // Never voted        
    };

    env.storage().instance().set(&user_key, &new_user);
    env.events().publish((symbol_short!("usr_reg"), user.clone()), "new user created");
    Ok(true)
}

pub fn propose_user_ban(env: &Env, user: Address, dao_member: Address, reason: String) -> Result<bool, UserManagementError> {
    // Check if banner is DAO member
    if !check_dao_member(env, &dao_member)? {
        return Err(UserManagementError::Unauthorized);
    }

    // Create ban proposal
    let proposal = Proposal {
        id: env.ledger().sequence(),
        proposer: dao_member.clone(),
        proposal_type: ProposalType::UserBan,
        title: String::from_str(env, "User Ban"),
        description: reason,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + 604800, // 7 days
        yes_votes: 0,     // Each vote counts as 1
        no_votes: 0,      // Each vote counts as 1
        status: ProposalStatus::Open,
        execution_data: user.clone().to_object().into(),
        required_quorum: CONFIG_MIN_QUORUM, // Standard quorum for bans
        voters: SdkVec::new(&env),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal.id as u64), &proposal);
    
    env.events().publish(
        (symbol_short!("usrban"), user.clone()),
        (dao_member, proposal.id)
    );
    Ok(true)
}

pub fn execute_user_status_change(env: &Env, proposal_id: u64) -> Result<bool, UserManagementError> {
    let proposal = env.storage().instance().get::<_, Proposal>(&DataKey::DAOProposal(proposal_id))
        .ok_or(UserManagementError::UserNotFound)?;
    
    if proposal.status != ProposalStatus::Passed {
        return Err(UserManagementError::Unauthorized);
    }

    let user_address: Address = proposal.execution_data.try_into().unwrap();
    let mut user = env.storage().instance().get::<_, User>(&DataKey::User(user_address.clone()))
        .ok_or(UserManagementError::UserNotFound)?;

    match proposal.proposal_type {
        ProposalType::UserBan => {
            user.status = UserStatus::Banned;
            env.events().publish((symbol_short!("usrban"), user_address.clone()), proposal_id);
        },
        ProposalType::MembershipChange => {
            user.is_dao_member=true;
            env.events().publish((symbol_short!("usrdao"), user_address.clone()), proposal_id);
        },
        _ => return Err(UserManagementError::Unauthorized),
    }

    env.storage().instance().set(&DataKey::User(user_address), &user);
    Ok(true)
}

pub fn check_dao_member(env: &Env, address: &Address) -> Result<bool, UserManagementError> {
    let user = env.storage().instance().get::<_, User>(&DataKey::User(address.clone()))
        .ok_or(UserManagementError::UserNotFound)?;
    Ok(user.is_dao_member)
}

pub fn update_user_reputation(env: &Env, user: Address, change: i32, dao_member: Address) -> Result<bool, UserManagementError> {
    // Only DAO members can update reputation
    if !check_dao_member(env, &dao_member)? {
        return Err(UserManagementError::Unauthorized);
    }

    let user_key = DataKey::User(user.clone());
    let mut user_data = env.storage().instance().get::<_, User>(&user_key)
        .ok_or(UserManagementError::UserNotFound)?;

    // Update reputation (0-500 range) - just for gamification now
    let new_reputation = if change < 0 {
        user_data.reputation_score.saturating_sub((-change) as u32)
    } else {
        user_data.reputation_score.saturating_add(change as u32)
    };
    user_data.reputation_score = new_reputation.min(500);

    env.storage().instance().set(&user_key, &user_data);
    
    env.events().publish(
        (symbol_short!("repupd"), user.clone()),
        (change, user_data.reputation_score)
    );
    Ok(true)
}

pub fn get_user(env: &Env, user: Address) -> Result<User, UserManagementError> {
    env.storage().instance().get(&DataKey::User(user))
        .ok_or(UserManagementError::UserNotFound)
}


pub fn propose_user_dao(env: &Env, new_member: Address, appointer: Address,reason: String) -> Result<bool, UserManagementError>  {
    if !check_dao_member(env, &appointer)? {
        return Err(UserManagementError::Unauthorized);

    }


     let proposal = Proposal {
        id: env.ledger().sequence(),
        proposer: appointer.clone(),
        proposal_type: ProposalType::MembershipChange,
        title: String::from_str(env, "add to dao"),
        description: reason,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + 604800, // 7 days
        yes_votes: 0,     // Each vote counts as 1
        no_votes: 0,      // Each vote counts as 1
        status: ProposalStatus::Open,
        execution_data: new_member.clone().to_object().into(),
        required_quorum: CONFIG_MIN_QUORUM, // Standard quorum for bans
        voters: SdkVec::new(&env),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal.id as u64), &proposal);
    
    env.events().publish(
        (symbol_short!("usrban"), &new_member),
        (appointer, proposal.id)
    );
    Ok(true)
}

