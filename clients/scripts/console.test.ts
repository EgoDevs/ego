import {getActor, getCanisterId, identity} from "@ego-js/utils";
import {_SERVICE as EgoStoreService} from "@/idls/ego_store";
import {ActorSubclass} from "@dfinity/agent";
import {idlFactory} from "@/idls/ego_ops.idl";
import {Principal} from "@dfinity/principal";

describe('admin_wallet_provider_add', () => {
    test('admin_wallet_provider_add', async () => {
        let actor = await getOperator<EgoStoreService>('ego_store');
        let resp = await actor.admin_wallet_provider_add({wallet_provider: Principal.fromText("q3fc5-haaaa-aaaaa-aaahq-cai"), wallet_app_id: "card_controller"})
        console.log(resp)
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