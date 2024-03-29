export const idlFactory = ({ IDL }) => {
  const Result = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const Result_2 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Principal))),
    'Err' : IDL.Text,
  });
  const CycleRecord = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Vec(CycleRecord),
    'Err' : IDL.Text,
  });
  const CycleInfo = IDL.Record({
    'records' : IDL.Vec(CycleRecord),
    'estimate_remaining' : IDL.Nat64,
  });
  const Result_4 = IDL.Variant({ 'Ok' : CycleInfo, 'Err' : IDL.Text });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const LogEntry = IDL.Record({
    'ts' : IDL.Nat64,
    'msg' : IDL.Text,
    'kind' : IDL.Text,
  });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Vec(LogEntry), 'Err' : IDL.Text });
  const Result_7 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))),
    'Err' : IDL.Text,
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Nat8), 'Err' : EgoError });
  const Result_9 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  return IDL.Service({
    'balance_get' : IDL.Func([], [Result], ['query']),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_1], []),
    'ego_canister_list' : IDL.Func([], [Result_2], []),
    'ego_canister_remove' : IDL.Func([IDL.Text, IDL.Principal], [Result_1], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_1], []),
    'ego_cycle_check' : IDL.Func([], [Result_1], []),
    'ego_cycle_estimate_set' : IDL.Func([IDL.Nat64], [Result_1], []),
    'ego_cycle_history' : IDL.Func([], [Result_3], []),
    'ego_cycle_info' : IDL.Func([], [Result_4], []),
    'ego_cycle_recharge' : IDL.Func([IDL.Nat], [Result_1], []),
    'ego_cycle_threshold_get' : IDL.Func([], [Result], []),
    'ego_is_op' : IDL.Func([], [Result_5], ['query']),
    'ego_is_owner' : IDL.Func([], [Result_5], ['query']),
    'ego_is_user' : IDL.Func([], [Result_5], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_6], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_op_list' : IDL.Func([], [Result_7], []),
    'ego_op_remove' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_1],
        [],
      ),
    'ego_owner_list' : IDL.Func([], [Result_7], []),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_1], []),
    'ego_runtime_cycle_threshold_get' : IDL.Func([], [Result], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_user_list' : IDL.Func([], [Result_7], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_1], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_1], []),
    'file_main_read' : IDL.Func([IDL.Text], [Result_8], ['query']),
    'file_main_write' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result_9],
        [],
      ),
    'state_persist' : IDL.Func([], [Result_9], []),
    'state_restore' : IDL.Func([], [Result_9], []),
  });
};
export const init = ({ IDL }) => { return []; };
