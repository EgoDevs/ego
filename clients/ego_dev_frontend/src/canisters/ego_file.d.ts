import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddManagersRequest { 'managers' : Array<Principal> }
export interface AddManagersResponse { 'manager_count' : bigint }
export type AppVersion = { 'Beta' : null } |
  { 'Main' : null };
export interface EgoError { 'msg' : string, 'code' : number }
export interface FileCountResponse { 'count' : bigint }
export interface FileInfo {
  'file_hash' : string,
  'created_at' : bigint,
  'created_by' : Principal,
  'file_num' : bigint,
  'file_size' : bigint,
  'app_id' : string,
  'app_version' : AppVersion,
  'file_id' : string,
}
export interface GetFileInfoResponse { 'file_info' : FileInfo }
export interface GetFileRequest { 'fid' : string }
export interface GetFileResponse { 'data' : Array<number> }
export interface ListFileResponse { 'list' : Array<FileInfo> }
export type Result = { 'Ok' : AddManagersResponse } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : FileCountResponse } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : GetFileResponse } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : GetFileInfoResponse } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : ListFileResponse } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : RmManagersResponse } |
  { 'Err' : EgoError };
export type Result_6 = { 'Ok' : SetFileResponse } |
  { 'Err' : EgoError };
export interface RmManagersRequest { 'managers' : Array<Principal> }
export interface RmManagersResponse { 'manager_count' : bigint }
export interface SetFileRequest {
  'fid' : string,
  'appid' : string,
  'data' : Array<number>,
  'hash' : string,
}
export interface SetFileResponse {
  'fid' : string,
  'file_num' : bigint,
  'file_size' : bigint,
}
export interface _SERVICE {
  'add_managers' : ActorMethod<[AddManagersRequest], Result>,
  'file_count' : ActorMethod<[], Result_1>,
  'get_file' : ActorMethod<[GetFileRequest], Result_2>,
  'get_file_info' : ActorMethod<[GetFileRequest], Result_3>,
  'is_manager' : ActorMethod<[Principal], boolean>,
  'list_file' : ActorMethod<[], Result_4>,
  'remove_managers' : ActorMethod<[RmManagersRequest], Result_5>,
  'set_file' : ActorMethod<[SetFileRequest], Result_6>,
  'set_file_stable' : ActorMethod<[GetFileRequest], Result_3>,
}
