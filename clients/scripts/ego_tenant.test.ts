import { idlFactory as Idl } from '../idls/ego_tenant.idl';
import { _SERVICE as Service } from '../idls/ego_tenant';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';
import {Principal} from "@dfinity/principal";

const file_path = '/tmp/ego_tenant.json'

describe('ego_tenant_export', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('export', async () => {
    const actor = await egoActor;

    const data = await actor.admin_export();

    const json = JSON.parse(Buffer.from(data).toString()) as { [key: string]: any }[];
    fs.writeFileSync(file_path, JSON.stringify(json));
  });
});

describe('ego_tenant_import', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('import', async () => {
    const actor = await egoActor;

    const data = fs.readFileSync(file_path);
    const json = JSON.parse(data.toString()) as any[];

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

    console.log('3. restore tasks')
    Object.entries(json['ego_tenant']['tasks']).forEach(([key, task]) => {
      console.log(key + " / " + task)
      actor.canister_main_track(Principal.fromText(task['wallet_id']), Principal.fromText(task['canister_id']))
    })
  });
});
