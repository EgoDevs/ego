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
export interface AppInstallRequest {
  'version' : Version,
  'cycles' : bigint,
  'app_id' : string,
}
export interface AppInstallResponse {
  'canister_ids' : Array<Principal>,
  'version' : Version,
}
export interface AppNativeStatus {
  'status' : CanisterStatusEnum,
  'memory_size' : bigint,
  'cycles' : bigint,
  'settings' : CanisterSettings,
  'module_hash' : [] | [Array<number>],
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
export interface BalanceResult { 'amount' : bigint }
export interface BalanceResult_1 { 'amount' : bigint }
export interface Canister {
  'canister_id' : Principal,
  'canister_type' : CanisterType,
}
export interface CanisterSettings {
  'controller' : [] | [Principal],
  'freezing_threshold' : [] | [bigint],
  'controllers' : [] | [Array<Principal>],
  'memory_allocation' : [] | [bigint],
  'compute_allocation' : [] | [bigint],
}
export type CanisterStatusEnum = { 'stopped' : null } |
  { 'stopping' : null } |
  { 'running' : null };
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export type Category = { 'System' : null } |
  { 'Vault' : null };
export interface CheckAppStatus {
  'canister_id' : Principal,
  'native_status' : AppNativeStatus,
  'app_id' : string,
  'canister_type' : CanisterType,
}
export interface CheckAppStatusResponse {
  'app_status_result' : Array<CheckAppStatus>,
}
export interface EgoError { 'msg' : string, 'code' : number }
export interface GetAppRequest { 'app_id' : string }
export interface GetAppResponse { 'app' : App }
export interface InitWalletCanister {
  'wallet_version' : Version,
  'cron_canister_id' : Principal,
  'store_canister_id' : Principal,
}
export type Result = { 'Ok' : null } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : CheckAppStatusResponse } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : GetAppResponse } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : UserAppResponse } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : AppInstallResponse } |
  { 'Err' : EgoError };
export type Result_6 = { 'Ok' : BalanceResult } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : BalanceResult_1 } |
  { 'Err' : EgoError };
export type Result_8 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export interface SendCyclesArgs { 'canister' : Principal, 'amount' : bigint }
export interface SendCyclesArgs_1 { 'canister' : Principal, 'amount' : bigint }
export interface UserApp {
  'version' : Version,
  'app_id' : string,
  'canisters' : Array<Canister>,
}
export interface UserAppResponse { 'canisters' : Array<UserApp> }
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface Wasm {
  'bucket_id' : Principal,
  'version' : Version,
  'app_id' : string,
  'canister_type' : CanisterType,
  'file_id' : string,
}
export interface _SERVICE {
  'app_deposit' : ActorMethod<[SendCyclesArgs], Result>,
  'check_app_status' : ActorMethod<[], Result_1>,
  'get_app' : ActorMethod<[GetAppRequest], Result_2>,
  'get_apps' : ActorMethod<[], Result_3>,
  'init_wallet_canister' : ActorMethod<[InitWalletCanister], Result_4>,
  'install_app' : ActorMethod<[AppInstallRequest], Result_5>,
  'uninstall_app' : ActorMethod<[AppInstallRequest], Result_5>,
  'upgrade_app' : ActorMethod<[AppInstallRequest], Result_5>,
  'wallet_balance' : ActorMethod<[], Result_6>,
  'wallet_balance128' : ActorMethod<[], Result_7>,
  'wallet_send' : ActorMethod<[SendCyclesArgs_1], Result_8>,
}
