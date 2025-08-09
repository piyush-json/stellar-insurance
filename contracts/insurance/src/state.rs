use soroban_sdk::{contracttype, Address, Bytes, BytesN, String, Vec};

// Contract DataKey for storage mapping
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    User(Address),
    Policy(u64),
    Subscription(u64),
    Claim(u64),
    DAOProposal(u64),
    DAOVote(u64),
    InvestorPool(Address),
    CreditScore(Address),
}

// Composite key for plans
use core::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[contracttype]
pub struct PlanKey {
    pub creator: Address,
    pub name: String,
    pub created_date: u64,
}

impl Hash for PlanKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.creator.to_object().hash(state);
        self.name.to_object().hash(state);
        self.created_date.hash(state);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum UserStatus {
    Pending,
    Active,
    Banned,
}

#[derive(Clone)]
#[contracttype]
pub struct User {
    pub address: Address,
    pub name: String,
    pub credit_score: u32,
    pub status: UserStatus,
    pub join_date: u64,
    pub is_dao_member: bool,
    pub reputation_score: u32,      // 0-500, affects vote weight
    pub staked_amount: i128,        // Amount of XLM staked
    pub last_vote_timestamp: u64,   // For vote cooldown
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum PolicyStatus {
    Pending,
    Active,
    Archived,
    Deleted,
}

// Enhanced Plan structure
#[derive(Clone)]
#[contracttype]
// Plan Status
pub struct Policy {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub params: PolicyParams,
    pub status: PolicyStatus,
    pub created_at: u64,
    pub creator: Address,
}

#[derive(Clone)]
#[contracttype]
pub struct PolicyParams {
    pub max_claim_amount: i128,
    pub interest_rate: u32,          // basis points (e.g., 500 = 5%)
    pub premium_amount: i128,
    pub claim_cooldown_days: u32,
    pub investor_lock_in_days: u32,
    pub requires_dao_approval: bool,
    pub credit_slash_on_reject: u32,
}

// Subscription Status
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum SubscriptionStatus {
    Active,
    GracePeriod,
    Suspended,
    Cancelled,
}

// Enhanced Subscription structure
#[derive(Clone)]
#[contracttype]
pub struct Subscription {
    pub id: u32,
    pub policy_id: u32,
    pub subscriber: Address,
    pub start_date: u64,
    pub status: SubscriptionStatus,
    pub last_payment_date: u64,
    pub next_payment_due: u64,
}
// Enhanced Claim Status
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ClaimStatus {
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Paid,
    Disputed,
}

#[derive(Clone)]
#[contracttype]
pub struct Claim {
    pub id: u32,
    pub subscription_id: u32,
    pub claimer: Address,
    pub amount: i128,
    pub image_hash: BytesN<32>,
    pub description: String,
    pub status: ClaimStatus,
    pub created_at: u64,
}
// DAO Governance Structures

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ProposalType {
    PolicyCreation,      // Create new insurance policy
    PolicyArchival,      // Stop new subscriptions
    PolicyDeletion,      // Remove archived policy
    UserBan,            // Ban user for violations
    ClaimResolution,     // Vote on insurance claim
    MembershipChange,    // Add/remove DAO member
}

// Proposal Status
#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ProposalStatus {
    Open,               // Accepting votes
    Passed,            // Approved and executed
    Failed,            // Rejected due to votes
    Expired,           // Time limit reached
    Executing,         // In process of execution
    Invalid,           // Found to be invalid
}

// Vote Weight structure
#[derive(Clone)]
#[contracttype]
pub struct VoteWeight {
    pub voter: Address,
    pub base_weight: u32,           // Base voting power (1)
    pub reputation_boost: u32,      // From reputation (0-5)
    pub stake_boost: u32,          // From XLM staked
    pub last_updated: u64,         // When weights were last calculated
}

// Proposal Configuration
#[derive(Clone)]
#[contracttype]
pub struct ProposalConfig {
    pub proposal_type: ProposalType,
    pub quorum_multiplier: u32,     // Multiplier for base quorum
    pub min_duration: u64,          // Minimum voting duration
    pub max_duration: u64,          // Maximum voting duration
    pub min_voter_reputation: u32,  // Minimum reputation to vote
    pub approval_threshold: u32,    // Percentage needed to pass (e.g., 7500 = 75%)
}

// Enhanced Proposal structure
#[derive(Clone)]
#[contracttype]
pub struct Proposal {
    pub id: u32,
    pub proposer: Address,
    pub proposal_type: ProposalType,
    pub title: String,
    pub description: String,
    pub start_time: u64,           // When voting begins
    pub end_time: u64,             // When voting ends
    pub yes_votes: u32,   // Sum of weighted yes votes
    pub no_votes: u32,    // Sum of weighted no votes
    pub status: ProposalStatus,
    pub execution_data: Bytes,     // Type-specific execution data
    pub required_quorum: u32,      // Required weighted votes
    pub voters: Vec<Address>,      // List of addresses that voted
}

// Vote Record
#[derive(Clone)]
#[contracttype]
pub struct VoteRecord {
    pub proposal_id: u32,
    pub voter: Address,
    pub weight: VoteWeight,
    pub vote: bool,                // true for yes, false for no
    pub timestamp: u64,
    pub metadata: Option<String>,  // Optional vote comment/reason
}



