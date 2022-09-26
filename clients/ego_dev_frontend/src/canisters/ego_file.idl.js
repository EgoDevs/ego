export const idlFactory = ({ IDL }) => {
  const AddManagersRequest = IDL.Record({
    'managers' : IDL.Vec(IDL.Principal),
  });
  const AddManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : AddManagersResponse, 'Err' : EgoError });
  const FileCountResponse = IDL.Record({ 'count' : IDL.Nat64 });
  const Result_1 = IDL.Variant({ 'Ok' : FileCountResponse, 'Err' : EgoError });
  const GetFileRequest = IDL.Record({ 'fid' : IDL.Text });
  const GetFileResponse = IDL.Record({ 'data' : IDL.Vec(IDL.Nat8) });
  const Result_2 = IDL.Variant({ 'Ok' : GetFileResponse, 'Err' : EgoError });
  const AppVersion = IDL.Variant({ 'Beta' : IDL.Null, 'Main' : IDL.Null });
  const FileInfo = IDL.Record({
    'file_hash' : IDL.Text,
    'created_at' : IDL.Nat64,
    'created_by' : IDL.Principal,
    'file_num' : IDL.Nat64,
    'file_size' : IDL.Nat64,
    'app_id' : IDL.Text,
    'app_version' : AppVersion,
    'file_id' : IDL.Text,
  });
  const GetFileInfoResponse = IDL.Record({ 'file_info' : FileInfo });
  const Result_3 = IDL.Variant({
    'Ok' : GetFileInfoResponse,
    'Err' : EgoError,
  });
  const ListFileResponse = IDL.Record({ 'list' : IDL.Vec(FileInfo) });
  const Result_4 = IDL.Variant({ 'Ok' : ListFileResponse, 'Err' : EgoError });
  const RmManagersRequest = IDL.Record({ 'managers' : IDL.Vec(IDL.Principal) });
  const RmManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const Result_5 = IDL.Variant({ 'Ok' : RmManagersResponse, 'Err' : EgoError });
  const SetFileRequest = IDL.Record({
    'fid' : IDL.Text,
    'appid' : IDL.Text,
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
  });
  const SetFileResponse = IDL.Record({
    'fid' : IDL.Text,
    'file_num' : IDL.Nat64,
    'file_size' : IDL.Nat64,
  });
  const Result_6 = IDL.Variant({ 'Ok' : SetFileResponse, 'Err' : EgoError });
  return IDL.Service({
    'add_managers' : IDL.Func([AddManagersRequest], [Result], []),
    'file_count' : IDL.Func([], [Result_1], ['query']),
    'get_file' : IDL.Func([GetFileRequest], [Result_2], ['query']),
    'get_file_info' : IDL.Func([GetFileRequest], [Result_3], ['query']),
    'is_manager' : IDL.Func([IDL.Principal], [IDL.Bool], ['query']),
    'list_file' : IDL.Func([], [Result_4], ['query']),
    'remove_managers' : IDL.Func([RmManagersRequest], [Result_5], []),
    'set_file' : IDL.Func([SetFileRequest], [Result_6], []),
    'set_file_stable' : IDL.Func([GetFileRequest], [Result_3], []),
  });
};
export const init = ({ IDL }) => { return []; };
