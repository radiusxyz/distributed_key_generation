#!/bin/bash
CURRENT_PATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
PROJECT_ROOT_PATH="$( cd $SCRIPT_PATH/../.. >/dev/null 2>&1 ; pwd -P )"
KEY_GENERATOR_BIN_PATH="$PROJECT_ROOT_PATH/scripts/key-generator"

DATA_PATH=$CURRENT_PATH/../../data
CONFIG_FILE_PATH=$DATA_PATH/Config.toml
PRIVATE_KEY_PATH=$DATA_PATH/signing_key

KEY_GENERATOR_INTERNAL_RPC_URL="http://127.0.0.1:7200"
KEY_GENERATOR_CLUSTER_RPC_URL="http://127.0.0.1:7300"
KEY_GENERATOR_EXTERNAL_RPC_URL="http://127.0.0.1:7100"

KEY_GENERATOR_PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"