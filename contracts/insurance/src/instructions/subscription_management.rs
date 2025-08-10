use soroban_sdk::{Env, Address,  Vec};
use crate::constant::{EVENT_SUB_CRT, EVENT_SUB_PAY, EVENT_SUB_SUS};
use crate::state::{DataKey, Subscription, SubscriptionStatus, Policy};
use crate::instructions::user_management::is_user_approved;

#[derive(Debug)]
pub enum SubscriptionManagementError {
    SubscriptionNotFound,
    PolicyNotFound,
    Unauthorized,
    InvalidSubscriptionData,
    InsufficientFunds,
    SubscriptionNotActive,
    PaymentFailed,
    InvalidPolicy,
}

pub fn create_subscription(
    env: &Env,
    subscriber: Address,
    policy_id: u64,
    start_date: u64,
    premium_amount: i128,
) -> Result<u64, SubscriptionManagementError> {
    
    if !is_user_approved(env, &subscriber) {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    let policy = env.storage().instance().get::<_, Policy>(&DataKey::Policy(policy_id))
        .ok_or(SubscriptionManagementError::InvalidPolicy)?;

    if policy.status != crate::state::PolicyStatus::Active {
        return Err(SubscriptionManagementError::InvalidPolicy);
    }

    
    if start_date < env.ledger().timestamp() {
        return Err(SubscriptionManagementError::InvalidSubscriptionData);
    }

    
    if premium_amount <= 0 {
        return Err(SubscriptionManagementError::InvalidSubscriptionData);
    }

    let subscription_id = env.ledger().sequence() as u64;
    let subscription = Subscription {
        id: subscription_id,
        policy_id,
        subscriber: subscriber.clone(),
        start_date,
        status: SubscriptionStatus::Active,
        last_payment_date: 0,
        next_payment_due: start_date + (7 * 24 * 60 * 60), 
        weeks_paid: 0,
        weeks_due: 1,
        grace_period_end: start_date + (14 * 24 * 60 * 60), 
        total_premiums_paid: 0,
    };

    env.storage().instance().set(&DataKey::Subscription(subscription_id), &subscription);
    
    env.events().publish(
        (EVENT_SUB_CRT, subscription_id),
        (subscriber, policy_id, premium_amount)
    );

    Ok(subscription_id)
}

pub fn update_subscription(
    env: &Env,
    subscription_id: u64,
    updater: Address,
    new_start_date: Option<u64>,
) -> Result<bool, SubscriptionManagementError> {
    let subscription_key = DataKey::Subscription(subscription_id);
    let mut subscription = env.storage().instance().get::<_, Subscription>(&subscription_key)
        .ok_or(SubscriptionManagementError::SubscriptionNotFound)?;

    
    if subscription.subscriber != updater && !is_user_approved(env, &updater) {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    if subscription.status != SubscriptionStatus::Active {
        return Err(SubscriptionManagementError::SubscriptionNotActive);
    }

    
    if let Some(date) = new_start_date {
        if date > env.ledger().timestamp() {
            subscription.start_date = date;
            subscription.next_payment_due = date + (7 * 24 * 60 * 60);
            subscription.grace_period_end = date + (14 * 24 * 60 * 60);
        }
    }

    env.storage().instance().set(&subscription_key, &subscription);

    env.events().publish(
        (EVENT_SUB_CRT, subscription_id),
        (updater, "subscription updated")
    );

    Ok(true)
}

pub fn cancel_subscription(
    env: &Env,
    subscription_id: u64,
    canceller: Address,
) -> Result<bool, SubscriptionManagementError> {
    let subscription_key = DataKey::Subscription(subscription_id);
    let mut subscription = env.storage().instance().get::<_, Subscription>(&subscription_key)
        .ok_or(SubscriptionManagementError::SubscriptionNotFound)?;

    
    if subscription.subscriber != canceller && !is_user_approved(env, &canceller) {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    if subscription.status != SubscriptionStatus::Active {
        return Err(SubscriptionManagementError::SubscriptionNotActive);
    }

    subscription.status = SubscriptionStatus::Cancelled;
    env.storage().instance().set(&subscription_key, &subscription);

    env.events().publish(
        (EVENT_SUB_SUS, subscription_id),
        (canceller, "subscription cancelled")
    );

    Ok(true)
}

pub fn process_payment(
    env: &Env,
    subscription_id: u64,
    payer: Address,
    amount: i128,
) -> Result<bool, SubscriptionManagementError> {
    let subscription_key = DataKey::Subscription(subscription_id);
    let mut subscription = env.storage().instance().get::<_, Subscription>(&subscription_key)
        .ok_or(SubscriptionManagementError::SubscriptionNotFound)?;

    
    if subscription.subscriber != payer {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    if subscription.status != SubscriptionStatus::Active {
        return Err(SubscriptionManagementError::SubscriptionNotActive);
    }

    
    subscription.last_payment_date = env.ledger().timestamp();
    subscription.weeks_paid += 1;
    subscription.total_premiums_paid += amount;
    
    
    subscription.next_payment_due = subscription.next_payment_due + (7 * 24 * 60 * 60);

    env.storage().instance().set(&subscription_key, &subscription);

    env.events().publish(
        (EVENT_SUB_PAY, subscription_id),
        (payer, amount, "payment processed")
    );

    Ok(true)
}

pub fn renew_subscription(
    env: &Env,
    subscription_id: u64,
    renewer: Address,
    new_start_date: u64,
) -> Result<bool, SubscriptionManagementError> {
    let subscription_key = DataKey::Subscription(subscription_id);
    let mut subscription = env.storage().instance().get::<_, Subscription>(&subscription_key)
        .ok_or(SubscriptionManagementError::SubscriptionNotFound)?;

    
    if subscription.subscriber != renewer {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    if subscription.status != SubscriptionStatus::Active {
        return Err(SubscriptionManagementError::SubscriptionNotActive);
    }

    
    subscription.start_date = new_start_date;
    subscription.next_payment_due = new_start_date + (7 * 24 * 60 * 60);
    subscription.grace_period_end = new_start_date + (14 * 24 * 60 * 60);
    
    env.storage().instance().set(&subscription_key, &subscription);

    env.events().publish(
        (EVENT_SUB_CRT, subscription_id),
        (renewer, "subscription renewed")
    );

    Ok(true)
}

pub fn get_subscription(env: &Env, subscription_id: u64) -> Result<Subscription, SubscriptionManagementError> {
    env.storage().instance().get(&DataKey::Subscription(subscription_id))
        .ok_or(SubscriptionManagementError::SubscriptionNotFound)
}

pub fn get_user_subscriptions(env: &Env, user: Address) -> Result<Vec<u64>, SubscriptionManagementError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    //TODO: Implement this
    Ok(Vec::new(&env))
}

pub fn get_active_subscriptions(env: &Env) -> Result<Vec<u64>, SubscriptionManagementError> {
    // This is a simplified implementation - in a real contract you might want to maintain an index
    // For now, we'll return an empty vector as this would require more complex storage patterns
    //TODO: Implement this
    Ok(Vec::new(&env))
}

pub fn calculate_premium(
    env: &Env,
    policy_id: u64,
    coverage_amount: i128,
    duration_days: u32,
    risk_score: u32,
) -> i128 {
    let policy = match env.storage().instance().get::<_, Policy>(&DataKey::Policy(policy_id)) {
        Some(p) => p,
        None => return 0,
    };

    let base_rate = policy.params.interest_rate;
    let risk_multiplier = 100 + (risk_score as i128 * 2); 
    let duration_multiplier = (duration_days as i128 * 100) / 365; 

    (coverage_amount * base_rate as i128 * risk_multiplier * duration_multiplier) / (100 * 100 * 100)
}

pub fn validate_subscription_eligibility(
    env: &Env,
    user: Address,
    policy_id: u64,
) -> Result<bool, SubscriptionManagementError> {
    
    if !is_user_approved(env, &user) {
        return Err(SubscriptionManagementError::Unauthorized);
    }

    
    let policy = env.storage().instance().get::<_, Policy>(&DataKey::Policy(policy_id))
        .ok_or(SubscriptionManagementError::InvalidPolicy)?;

    if policy.status != crate::state::PolicyStatus::Active {
        return Err(SubscriptionManagementError::InvalidPolicy);
    }

    // Check if user already has an active subscription for this policy
    //TODO: Implement this  
    // This would require maintaining an index in a real implementation
    
    Ok(true)
}


pub fn is_in_grace_period(env: &Env, subscription: &Subscription) -> bool {
    env.ledger().timestamp() <= subscription.grace_period_end
}


pub fn can_renew_subscription(env: &Env, subscription: &Subscription) -> bool {
    subscription.status == SubscriptionStatus::Active && 
    env.ledger().timestamp() >= subscription.start_date - (30 * 24 * 60 * 60) 
}


pub fn is_payment_overdue(env: &Env, subscription: &Subscription) -> bool {
    env.ledger().timestamp() > subscription.next_payment_due + (7 * 24 * 60 * 60) 
}
