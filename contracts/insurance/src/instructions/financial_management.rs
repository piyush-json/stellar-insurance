use soroban_sdk::{Address, Env, IntoVal, Map, String, Symbol, Vec};
use crate::state::{SafetyPool, Payment, PlatformConfig};
use crate::instructions::user_management::is_council_member;
use crate::constant::{
    SAFETY_POOL, PREMIUM_PAYMENTS, CLAIMS, PLATFORM_CONFIG, EMERGENCY_FUND_FREEZE,
    EXTERNAL_FUNDING_ADDED, RESERVE_FUNDS_WITHDRAWN, INVESTMENT_RETURNS_UPDATED,
    MINIMUM_RESERVE_UPDATED, RESERVE_RATIO_UPDATED, FINANCIAL_AUDIT_COMPLETED,
    AUDIT_DISCREPANCY_FOUND, PLATFORM_CONFIG_UPDATED, EMERGENCY_FUND_UNFREEZE
};

#[derive(Debug, Clone)]
pub enum FinancialManagementError {
    Unauthorized,
    InvalidAmount,
    InsufficientReserves,
    InvalidRatio,
    FundFrozen,
    StorageError,
    ValidationError,
}

pub type FinancialResult<T> = Result<T, FinancialManagementError>;

pub struct FinancialManagementService;

impl FinancialManagementService {
        pub fn get_safety_pool_balance(env: &Env) -> i128 {
        Self::get_safety_pool(env).total_balance
    }

    pub fn get_safety_pool_details(env: &Env) -> SafetyPool {
        Self::get_safety_pool(env)
    }

    pub fn add_external_funding(
        env: &Env, 
        funder: Address, 
        amount: i128
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &funder)?;
        
                Self::validate_positive_amount(amount)?;
        
                Self::ensure_funds_not_frozen(env)?;

        let mut safety_pool = Self::get_safety_pool(env);
        safety_pool.total_balance += amount;
        safety_pool.investment_returns += amount;

        Self::save_safety_pool(env, &safety_pool);
        Self::emit_event(env, EXTERNAL_FUNDING_ADDED, funder, amount);
        
        Ok(true)
    }

    pub fn withdraw_reserve_funds(
        env: &Env, 
        withdrawer: Address, 
        amount: i128, 
        purpose: String
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &withdrawer)?;
        
                Self::ensure_positive_amount(amount)?;
        
                Self::ensure_funds_not_frozen(env)?;

        let mut safety_pool = Self::get_safety_pool(env);
        
                Self::validate_reserve_withdrawal(env, &safety_pool, amount)?;

        safety_pool.total_balance -= amount;
        Self::save_safety_pool(env, &safety_pool);

        Self::emit_event(env, RESERVE_FUNDS_WITHDRAWN, withdrawer, (amount, purpose));
        Ok(true)
    }

    pub fn update_investment_returns(
        env: &Env, 
        updater: Address, 
        returns: i128
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &updater)?;
        
                Self::ensure_funds_not_frozen(env)?;

        let mut safety_pool = Self::get_safety_pool(env);
        safety_pool.total_balance += returns;
        safety_pool.investment_returns += returns;

        Self::save_safety_pool(env, &safety_pool);
        Self::emit_event(env, INVESTMENT_RETURNS_UPDATED, updater, returns);
        
        Ok(true)
    }

    pub fn set_minimum_reserve(
        env: &Env, 
        setter: Address, 
        new_minimum: i128
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &setter)?;
        
                Self::ensure_positive_amount(new_minimum)?;

        let mut safety_pool = Self::get_safety_pool(env);
        safety_pool.minimum_reserve = new_minimum;

        Self::save_safety_pool(env, &safety_pool);
        Self::emit_event(env, MINIMUM_RESERVE_UPDATED, setter, new_minimum);
        
        Ok(true)
    }

    pub fn update_reserve_ratio(
        env: &Env, 
        updater: Address, 
        new_ratio: u64
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &updater)?;
        
                Self::validate_percentage_ratio(new_ratio)?;

        let mut safety_pool = Self::get_safety_pool(env);
        safety_pool.reserve_ratio = new_ratio;

        Self::save_safety_pool(env, &safety_pool);
        Self::emit_event(env, RESERVE_RATIO_UPDATED, updater, new_ratio);
        
        Ok(true)
    }

        pub fn conduct_financial_audit(env: &Env, auditor: Address) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &auditor)?;

        let mut safety_pool = Self::get_safety_pool(env);
        safety_pool.last_audit_date = env.ledger().timestamp();

                let (total_premiums, total_claims, net_balance) = Self::calculate_financial_summary(env);
        
                let expected_balance = total_premiums + safety_pool.investment_returns - total_claims;
        let discrepancy = (safety_pool.total_balance - expected_balance).abs();
        
        if discrepancy > 100 {             Self::emit_event(env, AUDIT_DISCREPANCY_FOUND, auditor.clone(), 
                           (safety_pool.total_balance, expected_balance));
        }

        Self::save_safety_pool(env, &safety_pool);
        Self::emit_event(env, FINANCIAL_AUDIT_COMPLETED, auditor, 
                       (total_premiums, total_claims, net_balance));
        
        Ok(true)
    }

        pub fn get_financial_summary(env: &Env) -> (i128, i128, i128, i128) {
        let (total_premiums, total_claims, net_balance) = Self::calculate_financial_summary(env);
        let safety_pool = Self::get_safety_pool(env);
        
        let reserve_percentage = if total_premiums > 0 {
            (safety_pool.total_balance * 10000) / total_premiums         } else {
            0
        };

        (total_premiums, total_claims, net_balance, reserve_percentage)
    }

    pub fn check_reserve_health(env: &Env) -> (bool, i128, i128) {
        let safety_pool = Self::get_safety_pool(env);
        let config = Self::get_platform_config(env);
        
        let minimum_required = (safety_pool.premium_contributions * config.max_claim_amount_ratio as i128) / 10000;
        let is_healthy = safety_pool.total_balance >= minimum_required && 
                         safety_pool.total_balance >= safety_pool.minimum_reserve;

        (is_healthy, safety_pool.total_balance, minimum_required)
    }

    pub fn calculate_claim_capacity(env: &Env) -> i128 {
        let safety_pool = Self::get_safety_pool(env);
        let available_for_claims = safety_pool.total_balance - safety_pool.minimum_reserve;
        
        available_for_claims.max(0)
    }

    pub fn get_premium_payment_summary(env: &Env) -> (i128, u64, i128) {
        let payments = Self::get_premium_payments(env);
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

    pub fn get_recent_financial_activity(
        env: &Env, 
        days_back: u64
    ) -> (i128, i128, u64, u64) {
        let cutoff_time = env.ledger().timestamp() - (days_back * 24 * 60 * 60);
        let mut recent_premiums = 0i128;
        let mut recent_claims = 0i128;
        let mut premium_count = 0u64;
        let mut claim_count = 0u64;

                let payments = Self::get_premium_payments(env);
        for (_, payment_list) in payments.iter() {
            for payment in payment_list.iter() {
                if payment.payment_date >= cutoff_time {
                    recent_premiums += payment.amount;
                    premium_count += 1;
                }
            }
        }

                let claims = Self::get_claims(env);
        for (_, claim) in claims.iter() {
            if let Some(payout_date) = claim.payout_date {
                if payout_date >= cutoff_time {
                    recent_claims += claim.amount;
                    claim_count += 1;
                }
            }
        }

        (recent_premiums, recent_claims, premium_count, claim_count)
    }

        pub fn set_platform_config(
        env: &Env, 
        setter: Address, 
        config: PlatformConfig
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &setter)?;

        Self::save_platform_config(env, &config);
        Self::emit_event(env, PLATFORM_CONFIG_UPDATED, setter, ());
        
        Ok(true)
    }

        pub fn emergency_fund_freeze(
        env: &Env, 
        freezer: Address, 
        reason: String
    ) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &freezer)?;

        Self::set_fund_freeze_status(env, true);
        Self::emit_event(env, EMERGENCY_FUND_FREEZE, freezer, reason);
        
        Ok(true)
    }

    pub fn emergency_fund_unfreeze(env: &Env, unfreezer: Address) -> FinancialResult<bool> {
                Self::ensure_council_member(env, &unfreezer)?;

        Self::set_fund_freeze_status(env, false);
        Self::emit_event(env, EMERGENCY_FUND_UNFREEZE, unfreezer, ());
        
        Ok(true)
    }

    pub fn is_fund_frozen(env: &Env) -> bool {
        Self::get_fund_freeze_status(env)
    }

        fn get_safety_pool(env: &Env) -> SafetyPool {
        env.storage().instance().get(&SAFETY_POOL).unwrap_or_else(|| SafetyPool {
            total_balance: 0,
            premium_contributions: 0,
            claim_payouts: 0,
            investment_returns: 0,
            reserve_ratio: 7000,             last_audit_date: env.ledger().timestamp(),
            minimum_reserve: 10000,
        })
    }

    fn save_safety_pool(env: &Env, safety_pool: &SafetyPool) {
        env.storage().instance().set(&SAFETY_POOL, safety_pool);
    }

    fn get_premium_payments(env: &Env) -> Map<Address, Vec<Payment>> {
        env.storage().instance().get(&PREMIUM_PAYMENTS)
            .unwrap_or_else(|| Map::new(env))
    }

    fn get_claims(env: &Env) -> Map<u64, crate::state::Claim> {
        env.storage().instance().get(&CLAIMS)
            .unwrap_or_else(|| Map::new(env))
    }

    fn get_platform_config(env: &Env) -> PlatformConfig {
        env.storage().instance().get(&PLATFORM_CONFIG).unwrap_or_else(|| PlatformConfig {
            grace_period_weeks: 2,
            minimum_quorum: 3,
            proposal_duration_days: 7,
            max_claim_amount_ratio: 80,
            penalty_rate: 500,
            council_size: 5,
        })
    }

    fn save_platform_config(env: &Env, config: &PlatformConfig) {
        env.storage().instance().set(&PLATFORM_CONFIG, config);
    }

    fn get_fund_freeze_status(env: &Env) -> bool {
        env.storage().instance().get(&EMERGENCY_FUND_FREEZE).unwrap_or(false)
    }

    fn set_fund_freeze_status(env: &Env, status: bool) {
        env.storage().instance().set(&EMERGENCY_FUND_FREEZE, &status);
    }

        fn ensure_council_member(env: &Env, address: &Address) -> FinancialResult<()> {
        if !is_council_member(env, address) {
            return Err(FinancialManagementError::Unauthorized);
        }
        Ok(())
    }

    fn validate_positive_amount(amount: i128) -> FinancialResult<()> {
        if amount <= 0 {
            return Err(FinancialManagementError::InvalidAmount);
        }
        Ok(())
    }

    fn ensure_positive_amount(amount: i128) -> FinancialResult<()> {
        Self::validate_positive_amount(amount)
    }

    fn validate_percentage_ratio(ratio: u64) -> FinancialResult<()> {
        if ratio > 10000 {
            return Err(FinancialManagementError::InvalidRatio);
        }
        Ok(())
    }

    fn ensure_funds_not_frozen(env: &Env) -> FinancialResult<()> {
        if Self::get_fund_freeze_status(env) {
            return Err(FinancialManagementError::FundFrozen);
        }
        Ok(())
    }

    fn validate_reserve_withdrawal(
        env: &Env, 
        safety_pool: &SafetyPool, 
        amount: i128
    ) -> FinancialResult<()> {
        let reserve_after_withdrawal = safety_pool.total_balance - amount;
        
                if reserve_after_withdrawal < safety_pool.minimum_reserve {
            return Err(FinancialManagementError::InsufficientReserves);
        }

                let config = Self::get_platform_config(env);
        let minimum_reserve_amount = (safety_pool.premium_contributions * config.max_claim_amount_ratio as i128) / 10000;
        
        if reserve_after_withdrawal < minimum_reserve_amount {
            return Err(FinancialManagementError::InsufficientReserves);
        }

        Ok(())
    }

        fn emit_event<T>(env: &Env, event_symbol: Symbol, address: Address, data: T)
    where T: IntoVal<Env, soroban_sdk::Val> {
        env.events().publish((event_symbol, address), data);
    }

        fn calculate_financial_summary(env: &Env) -> (i128, i128, i128) {
        let payments = Self::get_premium_payments(env);
        let mut total_premiums = 0i128;
        
        for (_, payment_list) in payments.iter() {
            for payment in payment_list.iter() {
                total_premiums += payment.amount;
            }
        }

        let claims = Self::get_claims(env);
        let mut total_claims = 0i128;
        
        for (_, claim) in claims.iter() {
            if claim.status == crate::state::ClaimStatus::Paid {
                total_claims += claim.amount;
            }
        }

        let net_balance = total_premiums - total_claims;
        (total_premiums, total_claims, net_balance)
    }
}

pub fn get_safety_pool_balance(env: &Env) -> i128 {
    FinancialManagementService::get_safety_pool_balance(env)
}

pub fn get_safety_pool_details(env: &Env) -> SafetyPool {
    FinancialManagementService::get_safety_pool_details(env)
}

pub fn add_external_funding(env: &Env, funder: Address, amount: i128) -> bool {
    FinancialManagementService::add_external_funding(env, funder, amount).unwrap_or(false)
}

pub fn withdraw_reserve_funds(env: &Env, withdrawer: Address, amount: i128, purpose: String) -> bool {
    FinancialManagementService::withdraw_reserve_funds(env, withdrawer, amount, purpose).unwrap_or(false)
}

pub fn update_investment_returns(env: &Env, updater: Address, returns: i128) -> bool {
    FinancialManagementService::update_investment_returns(env, updater, returns).unwrap_or(false)
}

pub fn set_minimum_reserve(env: &Env, setter: Address, new_minimum: i128) -> bool {
    FinancialManagementService::set_minimum_reserve(env, setter, new_minimum).unwrap_or(false)
}

pub fn update_reserve_ratio(env: &Env, updater: Address, new_ratio: u64) -> bool {
    FinancialManagementService::update_reserve_ratio(env, updater, new_ratio).unwrap_or(false)
}

pub fn conduct_financial_audit(env: &Env, auditor: Address) -> bool {
    FinancialManagementService::conduct_financial_audit(env, auditor).unwrap_or(false)
}

pub fn get_financial_summary(env: &Env) -> (i128, i128, i128, i128) {
    FinancialManagementService::get_financial_summary(env)
}

pub fn check_reserve_health(env: &Env) -> (bool, i128, i128) {
    FinancialManagementService::check_reserve_health(env)
}

pub fn calculate_claim_capacity(env: &Env) -> i128 {
    FinancialManagementService::calculate_claim_capacity(env)
}

pub fn get_premium_payment_summary(env: &Env) -> (i128, u64, i128) {
    FinancialManagementService::get_premium_payment_summary(env)
}

pub fn get_recent_financial_activity(env: &Env, days_back: u64) -> (i128, i128, u64, u64) {
    FinancialManagementService::get_recent_financial_activity(env, days_back)
}

pub fn set_platform_config(env: &Env, setter: Address, config: PlatformConfig) -> bool {
    FinancialManagementService::set_platform_config(env, setter, config).unwrap_or(false)
}

pub fn emergency_fund_freeze(env: &Env, freezer: Address, reason: String) -> bool {
    FinancialManagementService::emergency_fund_freeze(env, freezer, reason).unwrap_or(false)
}

pub fn emergency_fund_unfreeze(env: &Env, unfreezer: Address) -> bool {
    FinancialManagementService::emergency_fund_unfreeze(env, unfreezer).unwrap_or(false)
}

pub fn is_fund_frozen(env: &Env) -> bool {
    FinancialManagementService::is_fund_frozen(env)
}
