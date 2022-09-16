import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { admins, developers, endUsers, operators } from '@/fixtures/identities';
import { getActor } from '@/settings/agent';
import { getCanisterId } from '@/settings/utils';
import { idlFactory } from '@/idls/ego_store.idl';
import { _SERVICE, GetAppResponse } from '@/idls/ego_store';
import { idlFactory as bucketIdl } from '@/idls/ego_bucket.idl';
import { _SERVICE as bucketService } from '@/idls/ego_bucket';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { identity } from '@/settings/identity';
import { Principal } from '@dfinity/principal';

export const endUserActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(endUsers[0]?.identity)),
  idlFactory,
  getCanisterId('ego_store')!,
);

export const developerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(developers[0]?.identity)),
  idlFactory,
  getCanisterId('ego_store')!,
);

const bucketActor = getActor<bucketService>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(developers[0]?.identity)),
  bucketIdl,
  getCanisterId('ego_bucket')!,
);

export const auditerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(operators[0]?.identity)),
  idlFactory,
  getCanisterId('ego_store')!,
);

export const managerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(admins[0]?.identity)),
  idlFactory,
  getCanisterId('ego_store')!,
);

export const deployerActor = getActor<_SERVICE>(
  identity,
  idlFactory,
  getCanisterId('ego_store')!,
);

export const ego_ledger_id = getCanisterId('ego_ledger');
export const ego_crond_id = getCanisterId('ego_crond');
export const ego_bucket_id = getCanisterId('ego_bucket');
export const assets_storage_id = getCanisterId('assets_storage');

const ego_wallet_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_wallet/ego_wallet_opt.wasm',
);

const ego_assets_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_assets/ego_assets_opt.wasm',
);

const app_1_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/clients/fixtures/app_1.wasm',
);

const app_1_asset_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/clients/fixtures/app_1_asset.wasm',
);

const version = {
  major: 1,
  minor: 0,
  patch: 0,
};

const next_version = {
  major: 1,
  minor: 0,
  patch: 1,
};

const is_same_version = (v1, v2) => {
  return v1.major == v2.major && v1.minor == v2.minor && v1.patch == v2.patch;
};

export const storePostInstall = async () => {
  const endUser = await endUserActor;
  const developer = await developerActor;
  const auditer = await auditerActor;
  const manager = await managerActor;
  const deployer = await deployerActor;

  console.log(`=== post install script of ego_store starts: ===\n`);

  // init store
  console.log(
    `1. init_store {ego_ledger_id : ${ego_ledger_id}, ego_crond_id : ${ego_crond_id}, ego_bucket_id : ${ego_bucket_id}}\n`,
  );

  let resp1 = await deployer.init_store({
    ego_ledger_id: Principal.fromText(ego_ledger_id!),
    ego_crond_id: Principal.fromText(ego_crond_id!),
    ego_bucket_id: Principal.fromText(ego_bucket_id!),
  });
  console.log(resp1);

  console.log(`2. register users\n`);
  await register_user(
    endUser,
    'user',
    endUsers[0]?.principal,
    false,
    false,
    false,
  );
  await register_user(
    developer,
    'developer',
    developers[0]?.principal,
    true,
    false,
    false,
  );
  await register_user(
    auditer,
    'auditer',
    operators[0]?.principal,
    false,
    true,
    false,
  );
  await register_user(
    manager,
    'manager',
    admins[0]?.principal,
    false,
    false,
    true,
  );

  console.log(`3. create apps\n`);
  await create_app(
    'ego_wallet',
    { System: null },
    'ego_wallet',
    1,
    version,
    ego_wallet_wasm,
  );

  await create_app(
    'ego_assets',
    { System: null },
    'ego_assets',
    1,
    version,
    ego_assets_wasm,
  );

  await create_app(
    'app_1',
    { Vault: null },
    'app_1',
    1,
    version,
    app_1_wasm,
    Principal.fromText(assets_storage_id!),
  );

  // comment out for poc

  // console.log(`4. create orders\n`);
  // await create_order(endUser, 'ego_wallet');
  // await create_order(endUser, 'app_1');
  //
  // console.log(`5. upgrade app version\n`);
  // await new_app_version('ego_wallet', next_version, ego_wallet_wasm);
  //
  // await new_app_version(
  //   'app_1',
  //   next_version,
  //   app_1_wasm,
  //   Principal.fromText(assets_storage_id!),
  // );
};

const register_user = async (
  actor,
  name,
  principal,
  is_app_developer,
  is_app_auditer,
  is_manager,
) => {
  const deployer = await deployerActor;

  console.log(`\t\t a.register user ${name}\n`);

  let resp1 = await actor.register_user({
    name: name,
  });
  console.log(resp1);

  console.log(`\t\t b.set role for ${name}\n`);
  let resp2 = await deployer.set_role({
    user_id: Principal.fromText(principal),
    is_app_developer,
    is_app_auditer,
    is_manager,
  });
  console.log(resp2);
};

const create_app = async (
  name: string,
  category: any,
  app_id: string,
  price: number,
  version: any,
  wasm_file: ArrayLike<number>,
  frontend_canister_id?: Principal,
) => {
  const developer = await developerActor;

  console.log(`\t\t --- create app for ${app_id} ---\n`);
  console.log(`\t\t a.register app ${app_id}\n`);

  let resp1 = await developer.register_app({
    name,
    app_id,
    category,
    price,
  });
  console.log(resp1);

  await new_app_version(app_id, version, wasm_file, frontend_canister_id);
};

const new_app_version = async (
  app_id: string,
  version: any,
  wasm_file: ArrayLike<number>,
  frontend_canister_id?: Principal,
) => {
  const developer = await developerActor;
  const bucket = await bucketActor;
  const auditer = await auditerActor;

  console.log(`\t\t b.new version for ${app_id}\n`);
  await developer.new_app_version({
    version,
    app_id,
  });

  console.log(`\t\t c.upload wasms for ${app_id}\n`);
  let getAppResponse = await developer.get_app({
    app_id,
  });

  let backend_wasm;
  let asset_wasm;
  (getAppResponse as { Ok: GetAppResponse }).Ok.app.versions.forEach(ver => {
    if (is_same_version(ver.version, version)) {
      ver.wasms.forEach(wasm => {
        if (wasm.canister_type.hasOwnProperty('BACKEND')) {
          backend_wasm = wasm;
        }
        if (wasm.canister_type.hasOwnProperty('ASSET')) {
          asset_wasm = wasm;
        }
      });
    }
  });

  const fileMd5 = crypto
    .createHash('md5')
    .update(wasm_file as BinaryLike)
    .digest('hex');

  console.log(`\t\t d.upload backend wasm file for ${app_id}\n`);
  const uploadBackendFileResponse = await bucket.upload_file({
    fid: backend_wasm.file_id!,
    appid: app_id,
    data: Array.from(wasm_file),
    hash: fileMd5,
    version: `${version.major}.${version.minor}.${version.patch}`,
  });

  console.log(uploadBackendFileResponse);

  if (frontend_canister_id) {
    console.log(`\t\t e.set frontend address for ${app_id}\n`);

    const setFrontendAddressResponse = await developer.set_frontend_address({
      app_id: app_id,
      version: version,
      canister_id: frontend_canister_id,
    });

    console.log(setFrontendAddressResponse);
  }

  console.log(`\t\t f.submit version for ${app_id}\n`);
  await developer.submit_app_version({
    version,
    app_id,
  });

  console.log(`\t\t g.approve version for ${app_id}\n`);
  await auditer.approve_app_version({
    version,
    app_id,
  });

  console.log(`\t\t h.release version for ${app_id}\n`);
  await developer.release_app_version({
    version,
    app_id,
  });
};

const create_order = async (user, app_id) => {
  const store = await developerActor;

  console.log(`\t\t--- begin create_order for ${app_id} --`);

  console.log(`\t\t a.create_order for ${app_id}`);
  const createOrderResponse = await user.create_app_order({
    app_id: app_id,
  });
  console.log(createOrderResponse);
  let order = createOrderResponse.Ok.order;

  console.log(`\t\t b.notify payment with memo`);
  const notifyPaymentResponse = await store.notify_payment({
    memo: order.memo,
  });
  console.log(notifyPaymentResponse);
};
