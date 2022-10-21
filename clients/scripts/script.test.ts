import { getActor } from '@/settings/agent';
import { _SERVICE as EgoDevService } from '@/idls/ego_dev';
import {_SERVICE as EgoStoreService, Category} from '@/idls/ego_store';
import { _SERVICE as EgoOpsService } from '@/idls/ego_ops';
import { identity } from '@/settings/identity';
import { idlFactory as EgoDevIdlFactory} from '@/idls/ego_dev.idl';
import { idlFactory as EgoStoreIdlFactory } from '@/idls/ego_store.idl';
import { getCanisterId } from '@/settings/utils';
import {Principal} from "@dfinity/principal";
import path from "path";

export const egoDevDeployerActor = getActor<EgoDevService>(
  identity,
  EgoDevIdlFactory,
  getCanisterId('ego_dev')!,
);

export const egoStoreDeployerActor = getActor<EgoStoreService>(
  identity,
  EgoStoreIdlFactory,
  getCanisterId('ego_store')!,
);

export const egoOpsDeployerActor = getActor<EgoOpsService>(
  identity,
  EgoStoreIdlFactory,
  getCanisterId('ego_store')!,
);

describe('scripts', () => {
  test('set_auditor', async () => {
    const deployer = await egoDevDeployerActor;

    let user_names = ['aaa', 'neeboo'];

    for (const user_name of user_names) {
      console.log(`\t\t set role for ${user_name}\n`);
      let resp1 = await deployer.user_main_list({ name: user_name });
      for (const user of (resp1 as any).Ok.users) {
        let resp = await deployer.user_role_set({
          user_id: user.user_id,
          is_app_auditor: true,
          is_manager: true,
        });
        console.log(resp);
      }
    }
  });

  // test('notify_payment', async () => {
  //   const deployer = await deployerActor;

  //   let memo = 3;
  //   let resp = await deployer.notify_payment({ memo: BigInt(memo) });
  //   console.log(resp);
  // });

  test('set_wallet_provider', async () => {
    const deployer = await egoOpsDeployerActor;

    let me_v1_canister_id = Principal.fromText("q4eej-kyaaa-aaaaa-aaaha-cai");

    console.log(me_v1_canister_id);

    console.log(`\t\t set me_v1 wallet provider\n`);
    let resp1 = await deployer.admin_wallet_provider_add({ wallet_provider: me_v1_canister_id, wallet_app_id: 'astrox_controller' });
    console.log(resp1);
  });

  test('app_main_list', async () => {
    const deployer = await egoStoreDeployerActor;

    console.log(`\t\t list System app\n`);
    let resp1 = await deployer.app_main_list({ query_param: { 'ByCategory' : { 'category' : { 'System' : null } } } });
    console.log(resp1.Ok.apps);

    console.log(`\t\t list Vault app\n`);
    let resp2 = await deployer.app_main_list({ query_param: { 'ByCategory' : { 'category' : { 'Vault' : null } } } });
    console.log(resp2.Ok.apps);
  });
});