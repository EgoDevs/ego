import { devPostInstall } from './ego_dev';
// import { storePostInstall } from './ego_store';
import { cronPostInstall } from './ego_cron';
// import { walletPostInstall } from './ego_wallet';
// import { ledgerPostInstall } from './ego_ledger';
import { assetsStorageInstall } from './assets_storage';
import { opsPostInstall } from './ego_ops';

/// use `post_` prefix in each describe to allow jest to specify each tests
/// for single project post install , please add below and use templates like `post_bucket`

// run post_infra
describe('post_infra', () => {
  test('post install', async () => {
    await assetsStorageInstall();

    // infra
    await devPostInstall();
    // await cronPostInstall();
    // await storePostInstall();
  });
});

// run post_apps
describe('post_apps', () => {
  test('post install', async () => {
    // await walletPostInstall();
    // await assetsStorageInstall();
    // await ledgerStorageInstall();
  });
});

// run post_bucket
describe('post_dev', () => {
  test('dev post install', async () => {
    await devPostInstall();
  });
});

// run post_cron
describe('post_cron', () => {
  test('cron post install', async () => {
    await crondPostInstall();
  });
});

// run post_store
describe('post_store', () => {
  test('store post install', async () => {
    await storePostInstall();
  });
});

// run post_assets
describe('post_assets', () => {
  test('assets post install', async () => {
    await assetsStorageInstall();
  });
});

// run post_ops
describe('post_ops', () => {
  test('ego_ops post install', async () => {
    await opsPostInstall();
  });
});

// run post_ledger
// describe('post_ledger', () => {
//   test('ledger post install', async () => {
//     await ledgerPostInstall();
//   });
// });
