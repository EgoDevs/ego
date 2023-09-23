import {getActor, getCanisterId, identity} from "@ego-js/utils";
import { _SERVICE as Service } from '@/idls/ego_store';
import { idlFactory } from '@/idls/ego_store.idl';
import {ActorSubclass} from "@dfinity/agent";
import {Principal} from "@dfinity/principal";

describe('add_owner', () => {
    test('fix', async () => {
        let actor = await getOperator<Service>('ego_dev');
        await actor.ego_owner_add(Principal.fromText('rdmx6-jaaaa-aaaaa-aaadq-cai'))

    });
});

async function getOperator<T>(canisterName: string): Promise<ActorSubclass<T>> {
    let operator = await getActor<T>(
      identity(),
      idlFactory,
      getCanisterId(canisterName)!,
    );
    return operator;
}