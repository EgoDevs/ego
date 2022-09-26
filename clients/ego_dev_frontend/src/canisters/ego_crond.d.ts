import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddCronTaskRequest {
  'method' : string,
  'interval' : CronInterval,
  'canister_id' : Principal,
}
export interface AddCronTaskResponse {
  'task_id' : bigint,
  'duration_nano' : bigint,
}
export interface AddManagersRequest { 'managers' : Array<Principal> }
export interface AddManagersResponse { 'manager_count' : bigint }
export interface CancelCronTaskRequest { 'canister_id' : Principal }
export interface CancelCronTaskResponse {
  'task_id' : bigint,
  'interval' : CronInterval,
}
export type CronInterval = { 'PerHour' : null } |
  { 'PerDay' : null } |
  { 'PerMinute' : null } |
  { 'PerSecond' : null };
export type EgoCronError = { 'AlreadyHasTask' : null } |
  { 'UnknownError' : string } |
  { 'TaskNotFound' : null } |
  { 'CancelFail' : bigint };
export interface GetCronTaskRequest { 'canister_id' : Principal }
export interface GetCronTaskResponse {
  'method' : string,
  'task_id' : bigint,
  'interval' : CronInterval,
  'canister_id' : Principal,
}
export type Result = { 'Ok' : AddCronTaskResponse } |
  { 'Err' : EgoCronError };
export type Result_1 = { 'Ok' : AddManagersResponse } |
  { 'Err' : EgoCronError };
export type Result_2 = { 'Ok' : CancelCronTaskResponse } |
  { 'Err' : EgoCronError };
export type Result_3 = { 'Ok' : GetCronTaskResponse } |
  { 'Err' : EgoCronError };
export type Result_4 = { 'Ok' : RmManagersResponse } |
  { 'Err' : EgoCronError };
export interface RmManagersRequest { 'managers' : Array<Principal> }
export interface RmManagersResponse { 'manager_count' : bigint }
export interface _SERVICE {
  'add_cron_task' : ActorMethod<[AddCronTaskRequest], Result>,
  'add_managers' : ActorMethod<[AddManagersRequest], Result_1>,
  'cancel_cron_task' : ActorMethod<[CancelCronTaskRequest], Result_2>,
  'get_cron_task' : ActorMethod<[GetCronTaskRequest], Result_3>,
  'is_manager' : ActorMethod<[Principal], string>,
  'remove_managers' : ActorMethod<[RmManagersRequest], Result_4>,
}
