#![cfg(test)]
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}

#[test]
fn test2() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    let times = 5;

    for _ in 0..times {
        client.increment();
    }

    assert_eq!(client.get_current_value(), times);
}

#[test]
fn test3() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    let times = 5;

    for _ in 0..times {
        client.increment();
    }
    for _ in 0..times {
        client.decrement();
    }
    assert_eq!(client.decrement(), 0);
}

#[test]
fn test4() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    let times = 5;

    for _ in 0..times {
        client.decrement();
    }
    assert_eq!(client.decrement(), 0);
}

#[test]
fn test5() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);
    let times = 5;

    for _ in 0..times {
        client.increment();
    }

    assert_eq!(client.reset(), 0);
}