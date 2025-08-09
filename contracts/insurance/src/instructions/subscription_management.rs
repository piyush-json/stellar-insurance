use soroban_sdk::{Env, Address, Map, Vec};
use crate::state::{Subscription, SubscriptionStatus, Payment, User, PlatformConfig};
use crate::instructions::user_management::{is_user_approved};
use crate::instructions::plan_management::{get_plan, is_plan_available_for_subscription, increment_plan_subscriber_count, decrement_plan_subscriber_count};
use crate::constant::*;

pub fn subscribe_to_plan(env: &Env, user: Address, plan_id: u64) -> bool {
    // Check if user is approved
    if !is_user_approved(env, &user) {
        return false;
    }

    // Check if plan is available for subscription
    if !is_plan_available_for_subscription(env, plan_id) {
        return false;
    }

    // Check if user already has an active subscription
    if has_active_subscription(env, &user) {
        return false;
    }

    let mut subscriptions: Map<u64, Subscription> = env.storage().instance().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let subscription_id = subscriptions.len() as u64 + 1;

    let new_subscription = Subscription {
        id: subscription_id,
        user: user.clone(),
        plan_id,
        start_date: env.ledger().timestamp(),
        status: SubscriptionStatus::Active,
        weeks_paid: 0,
        weeks_due: 1, // Start with 1 week due
        last_payment_date: 0,
        grace_period_end: 0,
        total_premiums_paid: 0,
    };

    subscriptions.set(subscription_id, new_subscription);
    env.storage().instance().set(&SUBSCRIPTIONS, &subscriptions);

    // Update user's subscribed plan
    let mut users: Map<Address, User> = env.storage().instance().get(&USERS).unwrap_or_else(|| Map::new(env));
    if let Some(mut user_data) = users.get(user.clone()) {
        user_data.subscribed_plan = Some(plan_id);
        users.set(user.clone(), user_data);
        env.storage().instance().set(&USERS, &users);
    }

    // Increment plan subscriber count
    increment_plan_subscriber_count(env, plan_id);

    env.events().publish((PLAN_SUBSCRIBED, user.clone()), plan_id);
    true
}

pub fn pay_premium(env: &Env, user: Address) -> bool {
    let subscription = match get_active_subscription_for_user(env, &user) {
        Some(sub) => sub,
        None => return false,
    };

    let plan = match get_plan(env, subscription.plan_id) {
        Some(p) => p,
        None => return false,
    };

    let mut subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let mut subscription = subscription;

    // Calculate penalty if payment is late
    let mut penalty = 0i128;
    let current_time = env.ledger().timestamp();
    let config = get_platform_config(env);
    
    if subscription.weeks_due > subscription.weeks_paid + 1 {
        penalty = (plan.weekly_premium * config.penalty_rate as i128) / 10000; // Basis points
    }

    let total_payment = plan.weekly_premium + penalty;

    // Update subscription
    subscription.weeks_paid += 1;
    subscription.last_payment_date = current_time;
    subscription.total_premiums_paid += total_payment;
    
    // Reset grace period if in grace period
    if subscription.status == SubscriptionStatus::GracePeriod {
        subscription.status = SubscriptionStatus::Active;
        subscription.grace_period_end = 0;
    }

    record_payment(env, user.clone(), subscription.plan_id, total_payment, subscription.weeks_paid, penalty);

    subscriptions.set(subscription.id, subscription);
    env.storage().persistent().set(&SUBSCRIPTIONS, &subscriptions);

    // Update safety pool
    update_safety_pool_with_premium(env, total_payment);

    env.events().publish((PREMIUM_PAID, user), total_payment);
    true
}

pub fn cancel_subscription(env: &Env, user: Address) -> bool {
    let subscription = match get_active_subscription_for_user(env, &user) {
        Some(sub) => sub,
        None => return false,
    };

    let mut subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let mut subscription_mut = subscription.clone();

    subscription_mut.status = SubscriptionStatus::Cancelled;
    subscriptions.set(subscription_mut.id, subscription_mut.clone());
    env.storage().persistent().set(&SUBSCRIPTIONS, &subscriptions);

    // Update user's subscribed plan
    let mut users: Map<Address, User> = env.storage().persistent().get(&USERS).unwrap_or_else(|| Map::new(env));
    if let Some(mut user_data) = users.get(user.clone()) {
        user_data.subscribed_plan = None;
        users.set(user.clone(), user_data);
        env.storage().persistent().set(&USERS, &users);
    }

    // Decrement plan subscriber count
    decrement_plan_subscriber_count(env, subscription_mut.plan_id);

    env.events().publish((SUBSCRIPTION_CANCELLED, user), subscription_mut.plan_id);
    true
}

pub fn reactivate_subscription(env: &Env, user: Address) -> bool {
    // Check if user is approved
    if !is_user_approved(env, &user) {
        return false;
    }

    let subscription = match get_subscription_for_user(env, &user) {
        Some(sub) => sub,
        None => return false,
    };

    // Can only reactivate suspended subscriptions
    if subscription.status != SubscriptionStatus::Suspended {
        return false;
    }

    let plan = match get_plan(env, subscription.plan_id) {
        Some(p) => p,
        None => return false,
    };

    let mut subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let mut subscription_mut = subscription.clone();

    // Calculate outstanding payments
    let weeks_outstanding = subscription_mut.weeks_due.saturating_sub(subscription_mut.weeks_paid);
    let config = get_platform_config(env);
    let penalty_rate = config.penalty_rate as i128;

    let outstanding_amount = weeks_outstanding as i128 * plan.weekly_premium;
    let penalty = (outstanding_amount * penalty_rate) / 10000;
    let total_reactivation_cost = outstanding_amount + penalty;

    // For now, we'll assume payment is made (in real implementation, this would be integrated with payment)
    subscription_mut.status = SubscriptionStatus::Active;
    subscription_mut.weeks_paid = subscription_mut.weeks_due;
    subscription_mut.last_payment_date = env.ledger().timestamp();
    subscription_mut.total_premiums_paid += total_reactivation_cost;
    subscription_mut.grace_period_end = 0;

    subscriptions.set(subscription_mut.id, subscription_mut.clone());
    env.storage().persistent().set(&SUBSCRIPTIONS, &subscriptions);

    // Update user's subscribed plan
    let mut users: Map<Address, User> = env.storage().persistent().get(&USERS).unwrap_or_else(|| Map::new(env));
    if let Some(mut user_data) = users.get(user.clone()) {
        user_data.subscribed_plan = Some(subscription_mut.plan_id);
        users.set(user.clone(), user_data);
        env.storage().persistent().set(&USERS, &users);
    }

    // Increment plan subscriber count
    increment_plan_subscriber_count(env, subscription_mut.plan_id);

    // Record reactivation payment
    record_payment(env, user.clone(), subscription_mut.plan_id, total_reactivation_cost, subscription_mut.weeks_paid, penalty);

    // Update safety pool
    update_safety_pool_with_premium(env, total_reactivation_cost);

    env.events().publish((SUBSCRIPTION_REACTIVATED, user), total_reactivation_cost);
    true
}

pub fn update_subscription_status(env: &Env) {
    // This function should be called periodically to update subscription statuses
    let mut subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    let current_time = env.ledger().timestamp();
    let config = get_platform_config(env);
    let grace_period_seconds = config.grace_period_weeks * 7 * 24 * 60 * 60;

    let mut any_updated = false;

    for (sub_id, mut subscription) in subscriptions.iter() {
        let mut updated = false;
        if subscription.status == SubscriptionStatus::Active {
            // Check if payment is overdue
            let weeks_since_start = (current_time - subscription.start_date) / (7 * 24 * 60 * 60);
            subscription.weeks_due = weeks_since_start + 1;

            if subscription.weeks_due > subscription.weeks_paid {
                // Payment is overdue, start grace period
                subscription.status = SubscriptionStatus::GracePeriod;
                subscription.grace_period_end = current_time + grace_period_seconds;
                updated = true;

                env.events().publish((SUBSCRIPTION_GRACE_PERIOD, subscription.user.clone()), sub_id);
            }
        } else if subscription.status == SubscriptionStatus::GracePeriod {
            // Check if grace period has expired
            if current_time > subscription.grace_period_end {
                subscription.status = SubscriptionStatus::Suspended;
                updated = true;

                env.events().publish((SUBSCRIPTION_SUSPENDED, subscription.user.clone()), sub_id);
            }
        }

        if updated {
            subscriptions.set(sub_id, subscription);
            any_updated = true;
        }
    }

    if any_updated {
        env.storage().persistent().set(&SUBSCRIPTIONS, &subscriptions);
    }
}

pub fn get_subscription_for_user(env: &Env, user: &Address) -> Option<Subscription> {
    let subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    
    for (_, subscription) in subscriptions.iter() {
        if subscription.user == user.clone() {
            return Some(subscription);
        }
    }
    None
}

pub fn get_active_subscription_for_user(env: &Env, user: &Address) -> Option<Subscription> {
    if let Some(subscription) = get_subscription_for_user(env, user) {
        match subscription.status {
            SubscriptionStatus::Active | SubscriptionStatus::GracePeriod => Some(subscription),
            _ => None,
        }
    } else {
        None
    }
}

pub fn has_active_subscription(env: &Env, user: &Address) -> bool {
    get_active_subscription_for_user(env, user).is_some()
}

pub fn get_subscription_by_id(env: &Env, subscription_id: u64) -> Option<Subscription> {
    let subscriptions: Map<u64, Subscription> = env.storage().persistent().get(&SUBSCRIPTIONS).unwrap_or_else(|| Map::new(env));
    subscriptions.get(subscription_id)
}

fn record_payment(env: &Env, user: Address, plan_id: u64, amount: i128, week_number: u64, penalty: i128) {
    let mut payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PAYMENTS).unwrap_or_else(|| Map::new(env));
    let mut user_payments = payments.get(user.clone()).unwrap_or(Vec::new(env));

    let payment = Payment {
        user: user.clone(),
        plan_id,
        amount,
        week_number,
        payment_date: env.ledger().timestamp(),
        penalty_applied: penalty,
    };

    user_payments.push_back(payment);
    payments.set(user, user_payments);
    env.storage().persistent().set(&PAYMENTS, &payments);
}

fn update_safety_pool_with_premium(env: &Env, premium_amount: i128) {
    use crate::state::SafetyPool;
    
    let mut safety_pool: SafetyPool = env.storage().persistent().get(&SAFETY_POOL).unwrap_or(SafetyPool {
        total_balance: 0,
        premium_contributions: 0,
        claim_payouts: 0,
        investment_returns: 0,
        reserve_ratio: DEFAULT_RESERVE_RATIO,
        last_audit_date: env.ledger().timestamp(),
        minimum_reserve: DEFAULT_MINIMUM_RESERVE,
    });

    safety_pool.total_balance += premium_amount;
    safety_pool.premium_contributions += premium_amount;

    env.storage().persistent().set(&SAFETY_POOL, &safety_pool);
}

fn get_platform_config(env: &Env) -> PlatformConfig {
    env.storage().persistent().get(&PLATFORM_CONFIG).unwrap_or(PlatformConfig {
        grace_period_weeks: DEFAULT_GRACE_PERIOD_WEEKS,
        minimum_quorum: DEFAULT_MINIMUM_QUORUM,
        proposal_duration_days: DEFAULT_PROPOSAL_DURATION_DAYS,
        max_claim_amount_ratio: DEFAULT_MAX_CLAIM_AMOUNT_RATIO,
        penalty_rate: DEFAULT_PENALTY_RATE,
        council_size: DEFAULT_COUNCIL_SIZE,
    })
}

pub fn get_user_payment_history(env: &Env, user: &Address) -> Vec<Payment> {
    let payments: Map<Address, Vec<Payment>> = env.storage().persistent().get(&PAYMENTS).unwrap_or_else(|| Map::new(env));
    payments.get(user.clone()).unwrap_or(Vec::new(env))
}

pub fn calculate_outstanding_premium(env: &Env, user: &Address) -> i128 {
    if let Some(subscription) = get_active_subscription_for_user(env, user) {
        if let Some(plan) = get_plan(env, subscription.plan_id) {
            let weeks_outstanding = subscription.weeks_due.saturating_sub(subscription.weeks_paid);
            return weeks_outstanding as i128 * plan.weekly_premium;
        }
    }
    0
}
