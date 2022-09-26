import {
  ApproveAppVersionRequest,
  Category,
  GetAppRequest,
  ListUserRequest,
  NewAppVersionRequest, RegisterAppRequest, Result, Result_10, Result_2, Result_3, Result_4, Result_5, Result_6, Result_7, Result_8, Result_9, SetFrontendAddressRequest, SetRoleRequest, Version, _SERVICE
} from "@/canisters/ego_store";
import { idlFactory as storeIdl } from "@/canisters/ego_store.idl";
import { BaseConnection } from "./base";
import { ActorSubclass, HttpAgent, SignIdentity } from "@dfinity/agent";
import { getActor } from "./base";


export const CategoryEnum = {
  "System": {
    System: null
  },
  "Vault": {
    Vault: null
  }
}

export class StoreConnection extends BaseConnection<_SERVICE> {
  constructor(
    public identity: SignIdentity,
    public actor: ActorSubclass<_SERVICE>,
    public agent: HttpAgent,
  ) {
    super(identity, process.env.EGO_STORE_CANISTERID!, storeIdl, actor, agent);

  }
  static async create(identity: SignIdentity): Promise<StoreConnection> {
    const { actor, agent } = await getActor(storeIdl, process.env.EGO_STORE_CANISTERID!, identity);
    return new StoreConnection(identity, actor, agent);
  }

  async approve_app_version({ version, app_id }: ApproveAppVersionRequest): Promise<Result> {
    const result = await this._actor.approve_app_version({ version, app_id });
    return result;
  }

  async created_apps(): Promise<Result_2> {
    const result = await this._actor.created_apps();
    return result;
  }

  async list_wait_for_audit_app(): Promise<Result_2> {
    const result = await this._actor.list_wait_for_audit_app();
    return result;
  }


  async get_app({ app_id }: GetAppRequest): Promise<Result_3> {
    const result = await this._actor.get_app({ app_id });
    return result;
  }

  async get_wallet(): Promise<Result_4> {
    const result = await this._actor.get_wallet();
    return result;
  }


  async list_user({
    name
  }: ListUserRequest
  ): Promise<Result_8> {
    const result = await this._actor.list_user({ name });
    return result;
  }

  async list_app(
    category: Category,
  ): Promise<Result_6> {
    const result = await this._actor.list_app({ query_param: { ByCategory: { category } } });
    return result;
  }

  async list_orders(): Promise<Result_7> {
    const result = await this._actor.list_orders();
    return result;
  }

  async me(): Promise<Result_9> {
    const result = await this._actor.me();
    return result;
  }


  async new_app_version({ version, app_id }: NewAppVersionRequest): Promise<Result_5> {
    const result = await this._actor.new_app_version({ version, app_id });
    return result;
  }

  async notify_payment(
    memo: bigint
  ): Promise<Result_5> {
    const result = await this._actor.notify_payment({ memo });
    return result;
  }

  async register_app({ name, app_id, category, price }: RegisterAppRequest): Promise<Result_3> {
    const result = await this._actor.register_app({ name, app_id, category, price });
    return result;
  }

  async register_developer({ name }: ListUserRequest): Promise<Result_9> {
    const result = await this._actor.register_developer({ name, });
    return result;
  }

  async register_user({ name }: ListUserRequest): Promise<Result_9> {
    const result = await this._actor.register_user({ name, });
    return result;
  }

  async set_role({ user_id, is_app_auditer, is_app_developer, is_manager }: SetRoleRequest): Promise<Result_10> {
    const result = await this._actor.set_role({ user_id, is_app_auditer, is_app_developer, is_manager });
    return result;
  }

  async set_frontend_address({ canister_id, app_id, version }: SetFrontendAddressRequest): Promise<Result_5> {
    const result = await this._actor.set_frontend_address({ canister_id, app_id, version });
    return result;
  }



  async reject_app_version({ version, app_id }: NewAppVersionRequest): Promise<Result_5> {
    const result = await this._actor.reject_app_version({ version, app_id });
    return result;
  }

  async release_app_version({ version, app_id }: NewAppVersionRequest): Promise<Result_5> {
    const result = await this._actor.release_app_version({ version, app_id });
    return result;
  }

  async revoke_app_version({ version, app_id }: NewAppVersionRequest): Promise<Result_5> {
    const result = await this._actor.revoke_app_version({ version, app_id });
    return result;
  }

  async submit_app_version({ version, app_id }: NewAppVersionRequest): Promise<Result_5> {
    const result = await this._actor.submit_app_version({ version, app_id });
    return result;
  }



}