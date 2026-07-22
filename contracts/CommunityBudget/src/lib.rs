#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env};

/// Storage keys. Each recipient has one claimable-balance entry.
#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Balance(Address), // recipient -> claimable USDC amount (in stroops-equivalent units)
}

#[contract]
pub struct RemitNow;

#[contractimpl]
impl RemitNow {
    /// Sender pushes a remittance to a recipient. In production this would be
    /// paired with an actual USDC token transfer into the contract; here we
    /// record the claimable balance directly to keep the MVP demo-able.
    pub fn send_remittance(env: Env, sender: Address, recipient: Address, amount: i128) {
        // Only the sender can authorize moving their own funds.
        sender.require_auth();
        assert!(amount > 0, "amount must be positive");

        let key = DataKey::Balance(recipient.clone());
        let current: i128 = env.storage().instance().get(&key).unwrap_or(0);
        env.storage().instance().set(&key, &(current + amount));

        // Emit an event so wallets/explorers can show the transfer.
        env.events()
            .publish((symbol_short!("sent"), sender, recipient), amount);
    }

    /// Recipient claims (withdraws) their full accumulated remittance balance.
    pub fn claim_remittance(env: Env, recipient: Address) -> i128 {
        // Only the recipient can claim their own funds.
        recipient.require_auth();

        let key = DataKey::Balance(recipient.clone());
        let balance: i128 = env.storage().instance().get(&key).unwrap_or(0);
        assert!(balance > 0, "no funds to claim");

        env.storage().instance().set(&key, &0i128);
        env.events()
            .publish((symbol_short!("claimed"),), (recipient, balance));

        balance
    }

    /// Read-only view of a recipient's current claimable balance.
    pub fn get_balance(env: Env, recipient: Address) -> i128 {
        let key = DataKey::Balance(recipient);
        env.storage().instance().get(&key).unwrap_or(0)
    }
}

mod test;