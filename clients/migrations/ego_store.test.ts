import { idlFactory as Idl } from '../idls/ego_store.idl';
import { _SERVICE as Service } from '../idls/ego_store';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';


describe('ego_store_backup', () => {
    it('export', async () => {
        const canister_name = 'ego_store'
        const storeActor = getActor<Service>(identity(), Idl, getCanisterId(canister_name)!);

        const inst = await storeActor;

        const data = await inst.admin_backup()
        const json = JSON.parse(Buffer.from(data).toString()) as { [key: string]: any }[];
        fs.writeFileSync(`${process.cwd()}/backup/datas/${canister_name}.json`, JSON.stringify(json));
    });
});
