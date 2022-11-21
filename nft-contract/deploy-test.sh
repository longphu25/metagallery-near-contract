#!/bin/bash

near login

export ID_TEST=metagallery.testnet
echo $ID_TEST
export SUB_ID=nft-contract.$ID_TEST
echo $SUB_ID

# near send $SUB_ID $ID_TEST 60

near create-account $SUB_ID --masterAccount $ID_TEST --initialBalance 5
near call $ID_TEST storage_deposit '{"account_id":"'$SUB_ID'"}' --accountId $ID_TEST --amount 50

# near deploy --wasmFile out/metag_nft.wasm --accountId $SUB_ID
near deploy --wasmFile out/nft_contract.wasm --accountId $SUB_ID

near call $SUB_ID new_default_metadata '{"owner_id": "'$SUB_ID'"}' --accountId $SUB_ID

# near call $SUB_ID nft_mint '{"token_id": "0", "receiver_id": "'$ID_TEST'", "token_metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId $ID_TEST --deposit 10
near delete $SUB_ID $ID_TEST