# ego-rust-template

This Template is created for EGO projects

## ENV setup

- rust 1.65.0+
- dfx 0.12.1+
- didc [download binary](https://github.com/dfinity/candid/releases), export PATH to `didc`

- **!! Important !! Manually Setup Credentials**

  - Under `credentials` folder, you need to add 2 files.
    1.  `seedPhrase.txt`: 12 words mnemonic phrases, to create secp256k1 account for local test
    2.  `production.pem`: pem file with secp256k1 curve encoded, use for `mainnet` deployment
    3.  You can change file names on `ego-config`.json
  - Modify `ego-config`.json, change `production_cycles_wallet` to your cycles wallet.

- setup project, see `ego-projects.json`,

- **Lazy Setup Credentials**

  ```
  pnpm run ego:credentials
  ```

  Will generate `seedPhrase.txt` and `production.pem` for you.
  **But!! You have to Setup manually on production!!**

## Quick Start

1. `pnpm install`
2. `pnpm run ego:run` to create and deploy
3. `pnpm run test ego_example` to run test file in `clients/tests`

## Scripts

1. build projects, infra/apps or single project

```bash
 pnpm run ego:build # build all projects
 pnpm run ego:build --infra # build infra projects
 pnpm run ego:build --apps # build apps projects
 pnpm run ego:build --project=ego_dev # build ego_dev
```

2. install projects, infra/apps or single project

```bash
 pnpm run ego:install # install all projects
 pnpm run ego:install --infra # install infra projects
 pnpm run ego:install --apps # install apps projects
 pnpm run ego:install --project=ego_dev # install ego_dev
```

3. reinstall projects, infra/apps or single project

```bash
 pnpm run ego:reinstall # reinstall all projects
 pnpm run ego:reinstall --infra # install infra projects
 pnpm run ego:reinstall --apps # install apps projects
 pnpm run ego:reinstall --project=ego_dev # install ego_dev
```

4. upgrade projects, infra/apps or single project

```bash
 pnpm run ego:upgrade # upgrade all projects
 pnpm run ego:upgrade --infra # upgrade infra projects
 pnpm run ego:upgrade --apps # upgrade apps projects
 pnpm run ego:upgrade --project=ego_wallet # upgrade ego_wallet
```

5. post install

```bash
pnpm run ego:post_install
pnpm run ego:post_install:infra # run infra post_install
pnpm run ego:post_install:apps # run apps post_install
pnpm run ego:post_install:project post_wallet # run wallet post_install, please use `post_` prefix to project
```
