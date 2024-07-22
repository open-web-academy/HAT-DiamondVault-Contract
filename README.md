## Description 📄

Diamond Vault Contract.

## Contract Methods 🚀

Assign the identifier of our deployed contract to a constant (Replace VAULT_CONTRACT with the deployed contract's identifier):
 
    Auctions
    VAULT_CONTRACT=diamondvault.hat-coin.near
    echo $VAULT_CONTRACT

    FT
    FT_CONTRACT=hat.tkn.near
    echo $FT_CONTRACT

Contract initialization:

    NEAR_ENV=mainnet near call $VAULT_CONTRACT new '{"owner_id":"'$VAULT_CONTRACT'","ft_token_id":"'$FT_CONTRACT'","treasury_id":"open-web-academy.sputnik-dao.near", "treasury_fee" : 1, "countdown_period_withdraw": 604800000000000 }' --accountId $VAULT_CONTRACT

    NEAR_ENV=mainnet near call $FT_CONTRACT storage_deposit '{"account_id":"testirving.testnet"}' --accountId yairnava.testnet --deposit 0.01

    NEAR_ENV=mainnet near call $FT_CONTRACT ft_transfer '{ "receiver_id": "testirving.testnet", "amount": "100000000000000000000000000"}' --accountId yairnava.testnet --depositYocto 1

    NEAR_ENV=mainnet near view $FT_CONTRACT ft_balance_of '{"account_id":"testirving.testnet"}'

Send Tokens

    1000000000000000000 = 1 token

    NEAR_ENV=mainnet near call $FT_CONTRACT ft_transfer_call '{"receiver_id": "'$VAULT_CONTRACT'", "amount": "100000000000000000000000", "msg": "{\"action_to_execute\": \"increase_deposit\"}"}' --accountId yairnava.testnet --depositYocto 1 --gas 200000000000000

Get Highest Withdraw

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_highest_withdraw 

Get Highest Deposit

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_highest_deposit 

Get Vault Balance

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_vault_balance 

Get Vaults

    NEAR_ENV=mainnet near view hat-diamondvault.testnet get_vaults '{"start_index":0, "limit":10}'

Get Last Vault

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_last_vault

Get Last Deposit

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_last_deposit

Get Time Last Deposit

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_time_last_deposit 

Get Vault End Date

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_end_date

Get Countdown Period To Withdraw

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_countdown_period_withdraw

Get Countdown Period

    NEAR_ENV=mainnet near view $VAULT_CONTRACT get_countdown_period

Claim Tokens

    NEAR_ENV=mainnet near call hat-diamondvault.testnet claim_vault '{"index": 0}' --accountId yairnava.testnet --depositYocto 1 --gas 200000000000000

Set Treasury

    NEAR_ENV=mainnet near call $VAULT_CONTRACT set_treasury '{"new_treasury_id": "yairnava.testnet"}' --accountId yairnava.testnet --depositYocto 1 --gas 200000000000000

Set Countdown Period To Withdraw

    NEAR_ENV=mainnet near call $VAULT_CONTRACT set_countdown_period_withdraw '{"new_countdown": 3600000000000}' --accountId yairnava.testnet --depositYocto 1 --gas 200000000000000

Set Owner

    NEAR_ENV=mainnet near call $VAULT_CONTRACT change_owner '{"new_owner_id": "yairnava.testnet"}' --accountId yairnava.testnet --depositYocto 1 --gas 200000000000000