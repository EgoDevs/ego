import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
export interface AppCreateRequest {
  'app_id' : string,
  'backend_data_hash' : string,
  'backend_data' : Uint8Array | number[],
}
export interface AppInfo {
  'app_id' : string,
  'current_version' : Version,
  'latest_version' : Version,
  'wallet_id' : [] | [Principal],
}
export type BackupExportFormat = { 'JSON' : null } |
  { 'BINARY' : null };
export interface BackupInfo {
  'state' : BackupStatus,
  'last_backup' : bigint,
  'recent_backup' : [] | [bigint],
}
export type BackupStatus = { 'MAINTAINING' : null } |
  { 'RUNNING' : null };
export interface ByteReadResponse {
  'data' : Uint8Array | number[],
  'hash' : string,
  'name' : string,
}
export interface ByteWriteRequest {
  'end' : bigint,
  'data' : Uint8Array | number[],
  'hash' : string,
  'name' : string,
  'start' : bigint,
  'format' : [] | [BackupExportFormat],
}
export interface Canister {
  'canister_id' : Principal,
  'canister_type' : CanisterType,
}
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export type Category = { 'System' : null } |
  { 'Vault' : null };
export type Command = { 'Ops' : null } |
  { 'Controllers' : null } |
  { 'RemoveCanister' : [string, Principal] } |
  { 'RemoveOwner' : Principal } |
  { 'Jobs' : null } |
  { 'Logs' : bigint } |
  { 'Users' : null } |
  { 'Canisters' : null } |
  { 'AddUser' : Principal } |
  { 'Owners' : null } |
  { 'RemoveOp' : Principal } |
  { 'AddOwner' : Principal } |
  { 'AddOp' : Principal } |
  { 'AddCanister' : [string, Principal] } |
  { 'AddController' : Principal } |
  { 'Cycles' : null } |
  { 'RemoveUser' : Principal } |
  { 'RemoveController' : Principal };
export interface CycleInfo {
  'records' : Array<HCycle>,
  'estimate_remaining' : bigint,
}
export interface EgoError { 'msg' : string, 'code' : number }
export interface HApp {
  'canister_id' : Principal,
  'app_id' : string,
  'last_backup' : bigint,
  'current_version' : Version,
  'backup_status' : BackupStatus,
  'latest_version' : Version,
}
export interface HController { 'principal' : Principal }
export interface HCycle { 'ts' : bigint, 'balance' : bigint }
export interface HJob { 'name' : string, 'amount' : bigint }
export interface HUser { 'principal' : Principal, 'name' : string }
export interface Record {
  'end' : bigint,
  'data' : Uint8Array | number[],
  'hash' : string,
  'name' : string,
  'start' : bigint,
}
export type Response = { 'Empty' : null } |
  { 'Controllers' : Array<HController> } |
  { 'Jobs' : Array<HJob> } |
  { 'Logs' : Array<string> } |
  { 'Users' : Array<HUser> } |
  { 'Canisters' : Array<HUser> } |
  { 'Cycles' : Array<HCycle> };
export type Result = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : Array<App> } |
  { 'Err' : EgoError };
export type Result_10 = { 'Ok' : Array<[string, Array<Principal>]> } |
  { 'Err' : string };
export type Result_11 = { 'Ok' : Array<HCycle> } |
  { 'Err' : string };
export type Result_12 = { 'Ok' : CycleInfo } |
  { 'Err' : string };
export type Result_13 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_14 = { 'Ok' : Array<string> } |
  { 'Err' : string };
export type Result_15 = { 'Ok' : [] | [Array<[Principal, string]>] } |
  { 'Err' : string };
export type Result_16 = { 'Ok' : [] | [ByteReadResponse] } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Response } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : HApp } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : Array<UserApp> } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : BackupInfo } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<HJob> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : AppInfo } |
  { 'Err' : string };
export type UpgradeStatus = { 'INIT' : null } |
  { 'RESTORED' : null } |
  { 'BACKUPED' : null } |
  { 'UPGRADED' : null };
export interface UserApp {
  'app' : App,
  'canister' : Canister,
  'latest_version' : Version,
}
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface _SERVICE {
  'admin_app_create' : ActorMethod<[AppCreateRequest], Result>,
  'admin_app_delete' : ActorMethod<[Principal], undefined>,
  'admin_app_deploy' : ActorMethod<[string], Result>,
  'admin_app_list' : ActorMethod<[], Result_1>,
  'admin_canister_backup' : ActorMethod<[Principal], undefined>,
  'admin_canister_call' : ActorMethod<[Principal, Command], Result_2>,
  'admin_canister_change_status' : ActorMethod<
    [Principal, BackupStatus],
    undefined
  >,
  'admin_canister_data' : ActorMethod<[Principal, Command], Result_2>,
  'admin_canister_data_export' : ActorMethod<
    [
      Principal,
      string,
      bigint,
      bigint,
      [] | [BackupExportFormat],
      [] | [bigint],
    ],
    Record
  >,
  'admin_canister_data_import' : ActorMethod<
    [
      Principal,
      string,
      bigint,
      bigint,
      Uint8Array | number[],
      string,
      [] | [BackupExportFormat],
    ],
    undefined
  >,
  'admin_canister_get' : ActorMethod<[Principal], Result_3>,
  'admin_canister_list' : ActorMethod<[], Result_4>,
  'admin_canister_restore' : ActorMethod<[Principal], undefined>,
  'admin_canister_upgrade' : ActorMethod<[Principal], undefined>,
  'admin_canister_upgrade_status_get' : ActorMethod<[Principal], UpgradeStatus>,
  'admin_wallet_app_transfer' : ActorMethod<
    [[] | [Principal], [] | [string], Principal],
    Result
  >,
  'backup_change_status' : ActorMethod<[BackupStatus], Result_5>,
  'backup_data_reset' : ActorMethod<[], undefined>,
  'backup_info_get' : ActorMethod<[], Result_6>,
  'backup_job_list' : ActorMethod<[], Result_7>,
  'balance_get' : ActorMethod<[], Result_8>,
  'ego_app_info_get' : ActorMethod<[], Result_9>,
  'ego_app_info_update' : ActorMethod<
    [[] | [Principal], string, Version],
    undefined
  >,
  'ego_app_version_check' : ActorMethod<[], Result_9>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_5>,
  'ego_canister_delete' : ActorMethod<[], Result_5>,
  'ego_canister_list' : ActorMethod<[], Result_10>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_5>,
  'ego_canister_upgrade' : ActorMethod<[], Result_5>,
  'ego_controller_add' : ActorMethod<[Principal], Result_5>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_5>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_5>,
  'ego_cycle_check' : ActorMethod<[], Result_5>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_5>,
  'ego_cycle_history' : ActorMethod<[], Result_11>,
  'ego_cycle_info' : ActorMethod<[], Result_12>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_5>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result_8>,
  'ego_is_op' : ActorMethod<[], Result_13>,
  'ego_is_owner' : ActorMethod<[], Result_13>,
  'ego_is_user' : ActorMethod<[], Result_13>,
  'ego_log_list' : ActorMethod<[bigint], Result_14>,
  'ego_op_add' : ActorMethod<[Principal], Result_5>,
  'ego_op_list' : ActorMethod<[], Result_15>,
  'ego_op_remove' : ActorMethod<[Principal], Result_5>,
  'ego_owner_add' : ActorMethod<[Principal], Result_5>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_5>,
  'ego_owner_list' : ActorMethod<[], Result_15>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_5>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_5>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result_8>,
  'ego_user_add' : ActorMethod<[Principal], Result_5>,
  'ego_user_list' : ActorMethod<[], Result_15>,
  'ego_user_remove' : ActorMethod<[Principal], Result_5>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_5>,
  'finish_backup' : ActorMethod<[], undefined>,
  'finish_restore' : ActorMethod<[], undefined>,
  'job_data_export' : ActorMethod<
    [string, bigint, bigint, [] | [BackupExportFormat], [] | [bigint]],
    Result_16
  >,
  'job_data_import' : ActorMethod<[ByteWriteRequest], Result_13>,
  'job_data_read' : ActorMethod<[string, bigint, bigint], Result_13>,
  'job_data_write' : ActorMethod<[string, bigint, bigint, boolean], Result_13>,
  'on_job_backup_init' : ActorMethod<[], undefined>,
  'on_job_restore_init' : ActorMethod<[], undefined>,
  'start_backup' : ActorMethod<[], undefined>,
  'start_restore' : ActorMethod<[Array<HJob>], undefined>,
  'wallet_main_register' : ActorMethod<[], Result>,
}
