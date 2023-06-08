import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AdminWalletCycleRechargeRequest {
  'cycle' : bigint,
  'comment' : string,
  'wallet_id' : Principal,
}
export interface AdminWalletProviderAddRequest {
  'wallet_provider' : Principal,
  'wallet_app_id' : string,
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
export interface AppInstallRequest { 'app_id' : string }
export interface AppReInstallRequest { 'canister_id' : Principal }
export interface AppUpgradeRequest { 'wallet_id' : Principal }
export interface Canister {
  'canister_id' : Principal,
  'canister_type' : CanisterType,
}
export type CanisterType = { 'BACKEND' : null } |
  { 'ASSET' : null };
export interface CashFlow {
  'balance' : bigint,
  'operator' : Principal,
  'created_at' : bigint,
  'comment' : string,
  'cycles' : bigint,
  'cash_flow_type' : CashFlowType,
}
export type CashFlowType = { 'CHARGE' : null } |
  { 'RECHARGE' : null };
export type Category = { 'System' : null } |
  { 'Vault' : null };
export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface EgoStoreApp { 'app' : App, 'wasm' : Wasm }
export interface LogEntry { 'ts' : bigint, 'msg' : string, 'kind' : string }
export interface Order {
  'to' : Uint8Array | number[],
  'status' : OrderStatus,
  'from' : Uint8Array | number[],
  'memo' : bigint,
  'amount' : number,
  'wallet_id' : Principal,
}
export type OrderStatus = { 'NEW' : null } |
  { 'SUCCESS' : null };
export type Result = { 'Ok' : Array<Principal> } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : UserApp } |
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
export type Result_16 = { 'Ok' : bigint } |
  { 'Err' : EgoError };
export type Result_17 = { 'Ok' : WalletCycleChargeResponse } |
  { 'Err' : EgoError };
export type Result_18 = { 'Ok' : Array<CashFlow> } |
  { 'Err' : EgoError };
export type Result_19 = { 'Ok' : Principal } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : Array<UserApp> } |
  { 'Err' : EgoError };
export type Result_20 = { 'Ok' : bigint } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : Array<Order> } |
  { 'Err' : EgoError };
export type Result_6 = { 'Ok' : App } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : Array<App> } |
  { 'Err' : EgoError };
export type Result_8 = { 'Ok' : bigint } |
  { 'Err' : string };
export type Result_9 = { 'Ok' : null } |
  { 'Err' : string };
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
export interface WalletCycleChargeRequest {
  'cycle' : bigint,
  'comment' : string,
  'wallet_id' : Principal,
}
export interface WalletCycleChargeResponse { 'ret' : boolean }
export interface Wasm {
  'canister_id' : Principal,
  'version' : Version,
  'app_id' : string,
  'canister_type' : CanisterType,
}
export interface _SERVICE {
  'admin_export' : ActorMethod<[], Uint8Array | number[]>,
  'admin_import' : ActorMethod<[Uint8Array | number[]], undefined>,
  'admin_tenant_list' : ActorMethod<[], Result>,
  'admin_wallet_app_get' : ActorMethod<[Principal, Principal], Result_1>,
  'admin_wallet_app_list' : ActorMethod<[Principal], Result_2>,
  'admin_wallet_app_transfer' : ActorMethod<
    [[] | [Principal], [] | [string], Principal],
    Result_3
  >,
  'admin_wallet_cycle_recharge' : ActorMethod<
    [AdminWalletCycleRechargeRequest],
    Result_4
  >,
  'admin_wallet_list' : ActorMethod<[], Result>,
  'admin_wallet_order_list' : ActorMethod<[], Result_5>,
  'admin_wallet_provider_add' : ActorMethod<
    [AdminWalletProviderAddRequest],
    Result_3
  >,
  'app_main_get' : ActorMethod<[string], Result_6>,
  'app_main_list' : ActorMethod<[], Result_7>,
  'app_main_release' : ActorMethod<[EgoStoreApp], Result_4>,
  'balance_get' : ActorMethod<[], Result_8>,
  'ego_canister_add' : ActorMethod<[string, Principal], Result_9>,
  'ego_canister_list' : ActorMethod<[], Result_10>,
  'ego_canister_remove' : ActorMethod<[string, Principal], Result_9>,
  'ego_controller_add' : ActorMethod<[Principal], Result_9>,
  'ego_controller_remove' : ActorMethod<[Principal], Result_9>,
  'ego_controller_set' : ActorMethod<[Array<Principal>], Result_9>,
  'ego_cycle_check' : ActorMethod<[], Result_9>,
  'ego_cycle_estimate_set' : ActorMethod<[bigint], Result_9>,
  'ego_cycle_history' : ActorMethod<[], Result_11>,
  'ego_cycle_info' : ActorMethod<[], Result_12>,
  'ego_cycle_recharge' : ActorMethod<[bigint], Result_9>,
  'ego_cycle_threshold_get' : ActorMethod<[], Result_8>,
  'ego_is_op' : ActorMethod<[], Result_13>,
  'ego_is_owner' : ActorMethod<[], Result_13>,
  'ego_is_user' : ActorMethod<[], Result_13>,
  'ego_log_list' : ActorMethod<[bigint], Result_14>,
  'ego_op_add' : ActorMethod<[Principal], Result_9>,
  'ego_op_list' : ActorMethod<[], Result_15>,
  'ego_op_remove' : ActorMethod<[Principal], Result_9>,
  'ego_owner_add' : ActorMethod<[Principal], Result_9>,
  'ego_owner_add_with_name' : ActorMethod<[string, Principal], Result_9>,
  'ego_owner_list' : ActorMethod<[], Result_15>,
  'ego_owner_remove' : ActorMethod<[Principal], Result_9>,
  'ego_owner_set' : ActorMethod<[Array<Principal>], Result_9>,
  'ego_runtime_cycle_threshold_get' : ActorMethod<[], Result_8>,
  'ego_user_add' : ActorMethod<[Principal], Result_9>,
  'ego_user_list' : ActorMethod<[], Result_15>,
  'ego_user_remove' : ActorMethod<[Principal], Result_9>,
  'ego_user_set' : ActorMethod<[Array<Principal>], Result_9>,
  'flush_wallet_change_record' : ActorMethod<[], undefined>,
  'wallet_app_install' : ActorMethod<[string], Result_1>,
  'wallet_app_install_v2' : ActorMethod<[AppInstallRequest], Result_1>,
  'wallet_app_list' : ActorMethod<[], Result_2>,
  'wallet_app_reinstall_by_wallet_v2' : ActorMethod<
    [AppReInstallRequest],
    Result_3
  >,
  'wallet_app_remove' : ActorMethod<[Principal], Result_3>,
  'wallet_app_upgrade' : ActorMethod<[Principal], Result_3>,
  'wallet_app_upgrade_by_wallet' : ActorMethod<[Principal], Result_3>,
  'wallet_app_upgrade_by_wallet_v2' : ActorMethod<
    [AppReInstallRequest],
    Result_3
  >,
  'wallet_app_upgrade_v2' : ActorMethod<[AppUpgradeRequest], Result_3>,
  'wallet_canister_track' : ActorMethod<[Principal], Result_3>,
  'wallet_canister_track_self' : ActorMethod<[Principal], Result_3>,
  'wallet_canister_untrack' : ActorMethod<[Principal], Result_3>,
  'wallet_canister_untrack_self' : ActorMethod<[Principal], Result_3>,
  'wallet_cycle_balance' : ActorMethod<[], Result_16>,
  'wallet_cycle_charge' : ActorMethod<[WalletCycleChargeRequest], Result_17>,
  'wallet_cycle_list' : ActorMethod<[], Result_18>,
  'wallet_main_new' : ActorMethod<[Principal], Result_1>,
  'wallet_main_register' : ActorMethod<[Principal], Result_19>,
  'wallet_order_list' : ActorMethod<[], Result_5>,
  'wallet_order_new' : ActorMethod<[number], Result_20>,
  'wallet_order_notify' : ActorMethod<[bigint], Result_4>,
  'wallet_tenant_get' : ActorMethod<[], Result_19>,
}
