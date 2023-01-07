#!/usr/bin/env bash
set -euo pipefail

# deploy  contract
echo ">>>>>> Customizing Installing ego_store"

dfx canister install --mode=install ego_store --argument '("development")'