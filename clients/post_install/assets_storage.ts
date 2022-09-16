import {getActor} from '@/settings/agent';
import {identity} from '@/settings/identity';

import {getCanisterId} from '@/settings/utils';
import {idlFactory} from '@/idls/assets_storage.idl';
import {_SERVICE} from '@/idls/assets_storage';
import {Principal} from '@dfinity/principal';
import {ManagementApi} from '@/settings/manager';

export const assetsActor = getActor<_SERVICE>(
    identity,
    idlFactory,
    getCanisterId('assets_storage')!,
);

export const assetsStorageInstall = async () => {
    let assets = await assetsActor;
    console.log(`=== post install script of assets_storage starts: ===\n`);

    console.log(`1. authoried ${identity.getPrincipal().toText()}`);
    let resp1 = await assets.authorize(Principal.fromText(getCanisterId('ego_dev')!));
    console.log(resp1);

    console.log(`2. add ego_dev as controller`);

    let resp2 = await ManagementApi.updateSettings(
        'assets_storage',
        getCanisterId('assets_storage')!,
        {
            freezing_threshold: [],
            controllers: [[identity.getPrincipal(), Principal.fromText(getCanisterId('ego_dev')!)]],
            memory_allocation: [],
            compute_allocation: [],
        },
    );
    console.log(resp2)
};
