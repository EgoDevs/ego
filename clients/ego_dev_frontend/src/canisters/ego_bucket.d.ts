import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AddManagersRequest { 'managers' : Array<Principal> }
export interface AddManagersResponse { 'manager_count' : bigint }
export interface AppCreateRequest { 'appid' : string, 'caller' : Principal }
export interface AppCreateResponse {
  'appid' : string,
  'file_canister' : Principal,
}
export interface AppFilesRequest { 'appid' : string, 'version' : string }
export interface AppFilesResponse {
  'files' : VersionFiles,
  'appid' : string,
  'versoin' : string,
}
export interface AppInfoRequest { 'appid' : string }
export interface AppInfoResponse {
  'appid' : string,
  'app_num' : bigint,
  'create_at' : bigint,
  'create_by' : Principal,
  'release' : string,
  'file_canister' : Principal,
  'removed' : boolean,
}
export interface AppListResponse { 'apps' : Array<string> }
export interface AppNewVersionRequest {
  'appid' : string,
  'fids' : Array<string>,
  'version' : string,
}
export interface AppNewVersionResponse {
  'fid_count' : bigint,
  'appid' : string,
  'version' : string,
}
export interface AppOperatorsResponse {
  'appid' : string,
  'operators' : Array<[Principal, Permission]>,
}
export interface AppSetOperatorsResponse {
  'appid' : string,
  'operator_count' : bigint,
}
export interface AppSetReleaseResponse {
  'appid' : string,
  'version' : string,
  'file_count' : bigint,
}
export interface EgoError { 'msg' : string, 'code' : number }
export interface GetFileCanisterResponse { 'canister' : Principal }
export interface LoadFileRequest {
  'fid' : string,
  'appid' : string,
  'version' : string,
}
export interface LoadFileResponse { 'fid' : string, 'data' : Array<number> }
export type Permission = { 'Read' : null } |
  { 'Write' : null } |
  { 'Delete' : null };
export type Result = { 'Ok' : AddManagersResponse } |
  { 'Err' : EgoError };
export type Result_1 = { 'Ok' : AppCreateResponse } |
  { 'Err' : EgoError };
export type Result_10 = { 'Ok' : GetFileCanisterResponse } |
  { 'Err' : EgoError };
export type Result_11 = { 'Ok' : LoadFileResponse } |
  { 'Err' : EgoError };
export type Result_12 = { 'Ok' : RmManagersResponse } |
  { 'Err' : EgoError };
export type Result_13 = { 'Ok' : UploadFileResponse } |
  { 'Err' : EgoError };
export type Result_2 = { 'Ok' : AppFilesResponse } |
  { 'Err' : EgoError };
export type Result_3 = { 'Ok' : AppInfoResponse } |
  { 'Err' : EgoError };
export type Result_4 = { 'Ok' : AppListResponse } |
  { 'Err' : EgoError };
export type Result_5 = { 'Ok' : AppNewVersionResponse } |
  { 'Err' : EgoError };
export type Result_6 = { 'Ok' : AppOperatorsResponse } |
  { 'Err' : EgoError };
export type Result_7 = { 'Ok' : AppInfoRequest } |
  { 'Err' : EgoError };
export type Result_8 = { 'Ok' : AppSetOperatorsResponse } |
  { 'Err' : EgoError };
export type Result_9 = { 'Ok' : AppSetReleaseResponse } |
  { 'Err' : EgoError };
export interface RmManagersRequest { 'managers' : Array<Principal> }
export interface RmManagersResponse { 'manager_count' : bigint }
export interface SetFileCanisterRequest { 'canister' : Principal }
export interface UploadFileRequest {
  'fid' : string,
  'appid' : string,
  'data' : Array<number>,
  'hash' : string,
  'version' : string,
}
export interface UploadFileResponse {
  'fid' : string,
  'appid' : string,
  'file_num' : bigint,
  'version' : string,
  'file_count' : bigint,
}
export interface VersionFiles {
  'files' : Array<[string, boolean]>,
  'fids' : Array<string>,
  'version' : string,
}
export interface _SERVICE {
  'add_managers' : ActorMethod<[AddManagersRequest], Result>,
  'app_create' : ActorMethod<[AppCreateRequest], Result_1>,
  'app_files' : ActorMethod<[AppFilesRequest], Result_2>,
  'app_info' : ActorMethod<[AppInfoRequest], Result_3>,
  'app_list' : ActorMethod<[], Result_4>,
  'app_new_version' : ActorMethod<[AppNewVersionRequest], Result_5>,
  'app_operators' : ActorMethod<[AppInfoRequest], Result_6>,
  'app_remove' : ActorMethod<[AppInfoRequest], Result_7>,
  'app_set_operators' : ActorMethod<[AppOperatorsResponse], Result_8>,
  'app_set_release' : ActorMethod<[AppFilesRequest], Result_9>,
  'get_file_canister' : ActorMethod<[], Result_10>,
  'is_manager' : ActorMethod<[Principal], boolean>,
  'load_file' : ActorMethod<[LoadFileRequest], Result_11>,
  'remove_managers' : ActorMethod<[RmManagersRequest], Result_12>,
  'set_file_canister' : ActorMethod<[SetFileCanisterRequest], undefined>,
  'upload_file' : ActorMethod<[UploadFileRequest], Result_13>,
}
