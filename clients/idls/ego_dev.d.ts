import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AdminAppCreateBackendRequest {
  'logo' : string,
  'name' : string,
  'description' : string,
  'version' : Version,
  'app_id' : string,
  'category' : Category,
  'backend_data_hash' : string,
  'backend_data' : Uint8Array | number[],
}
export interface App {
  'logo' : string,
  'name' : string,
  'description' : string,
  'app_id' : string,
  'app_hash' : string,
  'category' : Category,
  'current_version' : Version,
  'price' : number,
}
export interface AppMainNewRequest {
  'logo' : string,
  'name' : string,
  'description' : string,
  'app_id' : string,
  'category' : Category,
  'price' : number,
}
export interface AppVersion {
  'id' : bigint,
  'status' : AppVersionStatus,
  'wasm' : [] | [Wasm],
  'version' : Version,
  'app_id' : string,
  'last_update' : bigint,
  'file_id' : Principal,
}
export interface AppVersionSetFrontendAddressRequest {
  'canister_id' : Principal,
  'version' : Version,
  'app_id' : string,
}
export type AppVersionStatus = { 'NEW' : null } |
  { 'REJECTED' : null } |
  { 'SUBMITTED' : null } |
  { 'REVOKED' : null } |
  { 'RELEASED' : null } |
  { 'APPROVED' : null };
export interface AppVersionUploadWasmRequest {
  'data' : Uint8Array | number[],
  'hash' : string,
  'version' : Version,
  'app_id' : string,
}
export interface BackupInfo { 'state' : BackupStatus }
export interface BackupJob { 'name' : string, 'amount' : bigint }
export type BackupStatus = { 'MAINTAINING' : null } |
  { 'RUNNING' : null };
export interface ByteReadResponse {
  'total' : bigint,
  'data' : Uint8Array | number[],
  'hash' : string,
  'name' : string,
}
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export type Category = { 'System' : null } |
  { 'Vault' : null };
export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface Developer {
  'name' : string,
  'is_app_auditor' : boolean,
  'developer_id' : Principal,
  'last_update' : bigint,
  'created_apps' : Array<string>,
  'is_manager' : boolean,
}
export interface EgoDevApp {
  'app' : App,
  'developer_id' : Principal,
  'last_update' : bigint,
  'audit_version' : [] | [Version],
}
export interface EgoError { 'msg' : string, 'code' : number }
export interface LogEntry { 'ts' : bigint, 'msg' : string, 'kind' : string }
export type Result = { 'Ok' : AppVersion } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_10 = { 'Ok' : Array<[string, Array<Principal>]> } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : Array<CycleRecord> } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : CycleInfo } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : Array<LogEntry> } |
  { 'Err' : string };
export type Result_15 = { 'Ok' : [] | [Array<[Principal, string]>] } |
  { 'Err' : string };
export type Result_16 = { 'Ok' : [] | [ByteReadResponse] } |
  { 'Err' : string };
export type Result_17 = { 'Ok' : Array<Developer> } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : Array<EgoDevApp> } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : BackupInfo } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<BackupJob> } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : EgoDevApp } |
  { 'Err' : EgoError };
export type Result_9 = { 'Ok' : Developer } |
  { 'Err' : EgoError };
export interface UserRoleSetRequest {
  'user_id' : Principal,
  'is_app_auditor' : boolean,
  'is_manager' : boolean,
}
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface Wasm {
  'canister_id' : Principal,
  'version' : Version,
  'app_id' : string,
  'canister_type' : CanisterType,
}
export interface _SERVICE {
  'admin_app_create' : ActorMethod<[AdminAppCreateBackendRequest], Result>,
  'admin_app_transfer' : ActorMethod<[string], Result_1>,
  'app_version_approve' : ActorMethod<[string], Result>,
  'app_version_new' : ActorMethod<[string, Version], Result>,
  'app_version_reject' : ActorMethod<[string], Result>,
  'app_version_release' : ActorMethod<[string, Version], Result>,
  'app_version_revoke' : ActorMethod<[string, Version], Result>,
  'app_version_set_frontend_address' : ActorMethod<
    [AppVersionSetFrontendAddressRequest],
    Result_2
  >,
  'app_version_submit' : ActorMethod<[string, Version], Result>,
  'app_version_upload_wasm' : ActorMethod<
    [AppVersionUploadWasmRequest],
    Result_2
  >,
  'app_version_wait_for_audit' : ActorMethod<[], Result_3>,
  'backup_change_status' : ActorMethod<[BackupStatus], Result_4>,
  'backup_info_get' : ActorMethod<[], Result_5>,
  'backup_job_list' : ActorMethod<[], Result_6>,
  'balance_get' : ActorMethod<[], Result_7>,
  'developer_app_get' : ActorMethod<[string], Result_8>,
  'developer_app_list' : ActorMethod<[], Result_3>,
  'developer_app_new' : ActorMethod<[AppMainNewRequest], Result_8>,
  'developer_main_get' : ActorMethod<[], Result_9>,
  'developer_main_register' : ActorMethod<[string], Result_9>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_4>,
  'ego_canister_list' : ActorMethod<[], Result_10>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_4>,
  'ego_controller_add' : ActorMethod<[Principal], Result_4>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_4>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_cycle_check' : ActorMethod<[], Result_4>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_4>,
  'ego_cycle_history' : ActorMethod<[], Result_11>,
  'ego_cycle_info' : ActorMethod<[], Result_12>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_4>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result_7>,
  'ego_is_op' : ActorMethod<[], Result_13>,
  'ego_is_owner' : ActorMethod<[], Result_13>,
  'ego_is_user' : ActorMethod<[], Result_13>,
  'ego_log_list' : ActorMethod<[bigint], Result_14>,
  'ego_op_add' : ActorMethod<[Principal], Result_4>,
  'ego_op_list' : ActorMethod<[], Result_15>,
  'ego_op_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_4>,
  'ego_owner_list' : ActorMethod<[], Result_15>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result_7>,
  'ego_user_add' : ActorMethod<[Principal], Result_4>,
  'ego_user_list' : ActorMethod<[], Result_15>,
  'ego_user_remove' : ActorMethod<[Principal], Result_4>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_4>,
  'job_data_backup' : ActorMethod<[string, bigint, bigint], Result_16>,
  'job_data_export' : ActorMethod<[string, bigint, bigint, bigint], Result_16>,
  'job_data_restore' : ActorMethod<[string, Uint8Array | number[]], Result_4>,
  'user_main_list' : ActorMethod<[string], Result_17>,
  'user_role_set' : ActorMethod<[UserRoleSetRequest], Result_2>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
