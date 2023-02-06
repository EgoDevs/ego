import { getActor, identity as defaultIdentity, getCanisterId } from '@ego-js/utils';
import { _SERVICE as EgoDevService } from '@/idls/ego_dev';
import { _SERVICE as EgoStoreService, Category } from '@/idls/ego_store';
import { _SERVICE as EgoOpsService, AdminWalletProviderAddRequest } from '@/idls/ego_ops';

import { _SERVICE as EgoLedgerService } from '@/idls/ego_ledger';
import { _SERVICE as EgoRecordService } from '@/idls/ego_record';

import { idlFactory as EgoDevIdlFactory } from '@/idls/ego_dev.idl';
import { idlFactory as EgoStoreIdlFactory } from '@/idls/ego_store.idl';

import { idlFactory as EgoOpsIdlFactory } from '@/idls/ego_ops.idl';
import { idlFactory as EgoLedgerIdlFactory } from '@/idls/ego_ledger.idl';
import { idlFactory as EgoRecordIdlFactory } from '@/idls/ego_record.idl';

import { Principal } from '@dfinity/principal';
import path from 'path';

const identity = defaultIdentity();

export const egoDevDeployerActor = getActor<EgoDevService>(identity, EgoDevIdlFactory, getCanisterId('ego_dev')!);

export const egoStoreDeployerActor = getActor<EgoStoreService>(identity, EgoStoreIdlFactory, getCanisterId('ego_store')!);

export const egoOpsDeployerActor = getActor<EgoOpsService>(identity, EgoOpsIdlFactory, getCanisterId('ego_ops')!);

export const egoLedgerDeployerActor = getActor<EgoLedgerService>(identity, EgoLedgerIdlFactory, getCanisterId('ego_ledger')!);

export const egoRecordDeployerActor = getActor<EgoRecordService>(identity, EgoRecordIdlFactory, getCanisterId('ego_record')!);

describe('scripts', () => {
  test('set_auditor', async () => {
    const deployer = await egoDevDeployerActor;

    let user_names = ['aaa', 'neeboo'];

    for (const user_name of user_names) {
      console.log(`\t\t set role for ${user_name}\n`);
      let resp1 = await deployer.user_main_list(user_name);
      console.log(`resp1`, resp1);
      for (const user of (resp1 as any).Ok) {
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
    console.log((resp1 as any).Ok.apps);
  });

  test('cycle_info', async () => {
    const deployer = await egoStoreDeployerActor;

    console.log(`\t\t get estimate\n`);
    let resp1 = await deployer.ego_cycle_info();
    console.log(resp1);
  });

  test('get_log', async () => {
    const deployer = await egoOpsDeployerActor;

    console.log(`\t\t create an order\n`);
    let logs = deployer.ego_log_list(BigInt(26));
    console.log(logs);
  });

  // manually create an order
  test('admin_wallet_order_new', async () => {
    const deployer = await egoOpsDeployerActor;

    console.log(`\t\t create an order\n`);
    await deployer.admin_wallet_order_new(0.001);
  });

  // list all the orders
  test('admin_wallet_order_list', async () => {
    const deployer = await egoStoreDeployerActor;

    console.log(`\t\t list all the orders\n`);
    let resp = await deployer.admin_wallet_order_list();
    let orders = (resp as any).Ok;
    orders.forEach(order => {
      console.log(order);
    });
  });

  // change the start block of ego_ledger
  test('ledger_main_init', async () => {
    const deployer = await egoLedgerDeployerActor;

    console.log(`\t\t set ledger start block index\n`);
    await deployer.ledger_main_init({ start: BigInt(4789139) });
  });

  test('ledger_payment_list', async () => {
    const deployer = await egoLedgerDeployerActor;

    console.log(`\t\t ledger_payment_list\n`);
    let resp = await deployer.ledger_payment_list();
    let payments = (resp as any).Ok;
    payments.forEach(payment => {
      console.log(payment);
    });
  });

  test('flush_wallet_change_record', async () => {
    const store = await egoStoreDeployerActor;
    const record = await egoRecordDeployerActor

    console.log(`\t\t flush_wallet_change_record\n`);
    await store.flush_wallet_change_record();

    let resp = await record.record_list(BigInt(100));
    console.log(resp)
  });

  test.skip('add_ops_owner', async () => {
    const deployer = await egoOpsDeployerActor;

    let principal = Principal.fromText('replace_this');
    let resp = await deployer.ego_owner_add(principal);
    console.log(resp);
  });
});
