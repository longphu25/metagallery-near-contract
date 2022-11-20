#!/bin/bash

rm -rf neardev/
near dev-deploy --wasmFile out/metag_ft.wasm --helperUrl https://near-contract-helper.onrender.com
source neardev/dev-account.env
echo $CONTRACT_NAME

near call $CONTRACT_NAME new '{"owner_id": "'$CONTRACT_NAME'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Meta Gallery token", "symbol": "META", "decimals": 8 }}' --accountId $CONTRACT_NAME

near view $CONTRACT_NAME ft_metadata

# # Transfer Example

# near create-account Cuong.$CONTRACT_NAME --masterAccount $CONTRACT_NAME --initialBalance 1

# # Add storage deposit for Cuong's account:

# near call $CONTRACT_NAME storage_deposit '' --accountId Cuong.$CONTRACT_NAME --amount 0.00125

# # Check balance of Cuong's account, it should be `0` for now:

# near view $CONTRACT_NAME ft_balance_of '{"account_id": "'Cuong.$CONTRACT_NAME'"}'
