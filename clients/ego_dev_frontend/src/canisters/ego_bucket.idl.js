export const idlFactory = ({ IDL }) => {
  const AddManagersRequest = IDL.Record({
    'managers' : IDL.Vec(IDL.Principal),
  });
  const AddManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : AddManagersResponse, 'Err' : EgoError });
  const AppCreateRequest = IDL.Record({
    'appid' : IDL.Text,
    'caller' : IDL.Principal,
  });
  const AppCreateResponse = IDL.Record({
    'appid' : IDL.Text,
    'file_canister' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : AppCreateResponse, 'Err' : EgoError });
  const AppFilesRequest = IDL.Record({
    'appid' : IDL.Text,
    'version' : IDL.Text,
  });
  const VersionFiles = IDL.Record({
    'files' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Bool)),
    'fids' : IDL.Vec(IDL.Text),
    'version' : IDL.Text,
  });
  const AppFilesResponse = IDL.Record({
    'files' : VersionFiles,
    'appid' : IDL.Text,
    'versoin' : IDL.Text,
  });
  const Result_2 = IDL.Variant({ 'Ok' : AppFilesResponse, 'Err' : EgoError });
  const AppInfoRequest = IDL.Record({ 'appid' : IDL.Text });
  const AppInfoResponse = IDL.Record({
    'appid' : IDL.Text,
    'app_num' : IDL.Nat64,
    'create_at' : IDL.Nat64,
    'create_by' : IDL.Principal,
    'release' : IDL.Text,
    'file_canister' : IDL.Principal,
    'removed' : IDL.Bool,
  });
  const Result_3 = IDL.Variant({ 'Ok' : AppInfoResponse, 'Err' : EgoError });
  const AppListResponse = IDL.Record({ 'apps' : IDL.Vec(IDL.Text) });
  const Result_4 = IDL.Variant({ 'Ok' : AppListResponse, 'Err' : EgoError });
  const AppNewVersionRequest = IDL.Record({
    'appid' : IDL.Text,
    'fids' : IDL.Vec(IDL.Text),
    'version' : IDL.Text,
  });
  const AppNewVersionResponse = IDL.Record({
    'fid_count' : IDL.Nat64,
    'appid' : IDL.Text,
    'version' : IDL.Text,
  });
  const Result_5 = IDL.Variant({
    'Ok' : AppNewVersionResponse,
    'Err' : EgoError,
  });
  const Permission = IDL.Variant({
    'Read' : IDL.Null,
    'Write' : IDL.Null,
    'Delete' : IDL.Null,
  });
  const AppOperatorsResponse = IDL.Record({
    'appid' : IDL.Text,
    'operators' : IDL.Vec(IDL.Tuple(IDL.Principal, Permission)),
  });
  const Result_6 = IDL.Variant({
    'Ok' : AppOperatorsResponse,
    'Err' : EgoError,
  });
  const Result_7 = IDL.Variant({ 'Ok' : AppInfoRequest, 'Err' : EgoError });
  const AppSetOperatorsResponse = IDL.Record({
    'appid' : IDL.Text,
    'operator_count' : IDL.Nat64,
  });
  const Result_8 = IDL.Variant({
    'Ok' : AppSetOperatorsResponse,
    'Err' : EgoError,
  });
  const AppSetReleaseResponse = IDL.Record({
    'appid' : IDL.Text,
    'version' : IDL.Text,
    'file_count' : IDL.Nat64,
  });
  const Result_9 = IDL.Variant({
    'Ok' : AppSetReleaseResponse,
    'Err' : EgoError,
  });
  const GetFileCanisterResponse = IDL.Record({ 'canister' : IDL.Principal });
  const Result_10 = IDL.Variant({
    'Ok' : GetFileCanisterResponse,
    'Err' : EgoError,
  });
  const LoadFileRequest = IDL.Record({
    'fid' : IDL.Text,
    'appid' : IDL.Text,
    'version' : IDL.Text,
  });
  const LoadFileResponse = IDL.Record({
    'fid' : IDL.Text,
    'data' : IDL.Vec(IDL.Nat8),
  });
  const Result_11 = IDL.Variant({ 'Ok' : LoadFileResponse, 'Err' : EgoError });
  const RmManagersRequest = IDL.Record({ 'managers' : IDL.Vec(IDL.Principal) });
  const RmManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const Result_12 = IDL.Variant({
    'Ok' : RmManagersResponse,
    'Err' : EgoError,
  });
  const SetFileCanisterRequest = IDL.Record({ 'canister' : IDL.Principal });
  const UploadFileRequest = IDL.Record({
    'fid' : IDL.Text,
    'appid' : IDL.Text,
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
    'version' : IDL.Text,
  });
  const UploadFileResponse = IDL.Record({
    'fid' : IDL.Text,
    'appid' : IDL.Text,
    'file_num' : IDL.Nat64,
    'version' : IDL.Text,
    'file_count' : IDL.Nat64,
  });
  const Result_13 = IDL.Variant({
    'Ok' : UploadFileResponse,
    'Err' : EgoError,
  });
  return IDL.Service({
    'add_managers' : IDL.Func([AddManagersRequest], [Result], []),
    'app_create' : IDL.Func([AppCreateRequest], [Result_1], []),
    'app_files' : IDL.Func([AppFilesRequest], [Result_2], ['query']),
    'app_info' : IDL.Func([AppInfoRequest], [Result_3], ['query']),
    'app_list' : IDL.Func([], [Result_4], ['query']),
    'app_new_version' : IDL.Func([AppNewVersionRequest], [Result_5], []),
    'app_operators' : IDL.Func([AppInfoRequest], [Result_6], ['query']),
    'app_remove' : IDL.Func([AppInfoRequest], [Result_7], []),
    'app_set_operators' : IDL.Func([AppOperatorsResponse], [Result_8], []),
    'app_set_release' : IDL.Func([AppFilesRequest], [Result_9], []),
    'get_file_canister' : IDL.Func([], [Result_10], ['query']),
    'is_manager' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'load_file' : IDL.Func([LoadFileRequest], [Result_11], []),
    'remove_managers' : IDL.Func([RmManagersRequest], [Result_12], []),
    'set_file_canister' : IDL.Func([SetFileCanisterRequest], [], []),
    'upload_file' : IDL.Func([UploadFileRequest], [Result_13], []),
  });
};
export const init = ({ IDL }) => { return []; };
