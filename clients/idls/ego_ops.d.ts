import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AdminAppCreateRequest {
  'logo' : string,
  'name' : string,
  'description' : string,
  'version' : Version,
  'app_id' : string,
  'category' : Category,
  'backend_data_hash' : string,
  'backend_data' : Uint8Array | number[],
}
export interface AdminWalletCycleRechargeRequest {
  'cycle' : bigint,
  'comment' : string,
  'wallet_id' : Principal,
}
export interface AdminWalletProviderAddRequest {
  'wallet_provider' : Principal,
  'wallet_app_id' : string,
}
export type Category = { 'System' : null } |
  { 'Vault' : null };
export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface InitArg { 'init_caller' : [] | [Principal] }
export type Result = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_2 = { 'Ok' : null } |
  { 'Err' : string };
export type Result_3 = { 'Ok' : Array<[string, Array<Principal>]> } |
  { 'Err' : string };
export type Result_4 = { 'Ok' : Array<CycleRecord> } |
  { 'Err' : string };
export type Result_5 = { 'Ok' : CycleInfo } |
  { 'Err' : string };
export type Result_6 = { 'Ok' : boolean } |
  { 'Err' : string };
export type Result_7 = { 'Ok' : Array<string> } |
  { 'Err' : string };
export type Result_8 = { 'Ok' : [] | [Array<[Principal, string]>] } |
  { 'Err' : string };
export interface Version {
  'major' : number,
  'minor' : number,
  'patch' : number,
}
export interface _SERVICE {
  'admin_app_create' : ActorMethod<[AdminAppCreateRequest], Result>,
  'admin_wallet_cycle_recharge' : ActorMethod<
    [AdminWalletCycleRechargeRequest],
    Result
  >,
  'admin_wallet_order_new' : ActorMethod<[number], Result>,
  'admin_wallet_provider_add' : ActorMethod<
    [AdminWalletProviderAddRequest],
    Result
  >,
  'balance_get' : ActorMethod<[], Result_1>,
  'canister_main_track' : ActorMethod<[string], undefined>,
  'canister_relation_update' : ActorMethod<[string], undefined>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_2>,
  'ego_canister_list' : ActorMethod<[], Result_3>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_2>,
  'ego_controller_add' : ActorMethod<[Principal], Result_2>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_2>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_2>,
  'ego_cycle_check' : ActorMethod<[], Result_2>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_2>,
  'ego_cycle_history' : ActorMethod<[], Result_4>,
  'ego_cycle_info' : ActorMethod<[], Result_5>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_2>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result_1>,
  'ego_is_owner' : ActorMethod<[], Result_6>,
  'ego_is_user' : ActorMethod<[], Result_6>,
  'ego_log_list' : ActorMethod<[bigint], Result_7>,
  'ego_op_add' : ActorMethod<[Principal], Result_2>,
  'ego_owner_add' : ActorMethod<[Principal], Result_2>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_2>,
  'ego_owner_list' : ActorMethod<[], Result_8>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_2>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_2>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result_1>,
  'ego_user_add' : ActorMethod<[Principal], Result_2>,
  'ego_user_remove' : ActorMethod<[Principal], Result_2>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_2>,
}
