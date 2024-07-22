use crate::*;
 
#[near_bindgen]
impl Contract {
    pub fn set_treasury(&mut self,new_treasury_id:AccountId) -> AccountId {
        //if the caller is the owner
        //It can modify the parameters
        self.is_the_owner();
        self.treasury_id=new_treasury_id;
        self.treasury_id.clone()
    }

    pub fn set_countdown_period_withdraw(&mut self,new_countdown:Timestamp) -> Timestamp {
        self.is_the_owner();
        self.countdown_period_withdraw=new_countdown;
        self.countdown_period_withdraw.clone()
    }

    pub fn change_owner(&mut self, new_owner_id: AccountId) -> AccountId {
        self.is_the_owner();        
        self.owner_id = new_owner_id;
        self.owner_id.clone()
    }

     //validate if the owner is the caller
     #[private]
    pub fn is_the_owner(&self)   {
        //validate that only the owner contract add new contract address
        assert_eq!(
            self.owner_id==env::predecessor_account_id(),
            true,
            "!you are not the contract owner address¡"
        );
    }


}
