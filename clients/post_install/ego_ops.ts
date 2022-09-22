import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { getActor } from '@/settings/agent';
import { getCanisterId } from '@/settings/utils';

import { _SERVICE as EgoOpsService} from '@/idls/ego_ops';
import { _SERVICE as EgoDevService} from '@/idls/ego_dev';
import { _SERVICE as EgoStoreService} from '@/idls/ego_store';
import { _SERVICE as EgoFileService} from '@/idls/ego_file';
import { _SERVICE as EgoTenantService} from '@/idls/ego_tenant';

import { idlFactory } from '@/idls/ego_ops.idl';
import { identity } from '@/settings/identity';
import {Principal} from "@dfinity/principal";
import {ActorSubclass} from "@dfinity/agent";

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

export const ego_ops_id = Principal.fromText(getCanisterId('ego_ops')!);
export const ego_dev_id = Principal.fromText(getCanisterId('ego_dev')!);
export const ego_store_id = Principal.fromText(getCanisterId('ego_store')!);
export const ego_file_id = Principal.fromText(getCanisterId('ego_file')!);
export const ego_tenant_id = Principal.fromText(getCanisterId('ego_tenant')!);


export const opsPostInstall = async () => {
  let opsOperator = await getOperator<EgoOpsService>('ego_ops');

  console.log(`=== post install script of ego_ops starts: ===\n`);

  console.log(`1. set ego_ops as canister owner\n`);
  // await role_owner_sets();

  console.log(`2. canister_main_register\n`);
  let resp11 = await opsOperator.canister_main_register({
    ego_dev_id, ego_store_id, ego_file_id, ego_tenant_id
  });

  console.log(`3. admin_app_create\n`);
  await admin_app_creates();

  console.log(resp11);

  let resp12 = await opsOperator.canister_main_list();

  console.log(resp12.Ok);
};

async function getOperator<T>(
  canisterName: string
): Promise<ActorSubclass<T>> {
  let operator = await getActor<T>(
    identity,
    idlFactory,
    getCanisterId(canisterName)!,
  )
  return operator;
}

const role_owner_sets = async () => {
  let devOperator = await getOperator<EgoDevService>('ego_dev');
  let storeOperator = await getOperator<EgoStoreService>('ego_store');
  let fileOperator = await getOperator<EgoFileService>('ego_file');
  let tenantOperator = await getOperator<EgoTenantService>('ego_tenant');
  let cronOperator = await getOperator<EgoTenantService>('ego_cron');

  console.log(`==> a. set ego_ops as ego_dev owner\n`);
  let resp1 = await devOperator.role_owner_set([ego_ops_id]);
  console.log(resp1)

  console.log(`==> b. set ego_ops as ego_store owner\n`);
  let resp2 = await storeOperator.role_owner_set([ego_ops_id]);
  console.log(resp2)

  console.log(`==> c. set ego_ops as ego_file owner\n`);
  let resp3 = await fileOperator.role_owner_set([ego_ops_id]);
  console.log(resp3)

  console.log(`==> d. set ego_ops as ego_tenant owner\n`);
  let resp4 = await tenantOperator.role_owner_set([ego_ops_id]);
  console.log(resp4)
}

const admin_app_creates = async () => {
  console.log(`==> a. admin_app_create ego_dev\n`);
  let resp1 = await admin_app_create('ego_dev', 'ego_dev', version, ego_dev_wasm);
  console.log(resp1)

  console.log(`==> b. admin_app_create ego_store\n`);
  let resp2 = await admin_app_create('ego_store', 'ego_store', version, ego_store_wasm);
  console.log(resp2)

  console.log(`==> c. admin_app_create ego_file\n`);
  let resp3 = await admin_app_create('ego_file', 'ego_file', version, ego_file_wasm);
  console.log(resp3)

  console.log(`==> d. admin_app_create ego_tenant\n`);
  let resp4 = await admin_app_create('ego_tenant', 'ego_tenant', version, ego_tenant_wasm);
  console.log(resp4)
}

const admin_app_create = async (
  app_id: string,
  name: string,
  version: any,
  backend_data: ArrayLike<number>,
  frontend_canister_id?: Principal,
) => {
  let opsOperator = await getOperator<EgoOpsService>('ego_ops');

  const backend_hash = crypto
    .createHash('md5')
    .update(backend_data as BinaryLike)
    .digest('hex');

  let resp1 = await opsOperator.admin_app_create({
    app_id,
    name,
    version,
    backend_data: Array.from(backend_data),
    backend_hash,
    frontend: frontend_canister_id ? [frontend_canister_id] : [],
  });
  console.log(resp1);
};