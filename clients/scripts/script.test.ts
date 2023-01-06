import { getActor } from '@/settings/agent';
import { _SERVICE as EgoDevService } from '@/idls/ego_dev';
import {_SERVICE as EgoStoreService, Category} from '@/idls/ego_store';
import {_SERVICE as EgoOpsService, AdminWalletProviderAddRequest} from '@/idls/ego_ops';

import { _SERVICE as EgoLedgerService } from '@/idls/ego_ledger';
import { identity } from '@/settings/identity';
import { idlFactory as EgoDevIdlFactory} from '@/idls/ego_dev.idl';
import { idlFactory as EgoStoreIdlFactory } from '@/idls/ego_store.idl';

import { idlFactory as EgoOpsIdlFactory } from '@/idls/ego_ops.idl';
import { idlFactory as EgoOLedgerIdlFactory } from '@/idls/ego_ledger.idl';
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
  EgoOpsIdlFactory,
  getCanisterId('ego_ops')!,
);


export const egoLedgerDeployerActor = getActor<EgoLedgerService>(
  identity,
  EgoOLedgerIdlFactory,
  getCanisterId('ego_ledger')!,
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

  test('app_main_list', async () => {
    const deployer = await egoStoreDeployerActor;

    console.log(`\t\t list app\n`);
    let resp1 = await deployer.app_main_list();
    console.log(resp1.Ok.apps);
  });

  test('get_log', async () => {
    const deployer = await egoOpsDeployerActor;

    console.log(`\t\t create an order\n`);
    let logs = await deployer.ego_log_list(BigInt(Date.now() - 3600 * 24));
    console.log(logs)
  })

  // manually create an order
  test('admin_wallet_order_new', async () => {
    const deployer = await egoOpsDeployerActor;

    console.log(`\t\t create an order\n`);
    await deployer.admin_wallet_order_new(0.001);
  })

  // list all the orders
  test('admin_wallet_order_list', async () => {
    const deployer = await egoStoreDeployerActor;

    console.log(`\t\t list all the orders\n`);
    let resp = await deployer.admin_wallet_order_list();
    let orders = resp.Ok
    orders.forEach(order => {
      console.log(order)
    })
  })

  // change the start block of ego_ledger
  test('ledger_main_init', async () => {
    const deployer = await egoLedgerDeployerActor;

    console.log(`\t\t set ledger start block index\n`);
    await deployer.ledger_main_init({start: BigInt(4789139)});
  })

  test('ledger_payment_list', async () => {
    const deployer = await egoLedgerDeployerActor;

    console.log(`\t\t ledger_payment_list\n`);
    let resp = await deployer.ledger_payment_list();
    let payments = resp.Ok
    payments.forEach(payment => {
      console.log(payment)
    })
  })

  test('add_ops_owner', async () => {
    const deployer = await egoOpsDeployerActor;

    let principal = Principal.fromText("replace_this");
    let resp = await deployer.ego_owner_add(principal);
    console.log(resp)
  })
});