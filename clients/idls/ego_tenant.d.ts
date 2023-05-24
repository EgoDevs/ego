import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AppMainInstallRequest {
  'wasm' : Wasm,
  'user_id' : Principal,
  'wallet_id' : Principal,
}
export interface AppMainUpgradeRequest {
  'canister_id' : Principal,
  'wasm' : Wasm,
}
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface InitArg { 'init_caller' : [] | [Principal] }
export interface LogEntry { 'ts' : bigint, 'msg' : string, 'kind' : string }
export type Result = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : Principal } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : Array<Task> } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<[string, Array<Principal>]> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<LogEntry> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : [] | [Array<[Principal, string]>] } |
  { 'Err' : string };
export interface Task {
  'canister_id' : Principal,
  'next_check_time' : bigint,
  'wallet_id' : Principal,
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
  'app_main_delete' : ActorMethod<[Principal], Result>,
  'app_main_install' : ActorMethod<[AppMainInstallRequest], Result_1>,
  'app_main_upgrade' : ActorMethod<[AppMainUpgradeRequest], Result_2>,
  'canister_main_track' : ActorMethod<[Principal, Principal], Result>,
  'canister_main_untrack' : ActorMethod<[Principal], Result>,
  'canister_task_list' : ActorMethod<[], Result_3>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_4>,
  'ego_canister_list' : ActorMethod<[], Result_5>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_4>,
  'ego_controller_add' : ActorMethod<[Principal], Result_4>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_4>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_cycle_check_cb' : ActorMethod<[Array<CycleRecord>, bigint], Result>,
  'ego_is_op' : ActorMethod<[], Result_6>,
  'ego_is_owner' : ActorMethod<[], Result_6>,
  'ego_is_user' : ActorMethod<[], Result_6>,
  'ego_log_list' : ActorMethod<[bigint], Result_7>,
  'ego_op_add' : ActorMethod<[Principal], Result_4>,
  'ego_op_list' : ActorMethod<[], Result_8>,
  'ego_op_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add' : ActorMethod<[Principal], Result_4>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_4>,
  'ego_owner_list' : ActorMethod<[], Result_8>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_4>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_4>,
  'ego_user_add' : ActorMethod<[Principal], Result_4>,
  'ego_user_list' : ActorMethod<[], Result_8>,
  'ego_user_remove' : ActorMethod<[Principal], Result_4>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_4>,
  'wallet_cycle_recharge' : ActorMethod<[bigint], Result>,
}
