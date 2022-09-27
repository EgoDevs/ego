import {
  _SERVICE,
  AppMainGetRequest,
  AppMainNewRequest,
  AppVersionApproveRequest,
  AppVersionNewRequest,
  AppVersionSetFrontendAddressRequest, AppVersionUploadWasmRequest,
  DeveloperMainRegisterRequest,
  Result, Result_1,
  Result_2,
  Result_3,
  Result_4,
  Result_5,
  Result_6, Result_8, Result_9, UserMainListRequest, UserRoleSetRequest
} from "@/../../idls/ego_dev";
import { idlFactory as devIdl } from "@/../../idls/ego_dev.idl";
import { BaseConnection } from "./base";
import { ActorSubclass, HttpAgent, Identity, SignIdentity } from "@dfinity/agent";
import { getActor } from "./base";


export const CategoryEnum = {
  "System": {
    System: null
  },
  "Vault": {
    Vault: null
  }
}

export class DevConnection extends BaseConnection<_SERVICE> {
  constructor(
    public identity: Identity,
    public actor: ActorSubclass<_SERVICE>,
    public agent: HttpAgent,
  ) {
    super(identity, process.env.EGO_DEV_CANISTERID!, devIdl, actor, agent);

  }
  static async create(identity: Identity): Promise<DevConnection> {
    const { actor, agent } = await getActor<_SERVICE>(devIdl, process.env.EGO_DEV_CANISTERID!, identity);
    return new DevConnection(identity, actor, agent);
  }

  /********************  anonymous  ********************/
  async developer_main_register({ name }: DeveloperMainRegisterRequest): Promise<Result_6> {
    const result = await this._actor.developer_main_register({ name, });
    return result;
  }

  /********************  developer  ********************/
  async developer_main_get(): Promise<Result_6> {
    const result = await this._actor.developer_main_get();
    return result;
  }

  async developer_app_list(): Promise<Result_5> {
    const result = await this._actor.developer_app_list();
    return result;
  }

  async developer_app_get({ app_id }: AppMainGetRequest): Promise<Result_4> {
    const result = await this._actor.developer_app_get({ app_id });
    return result;
  }

  async developer_app_new({ name, app_id, logo, description, category, price }: AppMainNewRequest): Promise<Result_4> {
    const result = await this._actor.developer_app_new({ name, app_id, logo, description, category, price });
    return result;
  }

  async app_version_new({ version, app_id }: AppVersionNewRequest): Promise<Result_2> {
    const result = await this._actor.app_version_new({ version, app_id });
    return result;
  }

  async app_version_set_frontend_address({ canister_id, app_id, version }: AppVersionSetFrontendAddressRequest): Promise<Result> {
    const result = await this._actor.app_version_set_frontend_address({ canister_id, app_id, version });
    return result;
  }

  async app_version_upload_wasm({ app_id, data, hash, version }: AppVersionUploadWasmRequest): Promise<Result> {
    const result = await this._actor.app_version_upload_wasm({ app_id, data, hash, version });
    return result;
  }

  async app_version_submit({ version, app_id }: AppVersionNewRequest): Promise<Result_2> {
    const result = await this._actor.app_version_submit({ version, app_id });
    return result;
  }

  async app_version_revoke({ version, app_id }: AppVersionNewRequest): Promise<Result_2> {
    const result = await this._actor.app_version_revoke({ version, app_id });
    return result;
  }

  async app_version_release({ version, app_id }: AppVersionNewRequest): Promise<Result_2> {
    const result = await this._actor.app_version_release({ version, app_id });
    return result;
  }

  /********************  auditor  ********************/
  async app_version_wait_for_audit(): Promise<Result_3> {
    const result = await this._actor.app_version_wait_for_audit();
    return result;
  }

  async app_version_approve({ version, app_id }: AppVersionApproveRequest): Promise<Result_1> {
    const result = await this._actor.app_version_approve({ version, app_id });
    return result;
  }

  async app_version_reject({ version, app_id }: AppVersionNewRequest): Promise<Result_2> {
    const result = await this._actor.app_version_reject({ version, app_id });
    return result;
  }

  /********************  manager  ********************/

  async user_main_list({name}: UserMainListRequest): Promise<Result_8> {
    const result = await this._actor.user_main_list({ name });
    return result;
  }

  async user_role_set({ user_id, is_app_auditor, is_manager }: UserRoleSetRequest): Promise<Result_9> {
    const result = await this._actor.user_role_set({ user_id, is_app_auditor, is_manager });
    return result;
  }
}