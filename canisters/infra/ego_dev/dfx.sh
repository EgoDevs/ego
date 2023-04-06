#!/usr/bin/env bash
set -euo pipefail

# deploy contract
echo ">>>>>> Customizing Installing ego_dev"

dfx canister install --mode=install ego_dev --argument '("development")'