#!/bin/bash

near login

export ID_TEST=metagallery.testnet
echo $ID_TEST

near deploy --wasmFile out/metag_ft.wasm --accountId $ID_TEST

near call $ID_TEST new '{"owner_id": "'$ID_TEST'", "total_supply": "1000000000000000", "metadata": { "spec": "ft-1.0.0", "name": "Meta Gallery token", "symbol": "META", "decimals": 8 }}' --accountId $ID_TEST

near view $ID_TEST ft_metadata