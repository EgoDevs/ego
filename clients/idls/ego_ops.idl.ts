export const idlFactory = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  const Version = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const Category = IDL.Variant({ 'System' : IDL.Null, 'Vault' : IDL.Null });
  const AdminAppCreateRequest = IDL.Record({
    'logo' : IDL.Text,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'version' : Version,
    'app_id' : IDL.Text,
    'category' : Category,
    'backend_data_hash' : IDL.Text,
    'backend_data' : IDL.Vec(IDL.Nat8),
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
  const AdminWalletCycleRechargeRequest = IDL.Record({
    'cycle' : IDL.Nat,
    'comment' : IDL.Text,
    'wallet_id' : IDL.Principal,
  });
  const AdminWalletProviderAddRequest = IDL.Record({
    'wallet_provider' : IDL.Principal,
    'wallet_app_id' : IDL.Text,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Principal))),
    'Err' : IDL.Text,
  });
  const CycleRecord = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Result_4 = IDL.Variant({
    'Ok' : IDL.Vec(CycleRecord),
    'Err' : IDL.Text,
  });
  const CycleInfo = IDL.Record({
    'records' : IDL.Vec(CycleRecord),
    'estimate_remaining' : IDL.Nat64,
  });
  const Result_5 = IDL.Variant({ 'Ok' : CycleInfo, 'Err' : IDL.Text });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const Result_7 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Text), 'Err' : IDL.Text });
  const Result_8 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))),
    'Err' : IDL.Text,
  });
  return IDL.Service({
    'admin_app_create' : IDL.Func([AdminAppCreateRequest], [Result], []),
    'admin_wallet_cycle_recharge' : IDL.Func(
        [AdminWalletCycleRechargeRequest],
        [Result],
        [],
      ),
    'admin_wallet_order_new' : IDL.Func([IDL.Float32], [Result], []),
    'admin_wallet_provider_add' : IDL.Func(
        [AdminWalletProviderAddRequest],
        [Result],
        [],
      ),
    'balance_get' : IDL.Func([], [Result_1], ['query']),
    'canister_main_track' : IDL.Func([IDL.Text], [], []),
    'canister_relation_update' : IDL.Func([IDL.Text], [], []),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_2], []),
    'ego_canister_list' : IDL.Func([], [Result_3], []),
    'ego_canister_remove' : IDL.Func([IDL.Text, IDL.Principal], [Result_2], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_2], []),
    'ego_cycle_check' : IDL.Func([], [Result_2], []),
    'ego_cycle_estimate_set' : IDL.Func([IDL.Nat64], [Result_2], []),
    'ego_cycle_history' : IDL.Func([], [Result_4], ['query']),
    'ego_cycle_info' : IDL.Func([], [Result_5], []),
    'ego_cycle_recharge' : IDL.Func([IDL.Nat], [Result_2], []),
    'ego_cycle_threshold_get' : IDL.Func([], [Result_1], []),
    'ego_is_owner' : IDL.Func([], [Result_6], ['query']),
    'ego_is_user' : IDL.Func([], [Result_6], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_7], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_2],
        [],
      ),
    'ego_owner_list' : IDL.Func([], [Result_8], []),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_2], []),
    'ego_runtime_cycle_threshold_get' : IDL.Func([], [Result_1], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_2], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_2], []),
  });
};
export const init = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  return [InitArg];
};
