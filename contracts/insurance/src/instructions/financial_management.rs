use soroban_sdk::{Env, Address, Map, String};
use crate::state::{SafetyPool, Payment, PlatformConfig};
use crate::instructions::user_management::is_council_member;
use crate::constant::{SAFETY_POOL, PREMIUM_PAYMENTS, CLAIMS, PLATFORM_CONFIG, EMERGENCY_FUND_FREEZE, EXTERNAL_FUNDING_ADDED, RESERVE_FUNDS_WITHDRAWN, INVESTMENT_RETURNS_UPDATED, MINIMUM_RESERVE_UPDATED, RESERVE_RATIO_UPDATED, FINANCIAL_AUDIT_COMPLETED, AUDIT_DISCREPANCY_FOUND, PLATFORM_CONFIG_UPDATED, EMERGENCY_FUND_UNFREEZE};

pub fn get_safety_pool_balance(env: &Env) -> i128 {
    let safety_pool: SafetyPool = env.storage().instance().get(&SAFETY_POOL).unwrap_or_default();
    return safety_pool.total_balance;
}

pub fn get_safety_pool_details(env: &Env) -> SafetyPool {
    env.storage().instance().get(&SAFETY_POOL).unwrap_or(SafetyPool {
        total_balance: 0,
        premium_contributions: 0,
        claim_payouts: 0,
        investment_returns: 0,
        reserve_ratio: 7000, // 70% in basis points
        last_audit_date: env.ledger().timestamp(),
        minimum_reserve: 10000,
    })
}

pub fn add_external_funding(env: &Env, funder: Address, amount: i128) -> bool {
    // Only council members can add external funding
    if !is_council_member(env, &funder) {
        return false;
    }

    if amount <= 0 {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    safety_pool.total_balance += amount;
    safety_pool.investment_returns += amount; // Treating external funding as investment returns

    env.storage().instance().set(&SAFETY_POOL, &safety_pool);
    env.events().publish((EXTERNAL_FUNDING_ADDED, funder), amount);
    
    true
}

pub fn withdraw_reserve_funds(env: &Env, withdrawer: Address, amount: i128, purpose: String) -> bool {
    // Only council members can withdraw reserve funds
    if !is_council_member(env, &withdrawer) {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    
    // Check if withdrawal would violate minimum reserve requirements
    let reserve_after_withdrawal = safety_pool.total_balance - amount;
    if reserve_after_withdrawal < safety_pool.minimum_reserve {
        return false;
    }

    // Check reserve ratio
    let config = get_platform_config(env);
    let minimum_reserve_amount = (safety_pool.premium_contributions * config.max_claim_amount_ratio as i128) / 10000;
    
    if reserve_after_withdrawal < minimum_reserve_amount {
        return false;
    }

    safety_pool.total_balance -= amount;
    env.storage().instance().set(&SAFETY_POOL, &safety_pool);

    env.events().publish((RESERVE_FUNDS_WITHDRAWN, withdrawer.clone()), (amount, purpose));
    true
}

pub fn update_investment_returns(env: &Env, updater: Address, returns: i128) -> bool {
    // Only council members can update investment returns
    if !is_council_member(env, &updater) {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    safety_pool.total_balance += returns;
    safety_pool.investment_returns += returns;

    env.storage().persistent().set(&SAFETY_POOL, &safety_pool);
    env.events().publish((INVESTMENT_RETURNS_UPDATED, updater), returns);
    
    true
}

pub fn set_minimum_reserve(env: &Env, setter: Address, new_minimum: i128) -> bool {
    // Only council members can set minimum reserve
    if !is_council_member(env, &setter) {
        return false;
    }

    if new_minimum < 0 {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    safety_pool.minimum_reserve = new_minimum;

    env.storage().persistent().set(&SAFETY_POOL, &safety_pool);
    env.events().publish((MINIMUM_RESERVE_UPDATED, setter), new_minimum);
    
    true
}

pub fn update_reserve_ratio(env: &Env, updater: Address, new_ratio: u64) -> bool {
    // Only council members can update reserve ratio
    if !is_council_member(env, &updater) {
        return false;
    }

    // Ratio should be between 0 and 10000 basis points (0-100%)
    if new_ratio > 10000 {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    safety_pool.reserve_ratio = new_ratio;

    env.storage().persistent().set(&SAFETY_POOL, &safety_pool);
    env.events().publish((RESERVE_RATIO_UPDATED, updater), new_ratio);
    
    true
}

pub fn conduct_financial_audit(env: &Env, auditor: Address) -> bool {
    // Only council members can conduct audits
    if !is_council_member(env, &auditor) {
        return false;
    }

    let mut safety_pool = get_safety_pool_details(env);
    safety_pool.last_audit_date = env.ledger().timestamp();

    // Perform audit calculations
    let (total_premiums, total_claims, net_balance) = calculate_financial_summary(env);
    
    // Verify the safety pool balance matches calculated balance
    let expected_balance = total_premiums + safety_pool.investment_returns - total_claims;
    
    if (safety_pool.total_balance - expected_balance).abs() > 100 { // Allow small discrepancies
        env.events().publish((AUDIT_DISCREPANCY_FOUND, auditor.clone()), 
                           (safety_pool.total_balance, expected_balance));
    }

    env.storage().persistent().set(&SAFETY_POOL, &safety_pool);
    env.events().publish((FINANCIAL_AUDIT_COMPLETED, auditor), 
                       (total_premiums, total_claims, net_balance));
    
    true
}

pub fn get_financial_summary(env: &Env) -> (i128, i128, i128, i128) {
    // Returns (total_premiums, total_claims, net_balance, reserve_percentage)
    let (total_premiums, total_claims, net_balance) = calculate_financial_summary(env);
    let safety_pool = get_safety_pool_details(env);
    
    let reserve_percentage = if total_premiums > 0 {
        (safety_pool.total_balance * 10000) / total_premiums // Basis points
    } else {
        0
    };

    (total_premiums, total_claims, net_balance, reserve_percentage)
}

pub fn check_reserve_health(env: &Env) -> (bool, i128, i128) {
    // Returns (is_healthy, current_reserve, minimum_required)
    let safety_pool = get_safety_pool_details(env);
    let config = get_platform_config(env);
    
    let minimum_required = (safety_pool.premium_contributions * config.max_claim_amount_ratio as i128) / 10000;
    let is_healthy = safety_pool.total_balance >= minimum_required && 
                     safety_pool.total_balance >= safety_pool.minimum_reserve;

    (is_healthy, safety_pool.total_balance, minimum_required)
}

pub fn calculate_claim_capacity(env: &Env) -> i128 {
    // Calculate how much can be paid out in claims based on current reserves
    let safety_pool = get_safety_pool_details(env);
    let available_for_claims = safety_pool.total_balance - safety_pool.minimum_reserve;
    
    available_for_claims.max(0)
}

pub fn get_premium_payment_summary(env: &Env) -> (i128, u64, i128) {
    // Returns (total_collected, payment_count, average_payment)
    use soroban_sdk::Vec;
    
    let payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PREMIUM_PAYMENTS).unwrap_or_else(|| Map::new(env));
    let mut total_collected = 0i128;
    let mut payment_count = 0u64;

    for (_, payment_list) in payments.iter() {
        for payment in payment_list.iter() {
            total_collected += payment.amount;
            payment_count += 1;
        }
    }

    let average_payment = if payment_count > 0 {
        total_collected / payment_count as i128
    } else {
        0
    };

    (total_collected, payment_count, average_payment)
}

pub fn get_recent_financial_activity(env: &Env, days_back: u64) -> (i128, i128, u64, u64) {
    // Returns (recent_premiums, recent_claims, premium_transactions, claim_transactions)
    use soroban_sdk::Vec;
    use crate::state::{Claim, ClaimStatus};
    
    let cutoff_time = env.ledger().timestamp() - (days_back * 24 * 60 * 60);
    let mut recent_premiums = 0i128;
    let mut recent_claims = 0i128;
    let mut premium_count = 0u64;
    let mut claim_count = 0u64;

    // Count recent premium payments
    let payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PREMIUM_PAYMENTS).unwrap_or_else(|| Map::new(env));
    for (_, payment_list) in payments.iter() {
        for payment in payment_list.iter() {
            if payment.payment_date >= cutoff_time {
                recent_premiums += payment.amount;
                premium_count += 1;
            }
        }
    }

    // Count recent claim payouts
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    for (_, claim) in claims.iter() {
        if claim.status == ClaimStatus::Paid {
            if let Some(payout_date) = claim.payout_date {
                if payout_date >= cutoff_time {
                    recent_claims += claim.amount;
                    claim_count += 1;
                }
            }
        }
    }

    (recent_premiums, recent_claims, premium_count, claim_count)
}

fn calculate_financial_summary(env: &Env) -> (i128, i128, i128) {
    use soroban_sdk::Vec;
    use crate::state::{Claim, ClaimStatus};
    
    // Calculate total premiums
    let payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PREMIUM_PAYMENTS).unwrap_or_else(|| Map::new(env));
    let mut total_premiums = 0i128;
    
    for (_, payment_list) in payments.iter() {
        for payment in payment_list.iter() {
            total_premiums += payment.amount;
        }
    }

    // Calculate total claims paid
    let claims: Map<u64, Claim> = env.storage().persistent().get(&CLAIMS).unwrap_or_else(|| Map::new(env));
    let mut total_claims = 0i128;
    
    for (_, claim) in claims.iter() {
        if claim.status == ClaimStatus::Paid {
            total_claims += claim.amount;
        }
    }

    let net_balance = total_premiums - total_claims;
    (total_premiums, total_claims, net_balance)
}

fn get_platform_config(env: &Env) -> PlatformConfig {
    env.storage().persistent().get(&PLATFORM_CONFIG).unwrap_or(PlatformConfig {
        grace_period_weeks: 2,
        minimum_quorum: 3,
        proposal_duration_days: 7,
        max_claim_amount_ratio: 80,
        penalty_rate: 500,
        council_size: 5,
    })
}

pub fn set_platform_config(env: &Env, setter: Address, config: PlatformConfig) -> bool {
    // Only council members can update platform configuration
    if !is_council_member(env, &setter) {
        return false;
    }

    env.storage().persistent().set(&PLATFORM_CONFIG, &config);
    env.events().publish((PLATFORM_CONFIG_UPDATED, setter), ());
    
    true
}

pub fn emergency_fund_freeze(env: &Env, freezer: Address, reason: String) -> bool {
    // Only council members can freeze funds in emergency
    if !is_council_member(env, &freezer) {
        return false;
    }

    // Set a freeze flag
    env.storage().persistent().set(&EMERGENCY_FUND_FREEZE, &true);
    env.events().publish((EMERGENCY_FUND_FREEZE, freezer), reason);
    
    true
}

pub fn emergency_fund_unfreeze(env: &Env, unfreezer: Address) -> bool {
    // Only council members can unfreeze funds
    if !is_council_member(env, &unfreezer) {
        return false;
    }

    env.storage().persistent().set(&EMERGENCY_FUND_FREEZE, &false);
    env.events().publish((EMERGENCY_FUND_UNFREEZE, unfreezer), ());
    
    true
}

pub fn is_fund_frozen(env: &Env) -> bool {
    env.storage().persistent().get(&EMERGENCY_FUND_FREEZE).unwrap_or(false)
}
