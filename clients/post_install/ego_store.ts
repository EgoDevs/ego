import { idlFactory } from '@/idls/ego_store.idl';
import { _SERVICE as EgoStoreService } from '@/idls/ego_store';
import { getActor } from '@/settings/agent';
import { identity } from '@/settings/identity';
import { getCanisterId } from '@/settings/utils';
import { ActorSubclass } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import fs from 'fs';
import path from 'path';

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

export const storePostInstall = async () => {
  const actor = await getStoreActor<EgoStoreService>('ego_store');

  const jsonFile = JSON.parse(
    fs
      .readFileSync(
        path.resolve(
          `${[process.cwd()]}` + '../../../me_v1/.dfx/local/canister_ids.json',
        ),
      )
      .toString(),
  );

  await actor.admin_wallet_provider_add({
    wallet_app_id: 'astrox_controller',
    wallet_provider: Principal.fromText(jsonFile['me_v1']['local']),
  });
};
