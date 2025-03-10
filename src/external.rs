use near_sdk::{ext_contract};
use near_sdk::json_types::U128;
use near_sdk::{AccountId};

pub const TGAS: u64 = 1_000_000_000_000;
pub const NO_DEPOSIT: u128 = 0;
pub const XCC_SUCCESS: u64 = 1;

#[ext_contract(ft_contract)]
pub trait ExternsContract {
    fn ft_transfer(
        &mut self, 
        receiver_id: AccountId, 
        amount: U128, 
        memo: Option<String>
    );
    
}

#[ext_contract(ext_self)]
trait NonFungibleTokenResolver {

}