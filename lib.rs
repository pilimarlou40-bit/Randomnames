#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol, symbol_short, log,
};

#[contracttype]
#[derive(Clone)]
pub struct Slot {
    pub user: Address,
    pub expiry: u64,
}

#[contracttype]
pub enum DataKey {
    Slot(u32), // slot_id → Slot
}

#[contract]
pub struct ParkLock;

#[contractimpl]
impl ParkLock {

    // Reserve a parking slot
    pub fn reserve_slot(env: Env, slot_id: u32, user: Address, duration: u64) {
        let key = DataKey::Slot(slot_id);

        // Check if slot exists and not expired
        if let Some(existing) = env.storage().persistent().get::<_, Slot>(&key) {
            let current_time = env.ledger().timestamp();
            if current_time < existing.expiry {
                panic!("Slot already reserved");
            }
        }

        let expiry = env.ledger().timestamp() + duration;

        let slot = Slot {
            user: user.clone(),
            expiry,
        };

        env.storage().persistent().set(&key, &slot);

        // Emit reservation event
        env.events().publish(
            (symbol_short!("RESERVE"),),
            (slot_id, user.clone(), expiry),
        );

        log!(&env, "Slot {} reserved by {:?}", slot_id, user);
    }

    // Check if slot is available
    pub fn is_available(env: Env, slot_id: u32) -> bool {
        let key = DataKey::Slot(slot_id);

        if let Some(slot) = env.storage().persistent().get::<_, Slot>(&key) {
            let current_time = env.ledger().timestamp();
            return current_time >= slot.expiry;
        }

        true
    }

    // Get slot info
    pub fn get_slot(env: Env, slot_id: u32) -> Option<Slot> {
        let key = DataKey::Slot(slot_id);
        env.storage().persistent().get(&key)
    }

    // Manually release slot (optional)
    pub fn release_slot(env: Env, slot_id: u32, user: Address) {
        let key = DataKey::Slot(slot_id);

        let slot: Slot = env.storage().persistent().get(&key).unwrap();

        if slot.user != user {
            panic!("Not slot owner");
        }

        env.storage().persistent().remove(&key);

        env.events().publish(
            (symbol_short!("RELEASE"),),
            slot_id,
        );
    }
}
