import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface InitArg { 'init_caller' : [] | [Principal] }
export type Result = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_1 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : Array<CycleRecord> } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : CycleInfo } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : Array<string> } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : Array<number> } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export interface _SERVICE {
  'balance_get' : ActorMethod<[], Result>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_1>,
  'ego_controller_add' : ActorMethod<[Principal], Result_1>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_1>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_1>,
  'ego_cycle_check' : ActorMethod<[], Result_1>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_1>,
  'ego_cycle_history' : ActorMethod<[], Result_2>,
  'ego_cycle_info' : ActorMethod<[], Result_3>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_1>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result>,
  'ego_is_owner' : ActorMethod<[], Result_4>,
  'ego_is_user' : ActorMethod<[], Result_4>,
  'ego_log_list' : ActorMethod<[bigint], Result_5>,
  'ego_op_add' : ActorMethod<[Principal], Result_1>,
  'ego_owner_add' : ActorMethod<[Principal], Result_1>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_1>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_1>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_1>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result>,
  'ego_user_add' : ActorMethod<[Principal], Result_1>,
  'ego_user_remove' : ActorMethod<[Principal], Result_1>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_1>,
  'file_main_read' : ActorMethod<[string], Result_6>,
  'file_main_write' : ActorMethod<[string, string, Array<number>], Result_7>,
  'state_persist' : ActorMethod<[], Result_7>,
  'state_restore' : ActorMethod<[], Result_7>,
}
