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

// Platform Configuration Constants
pub const CONFIG_GRACE_WEEKS: u64 = 2;
pub const CONFIG_MIN_QUORUM: u32 = 3;
pub const CONFIG_PROP_DAYS: u64 = 7;
pub const CONFIG_CLAIM_COOLDOWN: u32 = 30;
pub const CONFIG_INV_LOCKIN: u32 = 90;
pub const CONFIG_INTEREST_RATE: u32 = 500;  // 5% in basis points
pub const CONFIG_CREDIT_SLASH: u32 = 10;    // 10 points slashed on claim rejection
