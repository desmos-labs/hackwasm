#!/usr/bin/env bash

HOMEDIR=$1
RELAYER_ADDR="bostrom13mmej4wlsgvmcaxeq2pej5ar4wp8u5hln4tx73"
TEST_ADDR="bostrom1lthj7cjrtlkkg6tt5e5twaftwdyku2tud6j8km"

# initial a test chain genesis and keybase config
cyber init testnet --chain-id "cyber" --home "$HOMEDIR/cyber"

# Add test node key
cyber keys add "node" --home "$HOMEDIR/cyber" --keyring-backend "test" --no-backup
cyber add-genesis-account "node" "1000000000000stake" --home "$HOMEDIR/cyber" --keyring-backend "test"

# Add test account keys
cyber keys add "test" --home "$HOMEDIR/cyber" --keyring-backend "test" --no-backup
cyber add-genesis-account "test" "1000000000000stake" --home "$HOMEDIR/cyber" --keyring-backend "test"

# Add relayer account defiened in relayer-config to genesis
cyber add-genesis-account $TEST_ADDR "1000000000000stake" --home "$HOMEDIR/cyber"

# Create validator set to genesis
cyber gentx "node" "500000000stake" --chain-id "cyber" --home "$HOMEDIR/cyber" --keyring-backend "test" --node-id "node"
cyber collect-gentxs --home "$HOMEDIR/cyber"