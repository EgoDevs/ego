export const idlFactory = ({ IDL }) => {
  const Task = IDL.Record({
    'canister_id' : IDL.Principal,
    'next_check_time' : IDL.Nat64,
    'last_update' : IDL.Nat64,
    'last_cycle' : IDL.Opt(IDL.Nat),
    'wallet_id' : IDL.Principal,
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : IDL.Vec(Task), 'Err' : EgoError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
  const Version = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const CanisterType = IDL.Variant({
    'BACKEND' : IDL.Null,
    'ASSET' : IDL.Null,
  });
  const Wasm = IDL.Record({
    'canister_id' : IDL.Principal,
    'version' : Version,
    'app_id' : IDL.Text,
    'canister_type' : CanisterType,
  });
  const AppMainInstallRequest = IDL.Record({
    'wasm' : Wasm,
    'user_id' : IDL.Principal,
    'wallet_id' : IDL.Principal,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : EgoError });
  const AppMainReInstallRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'wasm' : Wasm,
  });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  const AppMainUpgradeRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'wasm' : Wasm,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Principal))),
    'Err' : IDL.Text,
  });
  const CycleRecord = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const LogEntry = IDL.Record({
    'ts' : IDL.Nat64,
    'msg' : IDL.Text,
    'kind' : IDL.Text,
  });
  const Result_7 = IDL.Variant({ 'Ok' : IDL.Vec(LogEntry), 'Err' : IDL.Text });
  const Result_8 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'admin_export' : IDL.Func([], [IDL.Vec(IDL.Nat8)], []),
    'admin_task_add' : IDL.Func([IDL.Vec(Task)], [], []),
    'admin_task_list' : IDL.Func([IDL.Nat64], [Result], []),
    'app_main_delete' : IDL.Func([IDL.Principal], [Result_1], []),
    'app_main_install' : IDL.Func([AppMainInstallRequest], [Result_2], []),
    'app_main_reinstall' : IDL.Func([AppMainReInstallRequest], [Result_3], []),
    'app_main_upgrade' : IDL.Func([AppMainUpgradeRequest], [Result_3], []),
    'canister_main_track' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'canister_main_untrack' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_4], []),
    'ego_canister_list' : IDL.Func([], [Result_5], []),
    'ego_canister_remove' : IDL.Func([IDL.Text, IDL.Principal], [Result_4], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_4], []),
    'ego_cycle_check_cb' : IDL.Func(
        [IDL.Vec(CycleRecord), IDL.Nat],
        [Result_1],
        [],
      ),
    'ego_is_op' : IDL.Func([], [Result_6], ['query']),
    'ego_is_owner' : IDL.Func([], [Result_6], ['query']),
    'ego_is_user' : IDL.Func([], [Result_6], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_7], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_op_list' : IDL.Func([], [Result_8], []),
    'ego_op_remove' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_4],
        [],
      ),
    'ego_owner_list' : IDL.Func([], [Result_8], []),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_4], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_user_list' : IDL.Func([], [Result_8], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_4], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_4], []),
    'wallet_cycle_recharge' : IDL.Func([IDL.Nat], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
