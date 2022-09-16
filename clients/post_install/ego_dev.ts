import {getCanisterId} from '@/settings/utils';
import {getActor} from '@/settings/agent';
import {idlFactory as devIdl} from '@/idls/ego_dev.idl';
import {_SERVICE} from '@/idls/ego_dev';
import {identity} from '@/settings/identity';
import {Principal} from '@dfinity/principal';
import {Ed25519KeyIdentity} from "@dfinity/identity";
import {admins, developers, operators} from "@/fixtures/identities";
import fs from "fs";
import crypto, {BinaryLike} from "crypto";

export const operatorActor = getActor<_SERVICE>(
  identity,
  devIdl,
  getCanisterId('ego_dev')!,
);

export const developerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(developers[0]?.identity)),
  devIdl,
  getCanisterId('ego_dev')!,
);

export const auditerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(operators[0]?.identity)),
  devIdl,
  getCanisterId('ego_dev')!,
);

export const managerActor = getActor<_SERVICE>(
  Ed25519KeyIdentity.fromJSON(JSON.stringify(admins[0]?.identity)),
  devIdl,
  getCanisterId('ego_dev')!,
);


export const assets_storage_id = getCanisterId('assets_storage');

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

export const devPostInstall = async () => {
  let operator = await operatorActor


  console.log(`=== post install script of ego_dev starts: ===\n`);

  console.log(`1.add ego_file to ego_dev`);
  let resp1 = await operator.admin_file_add({
    file_id: Principal.fromText(getCanisterId('ego_file')!),
  });
  console.log(resp1)

  console.log(`2. register users\n`);
  await register_users()

  console.log(`3. create apps\n`);
  await create_app(
    'ego_assets',
    { System: null },
    'ego_assets',
    1,
    version,
    ego_assets_wasm,
  );

  // await create_app(
  //   'app_1',
  //   { Vault: null },
  //   'app_1',
  //   1,
  //   version,
  //   app_1_wasm,
  //   Principal.fromText(assets_storage_id!),
  // );
};

const register_users = async () => {
  let developer = await developerActor;
  let auditer = await auditerActor;
  let manager = await managerActor;

  await developer_main_register(
    developer,
    'developer',
    developers[0]?.principal,
    false,
    false,
  );
  await developer_main_register(
    auditer,
    'auditer',
    operators[0]?.principal,
    true,
    false,
  );
  await developer_main_register(
    manager,
    'manager',
    admins[0]?.principal,
    false,
    true,
  );
}

const developer_main_register = async (
  actor,
  name,
  principal,
  is_app_auditer,
  is_manager,
) => {
  const operator = await operatorActor;

  console.log(`\t\t a.register user ${name}\n`);

  let resp1 = await actor.developer_main_register({
    name: name,
  });
  console.log(resp1);

  console.log(`\t\t b.set role for ${name}\n`);
  let resp2 = await operator.user_role_set({
    user_id: Principal.fromText(principal),
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

  let resp1 = await developer.developer_app_new({
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
  const auditer = await auditerActor;

  console.log(`\t\t b.new version for ${app_id}\n`);
  let resp_b = await developer.app_version_new({
    version,
    app_id,
  });

  console.log(resp_b)

  const fileMd5 = crypto
    .createHash('md5')
    .update(wasm_file as BinaryLike)
    .digest('hex');

  console.log(`\t\t c.1.upload wasms for ${app_id}\n`);
  let resp_c_1 = await developer.app_version_upload_wasm({
    app_id,
    version,
    data: Array.from(wasm_file),
    hash: fileMd5
  });

  console.log(resp_c_1);

  if (frontend_canister_id) {
    console.log(`\t\t c.2.set frontend address for ${app_id}\n`);

    let resp_c_2 = await developer.app_version_set_frontend_address({
      app_id: app_id,
      version: version,
      canister_id: frontend_canister_id,
    });

    console.log(resp_c_2);
  }

  console.log(`\t\t d.submit version for ${app_id}\n`);
  let resp_d = await developer.app_version_submit({
    version,
    app_id,
  });
  console.log(resp_d);

  console.log(`\t\t e.approve version for ${app_id}\n`);
  let resp_e = await auditer.app_version_approve({
    version,
    app_id,
  });
  console.log(resp_e);

  console.log(`\t\t f.release version for ${app_id}\n`);
  let resp_f = await developer.app_version_release({
    version,
    app_id,
  });
  console.log(resp_f);
};

