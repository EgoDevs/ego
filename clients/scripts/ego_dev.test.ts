import { idlFactory as Idl } from '../idls/ego_dev.idl';
import { _SERVICE as Service } from '../idls/ego_dev';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';
import {Principal} from "@dfinity/principal";

const file_path = '/tmp/ego_dev.json'

describe('ego_dev_export', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_dev')!);

  it('export', async () => {
    const actor = await egoActor;

    const data = await actor.admin_export();

    const json = JSON.parse(Buffer.from(data).toString()) as { [key: string]: any }[];
    fs.writeFileSync(file_path, JSON.stringify(json));
  });
});

describe('ego_dev_import', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_dev')!);

  it('import', async () => {
    const actor = await egoActor;

    const data = fs.readFileSync(file_path);
    const json = JSON.parse(data.toString()) as any[];

    const task_offset = 0

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
      console.log(key + " / " + value)
      actor.ego_canister_add(key, Principal.fromText(value + ''))
    })

    console.log('3. restore ego_dev_apps')
    let ego_dev = json['ego_dev']

    ego_dev['ego_files'].forEach((ego_file) => {
      ego_file['canister_id'] = Principal.fromText(ego_file['canister_id'])
    })

    let developers = []
    Object.entries(ego_dev['developers']).forEach(([_, developer]) => {
      developer['developer_id'] = Principal.fromText(developer['developer_id'])
      developer['last_update'] = 0
      developers.push(developer)
    })
    ego_dev['developers'] = developers;

    let apps = []
    Object.entries(ego_dev['apps']).forEach(([_, app]) => {
      let category = {}
      category[app['app']['category']] = null
      app['app']['category'] = category

      app['developer_id'] = Principal.fromText(app['developer_id'])
      app['audit_version'] = app['audit_version'] ? [app['audit_version']] : []

      app['last_update'] = 0

      app['versions'].forEach((app_version) => {
        app_version['id'] = 0

        let status = {}
        status[app_version['status']] = null
        app_version['status'] = status

        app_version['file_id'] = Principal.fromText(app_version['file_id'])

        app_version['wasm']['canister_id'] = Principal.fromText(app_version['wasm']['canister_id'])

        let canister_type = {}
        canister_type[app_version['wasm']['canister_type']] = null
        app_version['wasm']['canister_type'] = canister_type

        app_version['wasm'] = [app_version['wasm']]

        app_version['last_update'] = 0
      })

      apps.push(app)
    })
    ego_dev['apps'] = apps;

    await actor.admin_import(ego_dev)
  });
});
