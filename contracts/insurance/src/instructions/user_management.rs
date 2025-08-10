use soroban_sdk::xdr::ToXdr;
use soroban_sdk::{symbol_short, Address, Env, IntoVal, String, TryIntoVal, Vec as SdkVec};
use crate::constant::{CONFIG_MIN_QUORUM, USR_REG, USR_BAN, USR_DAO, REP_UPD};
use crate::state::{DataKey, User, UserStatus, ProposalType, Proposal, ProposalStatus};

#[derive(Debug)]
pub enum UserManagementError {
    UserAlreadyExists,
    UserNotFound,
    Unauthorized,
    VotingPeriodEnded,
    AlreadyVoted,
}

pub fn register_user(env: &Env, user: Address, name: Option<String>) -> Result<bool, UserManagementError> {
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
        subscribed_plan: None,     // No initial subscription
        village_contributions: 0,  // No initial contributions
    };

    env.storage().instance().set(&user_key, &new_user);
    env.events().publish((USR_REG, user.clone()), "new user created");
    Ok(true)
}

pub fn approve_user(env: &Env, user: Address, approver: Address) -> Result<bool, UserManagementError> {
    // Check if approver is DAO member
    if !check_dao_member(env, &approver)? {
        return Err(UserManagementError::Unauthorized);
    }

    let user_key = DataKey::User(user.clone());
    let mut user_data = env.storage().instance().get::<_, User>(&user_key)
        .ok_or(UserManagementError::UserNotFound)?;

    user_data.status = UserStatus::Active;
    env.storage().instance().set(&user_key, &user_data);
    
    env.events().publish((USR_REG, user.clone()), "user approved");
    Ok(true)
}

pub fn suspend_user(env: &Env, user: Address, admin: Address, reason: String) -> Result<bool, UserManagementError> {
    // Check if admin is DAO member
    if !check_dao_member(env, &admin)? {
        return Err(UserManagementError::Unauthorized);
    }

    let user_key = DataKey::User(user.clone());
    let mut user_data = env.storage().instance().get::<_, User>(&user_key)
        .ok_or(UserManagementError::UserNotFound)?;

    user_data.status = UserStatus::Pending;
    env.storage().instance().set(&user_key, &user_data);
    
    env.events().publish((USR_BAN, user.clone()), reason);
    Ok(true)
}

pub fn ban_user(env: &Env, user: Address, admin: Address, reason: String) -> Result<bool, UserManagementError> {
    // Check if admin is DAO member
    if !check_dao_member(env, &admin)? {
        return Err(UserManagementError::Unauthorized);
    }

    let user_key = DataKey::User(user.clone());
    let mut user_data = env.storage().instance().get::<_, User>(&user_key)
        .ok_or(UserManagementError::UserNotFound)?;

    user_data.status = UserStatus::Banned;
    env.storage().instance().set(&user_key, &user_data);
    
    env.events().publish((USR_BAN, user.clone()), reason);
    Ok(true)
}

pub fn add_council_member(env: &Env, new_member: Address, appointer: Address) -> Result<bool, UserManagementError> {
    // Check if appointer is DAO member
    if !check_dao_member(env, &appointer)? {
        return Err(UserManagementError::Unauthorized);
    }

    let user_key = DataKey::User(new_member.clone());
    let mut user_data = env.storage().instance().get::<_, User>(&user_key)
        .ok_or(UserManagementError::UserNotFound)?;

    user_data.is_dao_member = true;
    env.storage().instance().set(&user_key, &user_data);
    
    env.events().publish((USR_DAO, new_member.clone()), "added to DAO");
    Ok(true)
}

pub fn propose_user_ban(env: &Env, user: Address, dao_member: Address, reason: String) -> Result<bool, UserManagementError> {
    // Check if banner is DAO member
    if !check_dao_member(env, &dao_member)? {
        return Err(UserManagementError::Unauthorized);
    }

    // Create ban proposal
    let proposal = Proposal {
        id: env.ledger().sequence() as u64,
        proposer: dao_member.clone(),
        proposal_type: ProposalType::UserBan,
        title: String::from_str(&env, "User Ban"),
        description: reason,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + 604800, // 7 days
        yes_votes: 0,     // Each vote counts as 1
        no_votes: 0,      // Each vote counts as 1
        status: ProposalStatus::Open,
        execution_data: user.clone().to_object().to_xdr(&env).try_into().unwrap(),
        required_quorum: CONFIG_MIN_QUORUM, // Standard quorum for bans
        voters: SdkVec::new(&env),
        voting_period_end: env.ledger().timestamp() + 604800,
        votes_for: 0,
        votes_against: 0,
        quorum_required: CONFIG_MIN_QUORUM as i128,
        created_date: env.ledger().timestamp(),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal.id), &proposal);
    
    env.events().publish(
        (USR_BAN, user.clone()),
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

    let user_address: Address = proposal.execution_data.try_into_val(env).unwrap();
    let mut user = env.storage().instance().get::<_, User>(&DataKey::User(user_address.clone()))
        .ok_or(UserManagementError::UserNotFound)?;

    match proposal.proposal_type {
        ProposalType::UserBan => {
            user.status = UserStatus::Banned;
            env.events().publish((USR_BAN, user_address.clone()), proposal_id);
        },
        ProposalType::MembershipChange => {
            user.is_dao_member = true;
            env.events().publish((USR_DAO, user_address.clone()), proposal_id);
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
        (REP_UPD, user.clone()),
        (change, user_data.reputation_score)
    );
    Ok(true)
}

pub fn get_user(env: &Env, user: Address) -> Result<User, UserManagementError> {
    env.storage().instance().get(&DataKey::User(user))
        .ok_or(UserManagementError::UserNotFound)
}

pub fn propose_user_dao(env: &Env, new_member: Address, appointer: Address, reason: String) -> Result<bool, UserManagementError>  {
    if !check_dao_member(env, &appointer)? {
        return Err(UserManagementError::Unauthorized);
    }

     let proposal = Proposal {
        id: env.ledger().sequence() as u64,
        proposer: appointer.clone(),
        proposal_type: ProposalType::MembershipChange,
        title: String::from_str(&env, "add to dao"),
        description: reason,
        start_time: env.ledger().timestamp(),
        end_time: env.ledger().timestamp() + 604800, // 7 days
        yes_votes: 0,     // Each vote counts as 1
        no_votes: 0,      // Each vote counts as 1
        status: ProposalStatus::Open,
        execution_data: new_member.clone().to_object().to_xdr(&env).try_into().unwrap(),
        required_quorum: CONFIG_MIN_QUORUM, // Standard quorum for bans
        voters: SdkVec::new(&env),
        voting_period_end: env.ledger().timestamp() + 604800,
        votes_for: 0,
        votes_against: 0,
        quorum_required: CONFIG_MIN_QUORUM as i128,
        created_date: env.ledger().timestamp(),
    };

    env.storage().instance().set(&DataKey::DAOProposal(proposal.id), &proposal);
    
    env.events().publish(
        (USR_DAO, new_member.clone()),
        (appointer, proposal.id)
    );
    Ok(true)
}

// Helper functions for other modules
pub fn is_user_approved(env: &Env, address: &Address) -> bool {
    if let Ok(user) = get_user(env, address.clone()) {
        matches!(user.status, UserStatus::Active)
    } else {
        false
    }
}

pub fn is_council_member(env: &Env, address: &Address) -> bool {
    if let Ok(user) = get_user(env, address.clone()) {
        user.is_dao_member
    } else {
        false
    }
}

