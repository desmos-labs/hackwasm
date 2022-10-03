#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
DESMOS_HOME="$SCRIPT_DIR/.desmos"
POST_TEMPLATE="$SCRIPT_DIR/post_template.json"

# Desmos rpc address
DESMOS_RPC="https://rpc.morpheus.desmos.network:443"
# Desmos chain id
DESMOS_CHAIN_ID="morpheus-apollo-2"
# Key used to sign the transactions
KEY_NAME="cosmoverse"
# Subspace in which the posts are created
SUBSPACE=9
# Address of the IBC contract that creates the cyber link on the cyber chain.
CONTRACT_ADDRESS="desmos1h6xfyx0q9zlm3rrm26dt0l900qjv3ucpuh7w357zwmu7p424ckfsdwcpk3"

desmos() {
  "$SCRIPT_DIR/desmos" --home="$DESMOS_HOME" "$@"
}

# Load the post template content
post=$(cat "$POST_TEMPLATE")

echo "Write the post text and press enter"
read -r

post_text="$REPLY"
# Extract keyword from post text
keywords=$(echo "$post_text" | tr " " "\n")

# Append the keyword to the tags field
for keyword in $keywords; do
  post=$(echo "$post" | jq ".tags += [\"$keyword\"]")
done

# Write post.json
echo "$post" | jq ".text = \"$post_text\"" >post.json

# Create post
echo "Creating post..."
output=$(desmos tx posts create $SUBSPACE 0 post.json \
  --from "$KEY_NAME" --chain-id "$DESMOS_CHAIN_ID" --fees 500udaric \
  --node "$DESMOS_RPC" --keyring-backend test -b block -y --output json)

echo "Tx output"
echo "$output" | jq
# Resolve post id from tx output
post_id=$(echo "$output" | jq '.logs[0].events[0].attributes[] | select(.key | contains("post_id")) | .value')
post_id="18"
echo "New post id $post_id"

msg="{\"cyber_index_post\":{\"subspace_id\":\"$SUBSPACE\",\"post_id\":\"$post_id\"}}"
echo "Contract msg: $msg"

# Create cyber link
echo "Creating cyber link..."
desmos tx wasm execute "$CONTRACT_ADDRESS" "$msg" \
  --chain-id "$DESMOS_CHAIN_ID" --from "$KEY_NAME" --fees 500udaric \
  --node "$DESMOS_RPC" --keyring-backend test -b block -y
