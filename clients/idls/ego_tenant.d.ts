import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AppMainInstallRequest {
  'wasm' : Wasm,
  'user_id' : Principal,
  'wallet_id' : Principal,
}
export interface AppMainReInstallRequest {
  'canister_id' : Principal,
  'wasm' : Wasm,
}
export interface AppMainUpgradeRequest {
  'canister_id' : Principal,
  'wasm' : Wasm,
}
export interface BackupInfo { 'state' : BackupStatus }
export interface BackupJob { 'name' : string, 'amount' : bigint }
export type BackupStatus = { 'MAINTAINING' : null } |
  { 'RUNNING' : null };
export interface ByteReadResponse {
  'data' : Uint8Array | number[],
  'hash' : string,
  'name' : string,
}
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface LogEntry { 'ts' : bigint, 'msg' : string, 'kind' : string }
export type Result = { 'Ok' : Array<Task> } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_10 = { 'Ok' : CycleInfo } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : Array<LogEntry> } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : [] | [Array<[Principal, string]>] } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : [] | [ByteReadResponse] } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Principal } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : BackupInfo } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<BackupJob> } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : Array<[string, Array<Principal>]> } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : Array<CycleRecord> } |
  { 'Err' : string };
export interface Task {
  'canister_id' : Principal,
  'next_check_time' : bigint,
  'last_update' : bigint,
  'last_cycle' : [] | [bigint],
  'try_count' : number,
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
  'admin_export' : ActorMethod<[], Uint8Array | number[]>,
  'admin_import' : ActorMethod<[Array<Task>], undefined>,
  'admin_task_check' : ActorMethod<[Principal], undefined>,
  'admin_task_list' : ActorMethod<[bigint], Result>,
  'app_main_delete' : ActorMethod<[Principal], Result_1>,
  'app_main_install' : ActorMethod<[AppMainInstallRequest], Result_2>,
  'app_main_reinstall' : ActorMethod<[AppMainReInstallRequest], Result_3>,
  'app_main_upgrade' : ActorMethod<[AppMainUpgradeRequest], Result_3>,
  'backup_change_status' : ActorMethod<[BackupStatus], Result_4>,
  'backup_info_get' : ActorMethod<[], Result_5>,
  'backup_job_list' : ActorMethod<[], Result_6>,
  'balance_get' : ActorMethod<[], Result_7>,
  'canister_main_track' : ActorMethod<[Principal], Result_1>,
  'canister_main_untrack' : ActorMethod<[Principal], Result_1>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_4>,
  'ego_canister_list' : ActorMethod<[], Result_8>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_4>,
  'ego_controller_add' : ActorMethod<[Principal], Result_4>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_4>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_cycle_check' : ActorMethod<[], Result_4>,
  'ego_cycle_check_cb' : ActorMethod<[Array<CycleRecord>, bigint], Result_1>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_4>,
  'ego_cycle_history' : ActorMethod<[], Result_9>,
  'ego_cycle_info' : ActorMethod<[], Result_10>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_4>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result_7>,
  'ego_is_op' : ActorMethod<[], Result_11>,
  'ego_is_owner' : ActorMethod<[], Result_11>,
  'ego_is_user' : ActorMethod<[], Result_11>,
  'ego_log_list' : ActorMethod<[bigint], Result_12>,
  'ego_op_add' : ActorMethod<[Principal], Result_4>,
  'ego_op_list' : ActorMethod<[], Result_13>,
  'ego_op_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_4>,
  'ego_owner_list' : ActorMethod<[], Result_13>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result_7>,
  'ego_user_add' : ActorMethod<[Principal], Result_4>,
  'ego_user_list' : ActorMethod<[], Result_13>,
  'ego_user_remove' : ActorMethod<[Principal], Result_4>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_4>,
  'job_data_export' : ActorMethod<[string, [] | [bigint]], Result_14>,
  'reset_next_check_time' : ActorMethod<[], undefined>,
  'wallet_cycle_recharge' : ActorMethod<[bigint], Result_1>,
}
