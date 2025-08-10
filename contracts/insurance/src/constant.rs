use soroban_sdk::{symbol_short, Symbol};

// Storage Keys
pub const STORAGE_USERS: Symbol = symbol_short!("users");
pub const STORAGE_POLICIES: Symbol = symbol_short!("policies");
pub const STORAGE_SUBS: Symbol = symbol_short!("subs");
pub const STORAGE_CLAIMS: Symbol = symbol_short!("claims");
pub const STORAGE_DAOPROP: Symbol = symbol_short!("daoprop");
pub const STORAGE_DAOVOTE: Symbol = symbol_short!("daovote");
pub const STORAGE_INVPOOL: Symbol = symbol_short!("invpool");
pub const STORAGE_CREDIT: Symbol = symbol_short!("credit");

// Legacy storage keys for compatibility
pub const USERS: Symbol = symbol_short!("users");
pub const POLICIES: Symbol = symbol_short!("policies");
pub const SUBSCRIPTIONS: Symbol = symbol_short!("subs");
pub const CLAIMS: Symbol = symbol_short!("claims");
pub const PROPOSALS: Symbol = symbol_short!("daoprop");
pub const DAO_VOTES: Symbol = symbol_short!("daovote");
pub const VOTE_DELEGATIONS: Symbol = symbol_short!("votedel");
pub const PLATFORM_CONFIG: Symbol = symbol_short!("platform");
pub const SAFETY_POOL: Symbol = symbol_short!("safety");
pub const PREMIUM_PAYMENTS: Symbol = symbol_short!("payments");
pub const CLAIM_ASSESSMENTS: Symbol = symbol_short!("assess");
pub const CLAIM_VOTES: Symbol = symbol_short!("clmvote");

// User Events
pub const EVENT_USER_REG: Symbol = symbol_short!("UsrReg");
pub const EVENT_USER_BAN: Symbol = symbol_short!("UsrBan");
pub const EVENT_DAO_ADD: Symbol = symbol_short!("DAOAdd");
pub const EVENT_DAO_REM: Symbol = symbol_short!("DAORem");

// Policy Events
pub const EVENT_POL_CRT: Symbol = symbol_short!("PolCrt");
pub const EVENT_POL_ARC: Symbol = symbol_short!("PolArc");
pub const EVENT_POL_DEL: Symbol = symbol_short!("PolDel");

// Subscription Events
pub const EVENT_SUB_CRT: Symbol = symbol_short!("SubCrt");
pub const EVENT_SUB_PAY: Symbol = symbol_short!("SubPay");
pub const EVENT_SUB_GRC: Symbol = symbol_short!("SubGrc");
pub const EVENT_SUB_SUS: Symbol = symbol_short!("SubSus");

// Claim Events
pub const EVENT_CLM_SUB: Symbol = symbol_short!("ClmSub");
pub const EVENT_CLM_VOT: Symbol = symbol_short!("ClmVot");
pub const EVENT_CLM_APP: Symbol = symbol_short!("ClmApp");
pub const EVENT_CLM_REJ: Symbol = symbol_short!("ClmRej");
pub const EVENT_CLM_PAY: Symbol = symbol_short!("ClmPay");

// Investment Events
pub const EVENT_INV_DEP: Symbol = symbol_short!("InvDep");
pub const EVENT_INV_WDR: Symbol = symbol_short!("InvWdr");
pub const EVENT_INV_YLD: Symbol = symbol_short!("InvYld");

// DAO Events
pub const PROPOSAL_CREATED: Symbol = symbol_short!("PropCrt");
pub const VOTE_CAST: Symbol = symbol_short!("VoteCast");
pub const PROPOSAL_EXECUTED: Symbol = symbol_short!("PropExe");
pub const VOTE_DELEGATED: Symbol = symbol_short!("VoteDel");

// Claim Events
pub const CLAIM_SUBMITTED: Symbol = symbol_short!("ClmSub");
pub const CLAIM_ASSESSED: Symbol = symbol_short!("ClmAss");
pub const CLAIM_VOTED: Symbol = symbol_short!("ClmVot");
pub const CLAIM_APPROVED: Symbol = symbol_short!("ClmApp");
pub const CLAIM_REJECTED: Symbol = symbol_short!("ClmRej");
pub const CLAIM_DISPUTED: Symbol = symbol_short!("ClmDis");
pub const CLAIM_PAID: Symbol = symbol_short!("ClmPay");

// Financial Events
pub const EXTERNAL_FUNDING_ADDED: Symbol = symbol_short!("ExtFund");
pub const RESERVE_FUNDS_WITHDRAWN: Symbol = symbol_short!("ResWdr");
pub const INVESTMENT_RETURNS_UPDATED: Symbol = symbol_short!("InvRet");
pub const MINIMUM_RESERVE_UPDATED: Symbol = symbol_short!("MinRes");
pub const RESERVE_RATIO_UPDATED: Symbol = symbol_short!("ResRat");
pub const FINANCIAL_AUDIT_COMPLETED: Symbol = symbol_short!("FinAud");
pub const AUDIT_DISCREPANCY_FOUND: Symbol = symbol_short!("AudDis");
pub const PLATFORM_CONFIG_UPDATED: Symbol = symbol_short!("PlatCfg");
pub const EMERGENCY_FUND_FREEZE: Symbol = symbol_short!("EmerFrz");
pub const EMERGENCY_FUND_UNFREEZE: Symbol = symbol_short!("EmerUnfrz");

// Subscription Events
pub const PLAN_SUBSCRIBED: Symbol = symbol_short!("PlanSub");
pub const PREMIUM_PAID: Symbol = symbol_short!("PremPaid");
pub const SUBSCRIPTION_CANCELLED: Symbol = symbol_short!("SubCan");
pub const SUBSCRIPTION_REACTIVATED: Symbol = symbol_short!("SubReact");
pub const SUBSCRIPTION_GRACE_PERIOD: Symbol = symbol_short!("SubGrace");
pub const SUBSCRIPTION_SUSPENDED: Symbol = symbol_short!("SubSus");

// Policy Events
pub const POL_PROP: Symbol = symbol_short!("PolProp");
pub const POL_ARCH: Symbol = symbol_short!("PolArch");
pub const POL_DEL: Symbol = symbol_short!("PolDel");
pub const POL_ACT: Symbol = symbol_short!("PolAct");

// User Events
pub const USR_REG: Symbol = symbol_short!("UsrReg");
pub const USR_BAN: Symbol = symbol_short!("UsrBan");
pub const USR_DAO: Symbol = symbol_short!("UsrDao");
pub const REP_UPD: Symbol = symbol_short!("RepUpd");

// Platform Configuration Constants
pub const CONFIG_GRACE_WEEKS: u64 = 2;
pub const CONFIG_MIN_QUORUM: u32 = 3;
pub const CONFIG_PROP_DAYS: u64 = 7;
pub const CONFIG_CLAIM_COOLDOWN: u32 = 30;
pub const CONFIG_INV_LOCKIN: u32 = 90;
pub const CONFIG_INTEREST_RATE: u32 = 500;  // 5% in basis points
pub const CONFIG_CREDIT_SLASH: u32 = 10;    // 10 points slashed on claim rejection

// Default values for platform configuration
pub const DEFAULT_GRACE_PERIOD_WEEKS: u64 = 2;
pub const DEFAULT_MINIMUM_QUORUM: u32 = 3;
pub const DEFAULT_PROPOSAL_DURATION_DAYS: u64 = 7;
pub const DEFAULT_MAX_CLAIM_AMOUNT_RATIO: u64 = 80;
pub const DEFAULT_PENALTY_RATE: u32 = 500;
pub const DEFAULT_COUNCIL_SIZE: u64 = 5;
pub const DEFAULT_RESERVE_RATIO: u64 = 7000; // 70% in basis points
pub const DEFAULT_MINIMUM_RESERVE: i128 = 10000;
