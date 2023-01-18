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
