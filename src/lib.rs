// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::serde_json::{from_str};
use near_sdk::json_types::U128;
use near_sdk::collections::{Vector, UnorderedSet};
use near_sdk::{log,Timestamp, near_bindgen, env, Gas, AccountId, PanicOnDefault, PromiseOrValue, Balance, serde_json::json};

pub use crate::external::*;
pub use crate::migrate::*;
pub use crate::governance::*;
pub use crate::views::*;

// Define modules
pub mod external;
mod migrate;
mod governance;
mod views;


// Define global variables

const BASE_GAS: u64 = 5_000_000_000_000;
const PROMISE_CALL: u64 = 5_000_000_000_000;
const GAS_FOR_FT_ON_TRANSFER: Gas = Gas(BASE_GAS + PROMISE_CALL);
const TGAS: u64 = 10_000_000_000_000;

// nanoseconds in a second
const NANOSECONDS: u64 = 1_000_000_000;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[cfg_attr(not(target_arch = "wasm32"), derive(Clone, Debug))]
#[serde(crate = "near_sdk::serde")]
pub struct DepositInfo {
    pub account_id: AccountId,
    pub date: Timestamp,
    pub ft_amount: String,
    pub deposit_or_withdraw: bool, //true=deposit - withdraw=false
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct VaultInfo {
    winner: AccountId,
    token_amount: Balance,
    token_amount_complete: Balance,
    date_start: Timestamp,
    date_end: Timestamp,
    claimed: bool,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]pub struct VaultWithIndex {
    index: u64,
    vault_info: VaultInfo,
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct OldContract {
    time_last_deposit: Timestamp, // Fecha ultimo deposito
    countdown_period: Timestamp, // Tiempo activo de boveda
    countdown_period_withdraw: Timestamp, // Tiempo para abrir la boveda    
    account_last_deposit: AccountId, // Cuenta que hizo el ultimo deposito
    ft_token_balance: Balance, // Balance de ft en boveda
    ft_token_id: AccountId, // Token id
    treasury_id: AccountId, // Account id tesoreria
    owner_id: AccountId, // Owner del contrato
    highest_deposit: Balance, // Highest amount somebody had deposit in the contract
    highest_withdraw: Balance, // Highest withdraw somebode had done when winning.
    deposit_history: UnorderedSet<DepositInfo>,
    vaults: Vector<VaultInfo>, // Vector para almacenar las bóvedas
    treasury_fee: u128
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    time_last_deposit: Timestamp, // Fecha ultimo deposito
    countdown_period: Timestamp, // Tiempo activo de boveda
    countdown_period_withdraw: Timestamp, // Tiempo para abrir la boveda
    account_last_deposit: AccountId, // Cuenta que hizo el ultimo deposito
    ft_token_balance: Balance, // Balance de ft en boveda
    ft_token_id: AccountId, // Token id
    treasury_id: AccountId, // Account id tesoreria
    owner_id: AccountId, // Owner del contrato
    highest_deposit: Balance, // Highest amount somebody had deposit in the contract
    highest_withdraw: Balance, // Highest withdraw somebode had done when winning.
    deposit_history: UnorderedSet<DepositInfo>,
    vaults: Vector<VaultInfo>, // Vector para almacenar las bóvedas,
    treasury_fee: u128
}


/// This is format of output via JSON for the auction message.
#[derive( Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct MsgInput {
    pub action_to_execute: String,
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(ft_token_id: AccountId, countdown_period_withdraw: Timestamp, owner_id: AccountId, treasury_id: AccountId, treasury_fee: u128) -> Self {
        assert!(!env::state_exists(), "Already initialized");
        let this = Self {
            time_last_deposit: 0,
            countdown_period: 0,
            countdown_period_withdraw: countdown_period_withdraw,
            account_last_deposit : owner_id.clone(),
            ft_token_balance: 0,
            ft_token_id: ft_token_id,
            treasury_id: treasury_id,
            owner_id: owner_id,
            highest_deposit:0,
            highest_withdraw:0, 
            deposit_history:UnorderedSet::new(b"d".to_vec()),
            vaults: Vector::new(b"v".to_vec()), // Inicializar el vector de bóvedas,
            treasury_fee: treasury_fee
        };
        this
    }

    pub fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        let msg_json: MsgInput = from_str(&msg).unwrap();
        let deposit = amount.0;
        let user_account = env::signer_account_id();

        log!("Deposit: {:?}",amount);
        
        env::log(
            json!(msg_json.clone())
            .to_string()
            .as_bytes(),
        );

        match msg_json.action_to_execute.as_str() {
            "increase_deposit" => {

                assert!(deposit % 1_000_000_000_000_000_000 == 0, "Deposit must be integer");
                assert!(amount.0 >= 10000000000000000000000, "Deposit must be a minimum of 10,000 $HAT");

                
                env::log_str("Processing deposit of tokens"); 
                assert_eq!(self.ft_token_id, env::predecessor_account_id(), "This token is not accepted.");

                // Get last vault
                let mut active_vault_index = self.vaults.len().checked_sub(1);

                log!("active_vault_index: {:?}", active_vault_index);

                // Get current timestamp
                let current_timestamp = env::block_timestamp();
                const one_day : u64 = 86400000000000;
                const one_hour : u64 = 3600000000000;
                const five_minutes : u64 = 300000000000;

                log!("current_timestamp: {}", current_timestamp);
                log!("countdown_period: {}", self.countdown_period);

                // Check if there is still time in the vault
                let is_vault_valid =  self.countdown_period > current_timestamp;
                log!("is_vault_valid: {:?}", is_vault_valid);

                // If there is no vault or the time has expired, create a new one.
                if active_vault_index.is_none() || !is_vault_valid{
                    assert!(amount.0 >= 100000000000000000000000, "The deposit to start a new vault must be a minimum of 100,000 $HAT");
                    self.countdown_period = (one_day*2)+current_timestamp;

                    let new_vault = VaultInfo {
                        winner: user_account.clone(),
                        token_amount: 0,
                        token_amount_complete: 0,
                        date_start: current_timestamp,
                        date_end: current_timestamp,
                        claimed: false,
                    };

                    // Reset amount of tokens in vault
                    self.ft_token_balance = 0;

                    self.vaults.push(&new_vault);
                    active_vault_index = Some(self.vaults.len() - 1);
    
                    log!("New vault added at index: {}", self.vaults.len() - 1);
                }
    
                // Get vault information
                let active_vault_index = active_vault_index.unwrap();
                let mut active_vault = self.vaults.get(active_vault_index.try_into().unwrap()).unwrap();
                
                // Update countdown period based on deposit amount
                if amount.0 <= 500000000000000000000000 { // 500,000 tokens or less - 0 days
                    if self.countdown_period-current_timestamp <= five_minutes {
                        log!("Five minutes added");
                        self.countdown_period = five_minutes+current_timestamp;
                    } else {
                        self.countdown_period = self.countdown_period;
                    }
                } else if amount.0 <= 1000000000000000000000000 { // 1,000,000 tokens or less - 1 day
                    self.countdown_period = one_day+current_timestamp;
                } else if amount.0 <= 5000000000000000000000000 { // 5,000,000 tokens or less - 12 hours
                    self.countdown_period = (one_day/2)+current_timestamp;
                } else if amount.0 < 20000000000000000000000000 { // less than 20,000,000 tokens - 1 hour
                    self.countdown_period = one_hour+current_timestamp;
                } else { // 20,000,000 tokens or more - 15 minutes
                    self.countdown_period = (one_hour/4)+current_timestamp;
                }

                log!("The new countdown period is: {}", self.countdown_period);
    
                // Send FT tokens to treasury as fee
                let covered_fees = amount.0 * self.treasury_fee / 100;

                ft_contract::ft_transfer(
                    self.treasury_id.clone(),
                    U128::from(covered_fees.clone()),
                    None,
                    self.ft_token_id.clone(),
                    1,
                    Gas(100_000_000_000_000)
                );
    
                log!("Deposit to fees: {}", covered_fees);
    
                // Calculate deposit without fees
                let deposit_without_fees = amount.0 * (100-self.treasury_fee) / 100;
                log!("Deposit to vault: {}", deposit_without_fees);
    
                // Update balance of active vault
                self.ft_token_balance += amount.0;
                log!("The new vault balance is: {}", self.ft_token_balance);
    
                // Update the active vault with the new deposit, user account and date end
                active_vault.winner = user_account.clone();
                active_vault.token_amount += deposit_without_fees;
                active_vault.token_amount_complete += amount.0;
                active_vault.date_end = self.countdown_period;
                
                self.time_last_deposit = current_timestamp;
                self.account_last_deposit = user_account;

                self.highest_deposit = if deposit > self.highest_deposit {
                    deposit
                } else {
                    self.highest_deposit
                };

                self.vaults.replace(active_vault_index.try_into().unwrap(), &active_vault);
    
                // Guardar la información del depósito en el historial
                self.deposit_history.insert(&DepositInfo {
                    account_id: self.account_last_deposit.clone(),
                    date: self.time_last_deposit,
                    ft_amount: amount.0.to_string(),
                    deposit_or_withdraw: true,
                });
    
                PromiseOrValue::Value(U128::from(0))
            }
            _ => PromiseOrValue::Value(U128::from(amount)),
        }
    }

    #[payable]
    pub fn claim_vault(&mut self, index: u64) {
        let user_account = env::predecessor_account_id();

        assert!(index < self.vaults.len(), "Invalid vault index");

        let mut vault = self.vaults.get(index).expect("Vault not found");

        assert!(!vault.claimed, "Vault already claimed");

        assert!(user_account == vault.winner, "Only the winner can claim the vault");

        let current_timestamp = env::block_timestamp();

        log!("vault.date_end: {}", vault.date_end);
        log!("countdown_period_withdraw: {}", self.countdown_period_withdraw);
        log!("date_end+countdown: {}", vault.date_end+self.countdown_period_withdraw);
        log!("current_timestamp: {}", current_timestamp);

        assert!(vault.date_end+self.countdown_period_withdraw < current_timestamp, "The time to open the vault is not over yet.");

        let vault_log = json!({
            "winner": vault.winner,
            "token_amount": vault.token_amount.to_string(),
            "date_start": vault.date_start,
            "date_end": vault.date_end,
            "claimed": vault.claimed,
        });
    
        env::log(
            vault_log.to_string().as_bytes(),
        );

        // Transfer tokens to winner
        ft_contract::ft_transfer(
            env::signer_account_id(),
            U128::from(vault.token_amount),
            None,
            self.ft_token_id.clone(),
            1,
            Gas(100_000_000_000_000)
        );

        // Change status of vault
        vault.claimed = true;
        self.vaults.replace(index, &vault);

        log!("Vault {} claimed by {}", index, env::signer_account_id());
    }
}