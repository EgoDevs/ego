import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddPaymentRequest {
  'to' : Array<number>,
  'from' : Array<number>,
  'memo' : bigint,
  'amount' : Tokens,
}
export interface AddPaymentResponse { 'result' : boolean, 'memo' : bigint }
export interface EgoError { 'msg' : string, 'code' : number }
export interface InitLedgerRequest {
  'init_method_name' : string,
  'cron_canister_id' : Principal,
  'start' : bigint,
  'store_canister_id' : Principal,
  'length' : bigint,
  'ledger_canister_id' : Principal,
}
export type Result = { 'Ok' : AddPaymentResponse } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : boolean } |
  { 'Err' : EgoError };
export interface Tokens { 'e8s' : bigint }
export interface _SERVICE {
  'add_payment' : ActorMethod<[AddPaymentRequest], Result>,
  'init_ledger' : ActorMethod<[InitLedgerRequest], Result_1>,
  'match_payment_task' : ActorMethod<[], Result_1>,
}
