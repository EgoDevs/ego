# EGO

**This Project is Under Development, Scripts and Documentations are under-construction!**

EGO is a smart-contract platform built on IC's canisters. EGO's goal is to allow users to own and control his/her canisters, in an easy way, like App store. Upgrade canisters into latest or specific version, backup and restore smartcontracts data, interlink with other installed canisters, even interact with user-owned distributed/decentralized marketplace.

There are at least 3 different roles in EGO.

- Developers: Upload and maintain canister wasm, and frontend applications. Pay to install or free to use.
- Admin/Auditors/Committee: set roles, audit wasm code, progress network updates and governance.
- Users: Install and use his/her owned DApps through his/her wallet.

## Prequeries

- rust
- nodejs
- dfx client

```bash
sh -ci "$(curl -fsSL https://smartcontracts.org/install.sh)"
```

## bootstrap

0. root folder but a new terminal run dfx, don't close

```
dfx start --clean
```

1. install dependencies

```bash
    npm install pnpm -g && pnpm install
```

2. put seedphrase(12 words) with a name `internal.text`, put it under `credentials` folder

```tree
    credentials/
        internal.txt
```

3. Scripts

   1. install IC Canisters, ledger/II/NNS

   ```bash
   pnpm run ego:pre
   ```

   2. bootstrap and create canister ids

   ```bash
   pnpm run ego:bootstrap
   ```

   3. build projects, infra/apps or single project

   ```bash
    pnpm run ego:build # build all projects
    pnpm run ego:build --infra # build infra projects
    pnpm run ego:build --apps # build apps projects
    pnpm run ego:build --project=ego_dev # build ego_dev
   ```

   4. install projects, infra/apps or single project

   ```bash
    pnpm run ego:install # install all projects
    pnpm run ego:install --infra # install infra projects
    pnpm run ego:install --apps # install apps projects
    pnpm run ego:install --project=ego_dev # install ego_dev
   ```

   5. reinstall projects, infra/apps or single project

   ```bash
    pnpm run ego:reinstall # reinstall all projects
    pnpm run ego:reinstall --infra # install infra projects
    pnpm run ego:reinstall --apps # install apps projects
    pnpm run ego:reinstall --project=ego_dev # install ego_dev
   ```

   6. upgrade projects, infra/apps or single project

   ```bash
    pnpm run ego:upgrade # upgrade all projects
    pnpm run ego:upgrade --infra # upgrade infra projects
    pnpm run ego:upgrade --apps # upgrade apps projects
    pnpm run ego:upgrade --project=ego_wallet # upgrade ego_wallet
   ```

   6. post install

   ```bash
   pnpm run ego:post_install
   pnpm run ego:post_install:infra # run infra post_install
   pnpm run ego:post_install:apps # run apps post_install
   pnpm run ego:post_install:project post_wallet # run wallet post_install, please use `post_` prefix to project
   ```

   7. run tests

   ```bash
   pnpm run test
   ```

4. Once and for all

   ```bash

   pnpm run ego:pre && pnpm run ego:bootstrap && pnpm run ego:build && pnpm run ego:install && pnpm run ego:post_install
   ```
