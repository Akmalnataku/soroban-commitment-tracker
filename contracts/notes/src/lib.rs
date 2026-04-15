#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Commitment {
    pub id: u64,
    pub title: String,
    pub is_completed: bool,
    pub deadline: u64,
}

const COMMITMENT_DATA: Symbol = symbol_short!("COMMIT");

#[contract]
pub struct CommitmentContract;

#[contractimpl]
impl CommitmentContract {

    pub fn get_commitments(env: Env) -> Vec<Commitment> {
        env.storage().instance().get(&COMMITMENT_DATA).unwrap_or(Vec::new(&env))
    }

    pub fn create_commitment(env: Env, title: String, deadline: u64) -> String {
        let mut commitments: Vec<Commitment> =
            env.storage().instance().get(&COMMITMENT_DATA).unwrap_or(Vec::new(&env));

        let current_time = env.ledger().timestamp();

        if deadline <= current_time {
            return String::from_str(&env, "Deadline must be in the future");
        }

        let commitment = Commitment {
            id: env.prng().gen(),
            title,
            is_completed: false,
            deadline,
        };

        commitments.push_back(commitment);
        env.storage().instance().set(&COMMITMENT_DATA, &commitments);

        String::from_str(&env, "Commitment created")
    }

    pub fn complete_commitment(env: Env, id: u64) -> String {
        let mut commitments: Vec<Commitment> =
            env.storage().instance().get(&COMMITMENT_DATA).unwrap_or(Vec::new(&env));

        for i in 0..commitments.len() {
            if let Some(mut c) = commitments.get(i) {
                if c.id == id {

                    if c.is_completed {
                        return String::from_str(&env, "Already completed");
                    }

                    c.is_completed = true;
                    commitments.set(i, c);
                    env.storage().instance().set(&COMMITMENT_DATA, &commitments);

                    return String::from_str(&env, "Marked as completed");
                }
            }
        }

        String::from_str(&env, "Commitment not found")
    }

    pub fn delete_commitment(env: Env, id: u64) -> String {
        let mut commitments: Vec<Commitment> =
            env.storage().instance().get(&COMMITMENT_DATA).unwrap_or(Vec::new(&env));

        let current_time = env.ledger().timestamp();

        for i in 0..commitments.len() {
            if let Some(c) = commitments.get(i) {
                if c.id == id {

                    if current_time < c.deadline {
                        return String::from_str(&env, "Cannot delete before deadline");
                    }

                    commitments.remove(i);
                    env.storage().instance().set(&COMMITMENT_DATA, &commitments);

                    return String::from_str(&env, "Deleted");
                }
            }
        }

        String::from_str(&env, "Commitment not found")
    }
}