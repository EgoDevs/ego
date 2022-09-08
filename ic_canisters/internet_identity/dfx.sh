#!/usr/bin/env bash
set -euo pipefail

# deploy ledger contract
echo ">>>>>> Deploying Internet Identitty"

II_ENV=development dfx deploy --no-wallet --argument '(null)'