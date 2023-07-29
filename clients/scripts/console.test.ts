import {getActor, getCanisterId, identity} from "@ego-js/utils";
import {_SERVICE as EgoStoreService} from "@/idls/ego_store";
import {ActorSubclass} from "@dfinity/agent";
import {idlFactory} from "@/idls/ego_ops.idl";
import {Principal} from "@dfinity/principal";

describe('get_owners', () => {
    test('get_owners', async () => {
        let actor = await getOperator<EgoStoreService>('ego_tenant');
        let resp = await actor.ego_owner_list()

        console.log(resp.Ok)
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