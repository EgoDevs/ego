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
  Result_6, Result_8, Result_9, UserMainListRequest, UserRoleSetRequest, EgoError, DeveloperMainGetResponse, DeveloperAppListResponse, AppVersionNewResponse, AdminAppCreateResponse, AppVersionWaitForAuditResponse, AppVersionApproveResponse, UserMainListResponse, UserRoleSetResponse
} from "@/../../idls/ego_dev";
import { idlFactory as devIdl } from "@/../../idls/ego_dev.idl";
import { BaseConnection } from "./base";
import { ActorSubclass, HttpAgent, Identity, SignIdentity } from "@dfinity/agent";
import { getActor } from "./base";
import { hasOwnProperty } from "@/utils";
import { message } from "antd";


export const CategoryEnum = {
  "System": {
    System: null
  },
  "Vault": {
    Vault: null
  }
}

export function handleResponse<T> (res: {Ok:T} | {Err: EgoError}, noThrow: undefined | boolean): T | EgoError {
  if(hasOwnProperty(res, 'Ok')) {
    return res['Ok'] as T
  } else {
    if(!noThrow) {
      message.error(res['Err']['msg'])
    }
    return res['Err']
  }
  
}


export async function handleResponseCatch<T, A> (fn: () => Promise<A>, noThrow?: boolean): Promise<T | EgoError | void> {
  try {
    const response = await fn()
    return handleResponse<T>(response as any, noThrow);
  } catch (err) {
    console.log(err)
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
  async developer_main_register({ name }: DeveloperMainRegisterRequest): Promise<DeveloperMainGetResponse> {
    const result = await handleResponseCatch<DeveloperMainGetResponse, Result_6>(() => this._actor.developer_main_register({ name, })) as DeveloperMainGetResponse;
    console.log('developer', result)
    return result;
  }

  /********************  developer  ********************/
  async developer_main_get(): Promise<DeveloperMainGetResponse> {
    const result = await handleResponseCatch<DeveloperMainGetResponse, Result_6>(() => this._actor.developer_main_get(), true) as DeveloperMainGetResponse;
    console.log('developer_main_get', result)
    return result;
  }

  async developer_app_list(): Promise<DeveloperAppListResponse> {
    const result = await handleResponseCatch<DeveloperAppListResponse, Result_5>(() => this._actor.developer_app_list()) as DeveloperAppListResponse;
    console.log('developer_app_list', result)
    return result;
  }

  async developer_app_get({ app_id }: AppMainGetRequest): Promise<DeveloperAppListResponse> {
    const result = await handleResponseCatch<DeveloperAppListResponse, Result_4>(() => this._actor.developer_app_get({ app_id })) as DeveloperAppListResponse;
    return result;
  }

  async developer_app_new({ name, app_id, logo, description, category, price }: AppMainNewRequest): Promise<DeveloperAppListResponse> {
    const result = await handleResponseCatch<DeveloperAppListResponse, Result_4>(() => this._actor.developer_app_new({ name, app_id, logo, description, category, price })) as DeveloperAppListResponse;
    return result;
  }

  async app_version_new({ version, app_id }: AppVersionNewRequest): Promise<AppVersionNewResponse> {
    const result = await handleResponseCatch<AppVersionNewResponse, Result_2>(() => this._actor.app_version_new({ version, app_id })) as AppVersionNewResponse;
    return result;
  }

  async app_version_set_frontend_address({ canister_id, app_id, version }: AppVersionSetFrontendAddressRequest): Promise<AdminAppCreateResponse> {
    const result = await handleResponseCatch<AdminAppCreateResponse, Result>(() => this._actor.app_version_set_frontend_address({ canister_id, app_id, version })) as AdminAppCreateResponse;
    return result;
  }

  async app_version_upload_wasm({ app_id, data, hash, version }: AppVersionUploadWasmRequest): Promise<AdminAppCreateResponse> {
    const result = await handleResponseCatch<AdminAppCreateResponse, Result>(() => this._actor.app_version_upload_wasm({ app_id, data, hash, version })) as AdminAppCreateResponse;
    return result;
  }

  async app_version_submit({ version, app_id }: AppVersionNewRequest): Promise<AppVersionNewResponse> {
    const result = await handleResponseCatch<AppVersionNewResponse, Result_2>(() => this._actor.app_version_submit({ version, app_id })) as AppVersionNewResponse;
    return result;
  }

  async app_version_revoke({ version, app_id }: AppVersionNewRequest): Promise<AppVersionNewResponse> {
    const result = await handleResponseCatch<AppVersionNewResponse, Result_2>(() => this._actor.app_version_revoke({ version, app_id })) as AppVersionNewResponse;
    return result;
  }

  async app_version_release({ version, app_id }: AppVersionNewRequest): Promise<AppVersionNewResponse> {
    const result = await handleResponseCatch<AppVersionNewResponse, Result_2>(() => this._actor.app_version_release({ version, app_id })) as AppVersionNewResponse;

    return result;
  }

  /********************  auditor  ********************/
  async app_version_wait_for_audit(): Promise<AppVersionWaitForAuditResponse> {
    const result = await handleResponseCatch<AppVersionWaitForAuditResponse, Result_3>(() => this._actor.app_version_wait_for_audit()) as AppVersionWaitForAuditResponse;
    return result;
  }

  async app_version_approve({ version, app_id }: AppVersionApproveRequest): Promise<AppVersionApproveResponse> {
    const result = await handleResponseCatch<AppVersionApproveResponse, Result_1>(() => this._actor.app_version_approve({ version, app_id })) as AppVersionApproveResponse;
    return result;
  }

  async app_version_reject({ version, app_id }: AppVersionNewRequest): Promise<AppVersionNewResponse> {
    const result = await handleResponseCatch<AppVersionNewResponse, Result_2>(() => this._actor.app_version_reject({ version, app_id })) as AppVersionNewResponse;
    return result;
  }

  /********************  manager  ********************/

  async user_main_list({name}: UserMainListRequest): Promise<UserMainListResponse> {
    const result = await handleResponseCatch<UserMainListResponse, Result_8>(() => this._actor.user_main_list({ name })) as UserMainListResponse;
    return result;
  }

  async user_role_set({ user_id, is_app_auditor, is_manager }: UserRoleSetRequest): Promise<UserRoleSetResponse> {
    const result = await handleResponseCatch<UserRoleSetResponse, Result_9>(() => this._actor.user_role_set({ user_id, is_app_auditor, is_manager })) as UserRoleSetResponse;
    return result;
  }
}