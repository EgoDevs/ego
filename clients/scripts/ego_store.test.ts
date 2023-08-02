import { idlFactory as Idl } from '../idls/ego_store.idl';
import { _SERVICE as Service } from '../idls/ego_store';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';
import {Principal} from "@dfinity/principal";

const file_path = '/tmp/ego_store.json'

describe('ego_store_export', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_store')!);

  it('export', async () => {
    const actor = await egoActor;

    const data = await actor.admin_export();

    const json = JSON.parse(Buffer.from(data).toString()) as { [key: string]: any }[];

    fs.writeFileSync(file_path, JSON.stringify(json));
  });
});

describe('ego_store_import', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_store')!);

  it('import', async () => {
    const actor = await egoActor;

    const data = fs.readFileSync(file_path);
    const json = JSON.parse(data.toString()) as any[];

    let wallet_offset = 0

    console.log('1. restore users')

    Object.entries(json['users']['owners']).forEach(([key, value]) => {
      actor.ego_owner_add(Principal.fromText(value))
    })

    Object.entries(json['users']['users']).forEach(([key, value]) => {
      actor.ego_user_add(Principal.fromText(value))
    })

    Object.entries(json['users']['ops']).forEach(([key, value]) => {
      actor.ego_op_add(Principal.fromText(value))
    })

    console.log('2. restore registry')
    Object.entries(json['registry']['canisters']).forEach(([key, value]) => {
      actor.ego_canister_add(key, Principal.fromText(value + ''))
    })

    console.log('3. restore apps')
    Object.entries(json['ego_store']['apps']).forEach(([key, value]) => {
      let category = {}
      category[value['app']['category']] = null
      value['app']['category'] = category

      value['wasm']['canister_id'] = Principal.fromText(value['wasm']['canister_id'])
      value['wasm']['canister_type'] = {'BACKEND': null}
      value['last_update'] = 0
      actor.app_main_release(value)
    })

    console.log('4. wallet providers')
    Object.entries(json['ego_store']['wallet_providers']).forEach(([key, value]) => {
      value['wallet_provider'] = Principal.fromText(value['wallet_provider'])
      value['wallet_app_id'] = value['app_id']
      actor.admin_wallet_provider_add(value)
    })

    console.log('5. wallets')
    let wallets = json['ego_store']['wallets']
    let walls = []

    for (let wallet of Object.values(wallets)) {
      wallet['user_id'] = Principal.fromText(wallet['user_id'])
      wallet['tenant_id'] = Principal.fromText(wallet['tenant_id'])
      wallet['wallet_id'] = Principal.fromText(wallet['wallet_id'])

      let apps = wallet['apps']
      let user_apps = []

      for (let user_app of Object.values(apps)) {
        let category = {}
        category[user_app['app']['category']] = null
        user_app['app']['category'] = category

        let canister_type = {}
        canister_type[user_app['canister']['canister_type']] = null
        user_app['canister']['canister_type'] = canister_type
        user_app['canister']['canister_id'] = Principal.fromText(user_app['canister']['canister_id'])

        user_app['wallet_id'] = []
        user_apps.push(user_app)
      }
      wallet['user_apps'] = user_apps

      let cash_flowes = wallet['cash_flowes']
      let cash_flows = []
      for (let cash_flow of cash_flowes) {
        let cash_flow_type = {}
        cash_flow_type[cash_flow['cash_flow_type']] = null
        cash_flow['cash_flow_type'] = cash_flow_type

        cash_flow['operator'] = Principal.fromText(cash_flow['operator'])
        cash_flows.push(cash_flow)
      }
      wallet['cash_flows'] = cash_flows

      walls.push(wallet)
    }
    await actor.admin_import(walls)
  });
});