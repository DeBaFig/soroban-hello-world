#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }

    pub fn get_current_value(env: Env) -> u32 {
        let count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "value: {}", count);

        count
    }

    pub fn decrement(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);
        if count == 0 {
            log!(&env, "value should no be less then 0", count);

            return count;
        }
        count -= 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }

    pub fn reset(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        if count == 0 {
            return count;
        }

        count = 0;
        
        log!(&env, "count: {}", count);
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }
}

mod test;