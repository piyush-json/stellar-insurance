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
    VoteWeight(Address),
    ProposalConfig(ProposalType),
    LatestPolicyId,
    LatestSubscriptionId,
    LatestClaimId,
    LatestProposalId,
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
    pub subscribed_plan: Option<u64>, // Current active subscription
    pub village_contributions: i128, // Contributions to village
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
#[derive(Clone, Debug)]
#[contracttype]
pub struct Policy {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub params: PolicyParams,
    pub status: PolicyStatus,
    pub created_at: u64,
    pub creator: Address,
}

#[derive(Clone, Debug)]
#[contracttype]
pub struct PolicyParams {
    pub max_claim_amount: i128,
    pub interest_rate: u32,          // basis points (e.g., 500 = 5%)
    pub premium_amount: i128,
    pub premium_currency: String,     // "XLM"
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
    pub id: u64,
    pub policy_id: u64,
    pub subscriber: Address,
    pub start_date: u64,
    pub status: SubscriptionStatus,
    pub last_payment_date: u64,
    pub next_payment_due: u64,
    pub weeks_paid: u64,
    pub weeks_due: u64,
    pub grace_period_end: u64,
    pub total_premiums_paid: i128,
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
    pub id: u64,
    pub subscription_id: u64,
    pub claimer: Address,
    pub amount: i128,
    pub image_hash: BytesN<32>,
    pub description: String,
    pub status: ClaimStatus,
    pub created_at: u64,
    pub plan_id: u64,
    pub claim_type: ClaimType,
    pub assessor_notes: String,
    pub payout_date: Option<u64>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ClaimType {
    Standard,
    Emergency,
    NaturalDisaster,
    CropLoss,
}

// DAO Governance Structures

#[derive(Clone, Debug, Eq, PartialEq)]
#[contracttype]
pub enum ProposalType {
    PolicyCreation,      // Create new insurance policy
    PolicyArchival,      // Stop new subscriptions
    PolicyDeletion,      // Remove archived policy
    UserApproval,        // Approve new user registration
    UserBan,            // Ban user for violations
    GovernanceUpdate,    // Change DAO rules/parameters
    FinancialDecision,   // Treasury/pool management
    ClaimResolution,     // Vote on insurance claim
    EmergencyAction,     // Time-critical actions
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
    Active,            // Active for voting
    Rejected,          // Rejected
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
    pub id: u64,
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
    pub voting_period_end: u64,    // When voting ends
    pub votes_for: u32,            // Alternative vote counting
    pub votes_against: u32,        // Alternative vote counting
    pub quorum_required: i128,     // Required quorum
    pub created_date: u64,         // Creation timestamp
}

// Vote Record
#[derive(Clone)]
#[contracttype]
pub struct VoteRecord {
    pub proposal_id: u64,
    pub voter: Address,
    pub weight: VoteWeight,
    pub vote: bool,                // true for yes, false for no
    pub timestamp: u64,
    pub metadata: Option<String>,  // Optional vote comment/reason
}

// DAO Vote structure
#[derive(Clone)]
#[contracttype]
pub struct DAOVote {
    pub proposal_id: u64,
    pub voter: Address,
    pub vote_weight: i128,
    pub vote_direction: bool,
    pub timestamp: u64,
}

// Claim Vote structure
#[derive(Clone)]
#[contracttype]
pub struct ClaimVote {
    pub voter: Address,
    pub approve: bool,
    pub weight: i128,
    pub timestamp: u64,
}

// Assessment structure
#[derive(Clone)]
#[contracttype]
pub struct Assessment {
    pub assessor: Address,
    pub claim_id: u64,
    pub decision: bool,
    pub reasoning: String,
    pub assessment_date: u64,
    pub weight: i128,
}

// Payment structure
#[derive(Clone)]
#[contracttype]
pub struct Payment {
    pub user: Address,
    pub plan_id: u64,
    pub amount: i128,
    pub week_number: u64,
    pub payment_date: u64,
    pub penalty_applied: i128,
}

// Safety Pool structure
#[derive(Clone, Default)]
#[contracttype]
pub struct SafetyPool {
    pub total_balance: i128,
    pub premium_contributions: i128,
    pub claim_payouts: i128,
    pub investment_returns: i128,
    pub reserve_ratio: u64,
    pub last_audit_date: u64,
    pub minimum_reserve: i128,
}

// Platform Configuration
#[derive(Clone)]
#[contracttype]
pub struct PlatformConfig {
    pub grace_period_weeks: u64,
    pub minimum_quorum: u32,
    pub proposal_duration_days: u64,
    pub max_claim_amount_ratio: u64,
    pub penalty_rate: u32,
    pub council_size: u64,
}

// Investor Pool structure
#[derive(Clone)]
#[contracttype]
pub struct InvestorPool {
    pub investor: Address,
    pub total_deposited: i128,
    pub current_balance: i128,
    pub lock_in_end: u64,
    pub yield_earned: i128,
    pub last_update: u64,
}

// Credit Score structure
#[derive(Clone)]
#[contracttype]
pub struct CreditScore {
    pub user: Address,
    pub score: u32,
    pub last_updated: u64,
    pub history: Vec<CreditScoreChange>,
}

// Credit Score Change
#[derive(Clone)]
#[contracttype]
pub struct CreditScoreChange {
    pub timestamp: u64,
    pub change: i32,
    pub reason: String,
    pub source: String,
}



