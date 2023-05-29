import { idlFactory as Idl } from '../idls/ego_store.idl';
import { _SERVICE as Service } from '../idls/ego_store';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';

const file_path = '/tmp/ego_store.json'

describe('ego_store_export', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_store')!);

  it('export', async () => {
    const actor = await egoActor;

    const data = await actor.admin_backup();

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
    const len = json.length;

    const jsBuff = Buffer.from(JSON.stringify(json));
    await actor.admin_restore(Array.from(jsBuff))
  });
});
