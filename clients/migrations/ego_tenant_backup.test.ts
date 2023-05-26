import { idlFactory as Idl } from '../idls/ego_tenant.idl';
import { _SERVICE as Service } from '../idls/ego_tenant';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';

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
    const len = json.length;

    const jsBuff = Buffer.from(JSON.stringify(json));
    await actor.admin_import(Array.from(jsBuff))
  });
});
