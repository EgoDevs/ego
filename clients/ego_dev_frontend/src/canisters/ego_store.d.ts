import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface App {
  'status' : AppStatus,
  'bucket_id' : Principal,
  'name' : string,
  'user_id' : Principal,
  'app_id' : string,
  'release_version' : [] | [Version],
  'category' : Category,
  'price' : number,
  'versions' : Array<AppVersion>,
  'audit_version' : [] | [Version],
}
export interface AppInfo {
  'name' : string,
  'user_id' : Principal,
  'app_id' : string,
  'release_version' : [] | [Version],
  'category' : Category,
  'price' : number,
}
export type AppStatus = { 'NEW' : null } |
  { 'CLOSED' : null } |
  { 'RELEASED' : null };
export interface AppVersion {
  'status' : AppVersionStatus,
  'bucket_id' : Principal,
  'version' : Version,
  'app_id' : string,
  'wasms' : Array<Wasm>,
}
export type AppVersionStatus = { 'NEW' : null } |
  { 'REJECTED' : null } |
  { 'SUBMITTED' : null } |
  { 'REVOKED' : null } |
  { 'RELEASED' : null } |
  { 'APPROVED' : null };
export interface ApproveAppVersionRequest {
  'version' : Version,
  'app_id' : string,
}
export interface ApproveAppVersionResponse { 'ret' : boolean }
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export type Category = { 'System' : null } |
  { 'Vault' : null };
export interface CreateOrderRequest { 'app_id' : string }
export interface CreateOrderResponse { 'order' : Order }
export interface CreateRechargeRequest { 'amount' : number }
export interface CreatedAppResponse { 'apps' : Array<App> }
export interface EgoError { 'msg' : string, 'code' : number }
export interface GetAppRequest { 'app_id' : string }
export interface GetAppResponse { 'app' : App }
export interface GetWalletResponse { 'wallet_id' : [] | [Principal] }
export interface InitStoreRequest {
  'ego_crond_id' : Principal,
  'ego_bucket_id' : Principal,
  'ego_ledger_id' : Principal,
}
export interface InitStoreResponse { 'ret' : boolean }
export interface ListAppRequest { 'query_param' : QueryParam }
export interface ListAppResponse { 'apps' : Array<AppInfo> }
export interface ListOrdersResponse { 'orders' : Array<Order> }
export interface ListUserRequest { 'name' : string }
export interface ListUserResponse { 'users' : Array<User> }
export interface MeResponse { 'user' : User }
export interface NewAppVersionRequest { 'version' : Version, 'app_id' : string }
export interface NotifyPaymentRequest { 'memo' : bigint }
export interface Order {
  'to' : Array<number>,
  'status' : OrderStatus,
  'from' : Array<number>,
  'memo' : bigint,
  'user_id' : Principal,
  'app_id' : [] | [string],
  'order_type' : OrderType,
  'amount' : number,
}
export type OrderStatus = { 'NEW' : null } |
  { 'SUCCESS' : null };
export type OrderType = { 'APP' : null } |
  { 'RECHARGE' : null };
export type QueryParam = { 'ByCategory' : { 'category' : Category } };
export interface RegisterAppRequest {
  'name' : string,
  'app_id' : string,
  'category' : Category,
  'price' : number,
}
export type Result = { 'Ok' : ApproveAppVersionResponse } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : CreateOrderResponse } |
  { 'Err' : EgoError };
export type Result_10 = { 'Ok' : SetRoleResponse } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : CreatedAppResponse } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : GetAppResponse } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : GetWalletResponse } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : InitStoreResponse } |
  { 'Err' : EgoError };
export type Result_6 = { 'Ok' : ListAppResponse } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : ListOrdersResponse } |
  { 'Err' : EgoError };
export type Result_8 = { 'Ok' : ListUserResponse } |
  { 'Err' : EgoError };
export type Result_9 = { 'Ok' : MeResponse } |
  { 'Err' : EgoError };
export interface SetFrontendAddressRequest {
  'canister_id' : Principal,
  'version' : Version,
  'app_id' : string,
}
export interface SetRoleRequest {
  'user_id' : Principal,
  'is_app_auditer' : boolean,
  'is_app_developer' : boolean,
  'is_manager' : boolean,
}
export interface SetRoleResponse { 'ret' : boolean }
export interface UpgradeWalletRequest { 'version' : Version }
export interface User {
  'installed_apps' : Array<string>,
  'orders' : Array<bigint>,
  'name' : string,
  'user_id' : Principal,
  'is_app_auditer' : boolean,
  'is_app_developer' : boolean,
  'created_apps' : Array<string>,
  'wallet_id' : [] | [Principal],
  'is_manager' : boolean,
}
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface Wasm {
  'bucket_id' : Principal,
  'canister_id' : [] | [Principal],
  'version' : Version,
  'app_id' : string,
  'canister_type' : CanisterType,
  'file_id' : string,
}
export interface _SERVICE {
  'approve_app_version' : ActorMethod<[ApproveAppVersionRequest], Result>,
  'create_app_order' : ActorMethod<[CreateOrderRequest], Result_1>,
  'create_recharge_order' : ActorMethod<[CreateRechargeRequest], Result_1>,
  'created_apps' : ActorMethod<[], Result_2>,
  'get_app' : ActorMethod<[GetAppRequest], Result_3>,
  'get_wallet' : ActorMethod<[], Result_4>,
  'init_store' : ActorMethod<[InitStoreRequest], Result_5>,
  'list_app' : ActorMethod<[ListAppRequest], Result_6>,
  'list_orders' : ActorMethod<[], Result_7>,
  'list_user' : ActorMethod<[ListUserRequest], Result_8>,
  'list_wait_for_audit_app' : ActorMethod<[], Result_2>,
  'me' : ActorMethod<[], Result_9>,
  'new_app_version' : ActorMethod<[NewAppVersionRequest], Result_5>,
  'notify_payment' : ActorMethod<[NotifyPaymentRequest], Result_5>,
  'register_app' : ActorMethod<[RegisterAppRequest], Result_3>,
  'register_developer' : ActorMethod<[ListUserRequest], Result_9>,
  'register_user' : ActorMethod<[ListUserRequest], Result_9>,
  'reject_app_version' : ActorMethod<[NewAppVersionRequest], Result_5>,
  'release_app_version' : ActorMethod<[NewAppVersionRequest], Result_5>,
  'revoke_app_version' : ActorMethod<[NewAppVersionRequest], Result_5>,
  'set_frontend_address' : ActorMethod<[SetFrontendAddressRequest], Result_5>,
  'set_role' : ActorMethod<[SetRoleRequest], Result_10>,
  'submit_app_version' : ActorMethod<[NewAppVersionRequest], Result_10>,
  'upgrade_wallet' : ActorMethod<[UpgradeWalletRequest], Result_10>,
}
