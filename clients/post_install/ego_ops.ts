import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { admins, developers, endUsers, operators } from '@/fixtures/identities';
import { getActor } from '@/settings/agent';
import { getCanisterId } from '@/settings/utils';
import { _SERVICE } from '@/idls/ego_ops';
import { idlFactory } from '@/idls/ego_ops.idl';
import { identity } from '@/settings/identity';
import { Principal } from '@dfinity/principal';

export const operatorActor = getActor<_SERVICE>(
  identity,
  idlFactory,
  getCanisterId('ego_ops')!,
);

const ego_file_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_file/ego_file_opt.wasm',
);

const ego_tenant_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_tenant/ego_tenant_opt.wasm',
);

const ego_dev_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_dev/ego_dev_opt.wasm',
);

const ego_store_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_store/ego_store_opt.wasm',
);

const version = {
  major: 1,
  minor: 0,
  patch: 0,
};


export const opsPostInstall = async () => {
  const operator = await operatorActor;
  console.log(`=== post install script of ego_ops starts: ===\n`);

  console.log(`1. install ego_file\n`);

  const fileMd5 = crypto
    .createHash('md5')
    .update(ego_file_wasm as BinaryLike)
    .digest('hex');

  let resp1 = await operator.canister_main_create({
    app_id: 'ego_file',
    version: version,
    data: Array.from(ego_file_wasm),
    hash: fileMd5,
  });

  console.log(resp1);

  console.log(`2. install ego_tenant\n`);

  const tenantMd5 = crypto
    .createHash('md5')
    .update(ego_tenant_wasm as BinaryLike)
    .digest('hex');

  let resp2 = await operator.canister_main_create({
    app_id: 'ego_tenant',
    version: version,
    data: Array.from(ego_tenant_wasm),
    hash: tenantMd5,
  });

  console.log(resp2);

  console.log(`3. install ego_dev\n`);

  const devMd5 = crypto
    .createHash('md5')
    .update(ego_dev_wasm as BinaryLike)
    .digest('hex');

  let resp3 = await operator.canister_main_create({
    app_id: 'ego_dev',
    version: version,
    data: Array.from(ego_dev_wasm),
    hash: devMd5,
  });

  console.log(resp3);

  console.log(`4. install ego_store\n`);

  const storeMd5 = crypto
    .createHash('md5')
    .update(ego_store_wasm as BinaryLike)
    .digest('hex');

  let resp4 = await operator.canister_main_create({
    app_id: 'ego_store',
    version: version,
    data: Array.from(ego_store_wasm),
    hash: storeMd5,
  });

  console.log(resp4);

  let resp5 = await operator.canister_main_list();

  console.log(resp5.Ok);
};
