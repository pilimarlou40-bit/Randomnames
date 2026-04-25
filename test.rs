#![cfg(test)]

use soroban_sdk::{Env, Address};
use crate::{ParkLockClient};

#[test]
fn test_reserve_slot_success() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::ParkLock);
    let client = ParkLockClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.reserve_slot(&1, &user, &1000);

    let available = client.is_available(&1);
    assert_eq!(available, false);
}

#[test]
#[should_panic(expected = "Slot already reserved")]
fn test_double_reserve_fail() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::ParkLock);
    let client = ParkLockClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.reserve_slot(&1, &user, &1000);
    client.reserve_slot(&1, &user, &1000); // should fail
}

#[test]
fn test_release_slot() {
    let env = Env::default();
    let contract_id = env.register_contract(None, crate::ParkLock);
    let client = ParkLockClient::new(&env, &contract_id);

    let user = Address::generate(&env);

    client.reserve_slot(&1, &user, &1000);
    client.release_slot(&1, &user);

    let available = client.is_available(&1);
    assert_eq!(available, true);
}
