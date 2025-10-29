#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Address, Symbol, symbol_short};

// Structure to store advertisement details
#[contracttype]
#[derive(Clone)]
pub struct Advertisement {
    pub ad_id: u64,
    pub advertiser: Address,
    pub reward_per_view: u64,
    pub total_budget: u64,
    pub remaining_budget: u64,
    pub total_views: u64,
    pub is_active: bool,
}

// Structure to track user rewards
#[contracttype]
#[derive(Clone)]
pub struct UserRewards {
    pub user: Address,
    pub total_earned: u64,
    pub ads_viewed: u64,
}

// Mapping for advertisements
#[contracttype]
pub enum AdBook {
    Ad(u64)
}

// Mapping for user rewards
#[contracttype]
pub enum RewardBook {
    User(Address)
}

// Counter for ad IDs
const AD_COUNTER: Symbol = symbol_short!("AD_COUNT");

#[contract]
pub struct DecentralizedAdContract;

#[contractimpl]
impl DecentralizedAdContract {

    // Function to create a new advertisement
    pub fn create_ad(env: Env, advertiser: Address, reward_per_view: u64, total_budget: u64) -> u64 {
        advertiser.require_auth();
        
        let mut ad_count: u64 = env.storage().instance().get(&AD_COUNTER).unwrap_or(0);
        ad_count += 1;

        let new_ad = Advertisement {
            ad_id: ad_count,
            advertiser: advertiser.clone(),
            reward_per_view,
            total_budget,
            remaining_budget: total_budget,
            total_views: 0,
            is_active: true,
        };

        env.storage().instance().set(&AdBook::Ad(ad_count), &new_ad);
        env.storage().instance().set(&AD_COUNTER, &ad_count);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Advertisement created with ID: {}", ad_count);
        ad_count
    }

    // Function for users to view an ad and earn rewards
    pub fn view_ad(env: Env, ad_id: u64, viewer: Address) {
        viewer.require_auth();

        let mut ad = Self::get_ad(env.clone(), ad_id);
        
        if !ad.is_active {
            log!(&env, "Advertisement is not active");
            panic!("Advertisement is not active");
        }

        if ad.remaining_budget < ad.reward_per_view {
            ad.is_active = false;
            env.storage().instance().set(&AdBook::Ad(ad_id), &ad);
            log!(&env, "Advertisement budget exhausted");
            panic!("Advertisement budget exhausted");
        }

        // Update advertisement stats
        ad.total_views += 1;
        ad.remaining_budget -= ad.reward_per_view;

        if ad.remaining_budget < ad.reward_per_view {
            ad.is_active = false;
        }

        env.storage().instance().set(&AdBook::Ad(ad_id), &ad);

        // Update user rewards
        let mut user_rewards = Self::get_user_rewards(env.clone(), viewer.clone());
        user_rewards.total_earned += ad.reward_per_view;
        user_rewards.ads_viewed += 1;

        env.storage().instance().set(&RewardBook::User(viewer.clone()), &user_rewards);
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "User {} viewed ad {} and earned {} tokens", viewer, ad_id, ad.reward_per_view);
    }

    // Function to get advertisement details
    pub fn get_ad(env: Env, ad_id: u64) -> Advertisement {
        env.storage().instance().get(&AdBook::Ad(ad_id)).unwrap_or(Advertisement {
            ad_id: 0,
            advertiser: Address::from_string(&soroban_sdk::String::from_str(&env, "GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF")),
            reward_per_view: 0,
            total_budget: 0,
            remaining_budget: 0,
            total_views: 0,
            is_active: false,
        })
    }

    // Function to get user rewards
    pub fn get_user_rewards(env: Env, user: Address) -> UserRewards {
        env.storage().instance().get(&RewardBook::User(user.clone())).unwrap_or(UserRewards {
            user: user.clone(),
            total_earned: 0,
            ads_viewed: 0,
        })
    }
}
