export const idlFactory = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
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
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : EgoError });
  const AppMainUpgradeRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'wasm' : Wasm,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  const Result_3 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const CycleRecord = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Text), 'Err' : IDL.Text });
  return IDL.Service({
    'app_main_delete' : IDL.Func([IDL.Principal], [Result], []),
    'app_main_install' : IDL.Func([AppMainInstallRequest], [Result_1], []),
    'app_main_upgrade' : IDL.Func([AppMainUpgradeRequest], [Result_2], []),
    'canister_main_track' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result],
        [],
      ),
    'canister_main_untrack' : IDL.Func([IDL.Principal], [Result], []),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_3], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_3], []),
    'ego_cycle_check_cb' : IDL.Func(
        [IDL.Vec(CycleRecord), IDL.Nat],
        [Result],
        [],
      ),
    'ego_is_owner' : IDL.Func([], [Result_4], ['query']),
    'ego_is_user' : IDL.Func([], [Result_4], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_5], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_3],
        [],
      ),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_3], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_3], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_3], []),
    'wallet_cycle_recharge' : IDL.Func([IDL.Nat], [Result], []),
  });
};
export const init = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  return [InitArg];
};
