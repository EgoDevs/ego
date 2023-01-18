// import { walletPostInstall } from './ego_wallet';
// import { ledgerPostInstall } from './ego_ledger';

import { assetsStorageInstall } from './assets_storage';
import { opsPostInstall } from './ego_ops';

/// use `post_` prefix in each describe to allow jest to specify each tests
/// for single project post install , please add below and use templates like `post_bucket`

// run post_infra
describe('post_infra', () => {
  test('post install', async () => {
    // await assetsStorageInstall();

    // infra
    await opsPostInstall();
  });
});

// run post_ops
describe('post_ops', () => {
  test('ego_ops post install', async () => {
    await opsPostInstall();
  });
});
