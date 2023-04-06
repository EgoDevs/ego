import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CycleInfo {
  'records' : Array<CycleRecord>,
  'estimate_remaining' : bigint,
}
export interface CycleRecord { 'ts' : bigint, 'balance' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface InitArg { 'init_caller' : [] | [Principal] }
export interface LedgerMainInitRequest { 'start' : bigint }
export interface LedgerPaymentAddRequest {
  'to' : Array<number>,
  'from' : Array<number>,
  'memo' : bigint,
  'amount' : Tokens,
}
export interface Payment {
  'to' : Array<number>,
  'status' : PaymentStatus,
  'from' : Array<number>,
  'memo' : bigint,
  'amount' : Tokens,
}
export type PaymentStatus = { 'NOTIFIED' : null } |
  { 'PENDING' : null } |
  { 'CONFIRMED' : null };
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
export type Result_6 = { 'Ok' : null } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : Array<Payment> } |
  { 'Err' : EgoError };
export interface Tokens { 'e8s' : bigint }
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
  'ledger_main_init' : ActorMethod<[LedgerMainInitRequest], Result_6>,
  'ledger_payment_add' : ActorMethod<[LedgerPaymentAddRequest], Result_6>,
  'ledger_payment_list' : ActorMethod<[], Result_7>,
  'message_main_notify' : ActorMethod<[], undefined>,
}
