import {getActor, getCanisterId, identity} from "@ego-js/utils";
import { _SERVICE as EgoDevService } from '@/idls/ego_dev';
import { idlFactory } from '@/idls/ego_dev.idl';
import {ActorSubclass} from "@dfinity/agent";
import {Principal} from "@dfinity/principal";

describe('fix', () => {
    test('fix', async () => {
        let actor = await getOperator<EgoDevService>('ego_dev');
        await actor.ego_canister_add('ego_file', Principal.fromText(getCanisterId("ego_file")!));
        await actor.ego_canister_add('ego_store', Principal.fromText(getCanisterId("ego_store")!));
        await actor.ego_canister_add('ego_tenant', Principal.fromText(getCanisterId("ego_tenant")!));
        await actor.ego_canister_add('ego_record', Principal.fromText(getCanisterId("ego_record")!));
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