import { idlFactory } from '@/idls/ego_store.idl';
import { _SERVICE as EgoStoreService } from '@/idls/ego_store';
import { getActor } from '@/settings/agent';
import { identity } from '@/settings/identity';
import { getCanisterId } from '@/settings/utils';
import { ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import fs from 'fs';
import path from 'path';
import { isProduction } from '@/settings/env';
import { admin_app_create } from './ego_ops';

async function getStoreActor<T>(
  canisterName: string,
): Promise<ActorSubclass<T>> {
  let actor = await getActor<T>(
    identity,
    idlFactory,
    getCanisterId(canisterName)!,
  );
  return actor;
}

export const egoStoreCanisterId = Principal.fromText(
  getCanisterId('ego_store')!,
);

const astrox_controller_wasm = fs.readFileSync(
  path.resolve(
    `${[process.cwd()]}` +
      '../../MePlus/artifacts/astrox_controller/astrox_controller_opt.wasm',
  ),
);

const astrox_controller_version = {
  major: 1,
  minor: 0,
  patch: 0,
};

const omni_wallet_wasm = fs.readFileSync(
  path.resolve(
    `${[process.cwd()]}` +
      '../../MePlus/artifacts/omni_wallet/omni_wallet_opt.wasm',
  ),
);

const omni_wallet_version = {
  major: 1,
  minor: 0,
  patch: 0,
};

export const storePostInstall = async () => {
  await admin_app_create(
    'astrox_controller',
    'astrox_controller',
    astrox_controller_version,
    { System: null },
    { DEDICATED: null },
    astrox_controller_wasm,
  );

  console.log(`4. release omni_wallet canister\n`);
  await admin_app_create(
    'omni_wallet',
    'omni_wallet',
    omni_wallet_version,
    { Vault: null },
    { DEDICATED: null },
    omni_wallet_wasm,
  );

  if (!isProduction) {
    const actor = await getStoreActor<EgoStoreService>('ego_store');
    const jsonFile = JSON.parse(
      fs
        .readFileSync(
          path.resolve(
            `${[process.cwd()]}` + '../../me_v1/.dfx/local/canister_ids.json',
          ),
        )
        .toString(),
    );

    await actor.admin_wallet_provider_add({
      wallet_app_id: 'astrox_controller',
      wallet_provider: Principal.fromText(jsonFile['me_v1']['local']),
    });
  } else {
    const actor = await getStoreActor<EgoStoreService>('ego_store');
    const jsonFile = JSON.parse(
      fs
        .readFileSync(
          path.resolve(`${[process.cwd()]}` + '../../me_v1/canister_ids.json'),
        )
        .toString(),
    );

    await actor.admin_wallet_provider_add({
      wallet_app_id: 'astrox_controller',
      wallet_provider: Principal.fromText(jsonFile['me_v1']['ic']),
    });
  }
};
