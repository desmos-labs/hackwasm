#!/usr/bin/env bash

HOMEDIR=$1
RELAYER_ADDR="desmos1punhxfyxvnwup70mc6cz9cmuxu74ars7x6hgjm"
TEST_ADDR="desmos1ha4f852205lgsntq579x74ndfnqacy8z9uqqqa"

# initial a test chain genesis and keybase config
desmos init testnet --chain-id "desmos" --home "$HOMEDIR/desmos"

# Add test node key
desmos keys add "node" --home "$HOMEDIR/desmos" --keyring-backend "test" --no-backup
desmos add-genesis-account "node" "1000000000000stake" --home "$HOMEDIR/desmos" --keyring-backend "test"

# Add test account keys
desmos keys add "test" --home "$HOMEDIR/desmos" --keyring-backend "test" --no-backup
desmos add-genesis-account "test" "1000000000000stake" --home "$HOMEDIR/desmos" --keyring-backend "test"

# Add relayer account defiened in relayer-config to genesis
desmos add-genesis-account $RELAYER_ADDR "1000000000000stake" --home "$HOMEDIR/desmos"

# Add test account
desmos add-genesis-account $TEST_ADDR "1000000000000stake" --home "$HOMEDIR/desmos"

# Create validator set to genesis
desmos gentx "node" "500000000stake" --chain-id "desmos" --home "$HOMEDIR/desmos" --keyring-backend "test" --node-id "node"
desmos collect-gentxs --home "$HOMEDIR/desmos"