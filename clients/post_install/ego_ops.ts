import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';
import path from 'path';

import { getActor } from '@/settings/agent';
import { getCanisterId, hasOwnProperty } from '@/settings/utils';

import { _SERVICE as EgoOpsService, Category } from '@/idls/ego_ops';

import { idlFactory } from '@/idls/ego_ops.idl';
import { identity } from '@/settings/identity';
import { Principal } from '@dfinity/principal';
import { ActorSubclass } from '@dfinity/agent';
import { DeployMode } from '@/idls/ego_dev';

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

const ego_assets_wasm = fs.readFileSync(
  `${[process.cwd()]}` + '/artifacts/ego_assets/ego_assets_opt.wasm',
);

const version = {
  major: 1,
  minor: 0,
  patch: 0,
};

export const ego_ops_id = Principal.fromText(getCanisterId('ego_ops')!);

export const opsPostInstall = async () => {
  let opsOperator = await getOperator<EgoOpsService>('ego_ops');

  console.log(`=== post install script of ego_ops starts: ===\n`);

  console.log(`1. canister_registers\n`);
  await canister_registers();

  console.log(`2. canister_relation_update\n`);
  await opsOperator.canister_relation_update('ego_dev');
  await opsOperator.canister_relation_update('ego_file');
  await opsOperator.canister_relation_update('ego_store');
  await opsOperator.canister_relation_update('ego_tenant');
  await opsOperator.canister_relation_update('ego_ledger');
  await opsOperator.canister_relation_update('ego_ops');

  console.log(`3. canister_main_track\n`);
  await opsOperator.canister_main_track();
};

const canister_registers = async () => {
  await canister_register('ego_dev');
  await canister_register('ego_file');
  await canister_register('ego_store');
  await canister_register('ego_tenant');
  await canister_register('ego_ledger');
};

async function getOperator<T>(canisterName: string): Promise<ActorSubclass<T>> {
  let operator = await getActor<T>(
    identity,
    idlFactory,
    getCanisterId(canisterName)!,
  );
  return operator;
}

async function canister_register(canister_name: string) {
  let opsOperator = await getOperator<EgoOpsService>('ego_ops');

  let actor = await getActor<EgoOpsService>(
    identity,
    idlFactory,
    getCanisterId(canister_name)!,
  );
  let canister_operator = await actor;
  let canister_id = Principal.fromText(getCanisterId(canister_name)!);

  console.log(`==> a. add ego_ops as ${canister_name} owner\n`);
  let resp1 = await canister_operator.ego_owner_add(ego_ops_id);
  console.log(resp1);

  console.log(`==> b. register ${canister_name} to ego_ops\n`);
  let resp2 = await opsOperator.ego_canister_add(canister_name, canister_id);
  console.log(resp2);
}

export const admin_app_create = async (
  app_id: string,
  name: string,
  version: any,
  category: Category,
  deploy_mode: DeployMode,
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
    logo: '',
    description: '',
    category,
    backend_data: Array.from(new Uint8Array(backend_data)),
    backend_hash,
    frontend: frontend_canister_id ? [frontend_canister_id] : [],
    deploy_mode,
  });
  console.log(resp1);
};
