import { idlFactory as Idl } from '../idls/ego_store.idl';
import { _SERVICE as Service } from '../idls/ego_store';

import { getActor, identity, getCanisterId } from '@ego-js/utils';

import fs from 'fs';
import {data_import} from "@/migrations/utils";


describe('ego_store_backup', () => {
    it('export', async () => {
        const canister_name = 'ego_store'
        const storeActor = getActor<Service>(identity(), Idl, getCanisterId(canister_name)!);

        const inst = await storeActor;

        const data = await inst.admin_backup()
        console.log(data)
        fs.writeFileSync(`${process.cwd()}/backup/datas/${canister_name}.json`, data);
    });
});

describe('ego_store_restore', () => {
    it('import', async () => {
        const canister_name = 'ego_store'
        const storeActor = getActor<Service>(identity(), Idl, getCanisterId(canister_name)!);
        const inst = await storeActor;

        const data = fs.readFileSync(`${process.cwd()}/backup/datas/${canister_name}.json`);
        const result = await inst.admin_restore(new Uint8Array(data));
        console.log(`written bytes ${data.length}`);

        console.log({ result });
    });
});
