import { getActor, getCanisterId, identity } from "@ego-js/utils";
import { _SERVICE as Service } from "@/idls/ego_tenant";
import { idlFactory } from "@/idls/ego_tenant.idl";
import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";

describe("add_controller", () => {
  test("add_controller", async () => {
    let actor = await getOperator<Service>("ego_tenant");
    await actor.delegate_controller_add(
      Principal.fromText("duh7p-aaaaa-aaaai-acmlq-cai"),
      Principal.fromText("clpmy-dyaaa-aaaai-acmpa-cai")
    );
  });

  test("remove_controller", async () => {
    let actor = await getOperator<Service>("ego_tenant");
    await actor.delegate_controller_remove(
      Principal.fromText("duh7p-aaaaa-aaaai-acmlq-cai"),
      Principal.fromText("clpmy-dyaaa-aaaai-acmpa-cai")
    );
  });
});

async function getOperator<T>(canisterName: string): Promise<ActorSubclass<T>> {
  let operator = await getActor<T>(
    identity(),
    idlFactory,
    getCanisterId(canisterName)!
  );
  return operator;
}
