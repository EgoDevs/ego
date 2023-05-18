export const idlFactory = ({ IDL }) => {
  const AppCreateRequest = IDL.Record({
    'app_id' : IDL.Text,
    'backend_data_hash' : IDL.Text,
    'backend_data' : IDL.Vec(IDL.Nat8),
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
  const Category = IDL.Variant({ 'System' : IDL.Null, 'Vault' : IDL.Null });
  const Version = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const App = IDL.Record({
    'logo' : IDL.Text,
    'name' : IDL.Text,
    'description' : IDL.Text,
    'app_id' : IDL.Text,
    'app_hash' : IDL.Text,
    'category' : Category,
    'current_version' : Version,
    'price' : IDL.Float32,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Vec(App), 'Err' : EgoError });
  const Command = IDL.Variant({
    'Ops' : IDL.Null,
    'Controllers' : IDL.Null,
    'RemoveCanister' : IDL.Tuple(IDL.Text, IDL.Principal),
    'RemoveOwner' : IDL.Principal,
    'Jobs' : IDL.Null,
    'Logs' : IDL.Nat64,
    'Users' : IDL.Null,
    'Canisters' : IDL.Null,
    'AddUser' : IDL.Principal,
    'Owners' : IDL.Null,
    'RemoveOp' : IDL.Principal,
    'AddOwner' : IDL.Principal,
    'AddOp' : IDL.Principal,
    'AddCanister' : IDL.Tuple(IDL.Text, IDL.Principal),
    'AddController' : IDL.Principal,
    'Cycles' : IDL.Null,
    'RemoveUser' : IDL.Principal,
    'RemoveController' : IDL.Principal,
  });
  const HController = IDL.Record({ 'principal' : IDL.Principal });
  const HJob = IDL.Record({ 'name' : IDL.Text, 'amount' : IDL.Nat64 });
  const HUser = IDL.Record({ 'principal' : IDL.Principal, 'name' : IDL.Text });
  const HCycle = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Response = IDL.Variant({
    'Empty' : IDL.Null,
    'Controllers' : IDL.Vec(HController),
    'Jobs' : IDL.Vec(HJob),
    'Logs' : IDL.Vec(IDL.Text),
    'Users' : IDL.Vec(HUser),
    'Canisters' : IDL.Vec(HUser),
    'Cycles' : IDL.Vec(HCycle),
  });
  const Result_2 = IDL.Variant({ 'Ok' : Response, 'Err' : EgoError });
  const BackupStatus = IDL.Variant({
    'MAINTAINING' : IDL.Null,
    'RUNNING' : IDL.Null,
  });
  const BackupExportFormat = IDL.Variant({
    'JSON' : IDL.Null,
    'BINARY' : IDL.Null,
  });
  const Record = IDL.Record({
    'end' : IDL.Nat64,
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
    'name' : IDL.Text,
    'start' : IDL.Nat64,
  });
  const HApp = IDL.Record({
    'canister_id' : IDL.Principal,
    'app_id' : IDL.Text,
    'last_backup' : IDL.Nat64,
    'current_version' : Version,
    'backup_status' : BackupStatus,
    'latest_version' : Version,
  });
  const Result_3 = IDL.Variant({ 'Ok' : HApp, 'Err' : EgoError });
  const CanisterType = IDL.Variant({
    'BACKEND' : IDL.Null,
    'ASSET' : IDL.Null,
  });
  const Canister = IDL.Record({
    'canister_id' : IDL.Principal,
    'canister_type' : CanisterType,
  });
  const UserApp = IDL.Record({
    'app' : App,
    'canister' : Canister,
    'latest_version' : Version,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Vec(UserApp), 'Err' : EgoError });
  const UpgradeStatus = IDL.Variant({
    'INIT' : IDL.Null,
    'RESTORED' : IDL.Null,
    'BACKUPED' : IDL.Null,
    'UPGRADED' : IDL.Null,
  });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const BackupInfo = IDL.Record({
    'state' : BackupStatus,
    'last_backup' : IDL.Nat64,
    'recent_backup' : IDL.Opt(IDL.Nat64),
  });
  const Result_6 = IDL.Variant({ 'Ok' : BackupInfo, 'Err' : IDL.Text });
  const Result_7 = IDL.Variant({ 'Ok' : IDL.Vec(HJob), 'Err' : IDL.Text });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const AppInfo = IDL.Record({
    'app_id' : IDL.Text,
    'current_version' : Version,
    'latest_version' : Version,
    'wallet_id' : IDL.Opt(IDL.Principal),
  });
  const Result_9 = IDL.Variant({ 'Ok' : AppInfo, 'Err' : IDL.Text });
  const Result_10 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Principal))),
    'Err' : IDL.Text,
  });
  const Result_11 = IDL.Variant({ 'Ok' : IDL.Vec(HCycle), 'Err' : IDL.Text });
  const CycleInfo = IDL.Record({
    'records' : IDL.Vec(HCycle),
    'estimate_remaining' : IDL.Nat64,
  });
  const Result_12 = IDL.Variant({ 'Ok' : CycleInfo, 'Err' : IDL.Text });
  const Result_13 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const Result_14 = IDL.Variant({ 'Ok' : IDL.Vec(IDL.Text), 'Err' : IDL.Text });
  const Result_15 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))),
    'Err' : IDL.Text,
  });
  const ByteReadResponse = IDL.Record({
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
    'name' : IDL.Text,
  });
  const Result_16 = IDL.Variant({
    'Ok' : IDL.Opt(ByteReadResponse),
    'Err' : IDL.Text,
  });
  const ByteWriteRequest = IDL.Record({
    'end' : IDL.Nat64,
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
    'name' : IDL.Text,
    'start' : IDL.Nat64,
    'format' : IDL.Opt(BackupExportFormat),
  });
  return IDL.Service({
    'admin_app_create' : IDL.Func([AppCreateRequest], [Result], []),
    'admin_app_delete' : IDL.Func([IDL.Principal], [], []),
    'admin_app_deploy' : IDL.Func([IDL.Text], [Result], []),
    'admin_app_list' : IDL.Func([], [Result_1], []),
    'admin_canister_backup' : IDL.Func([IDL.Principal], [], []),
    'admin_canister_call' : IDL.Func([IDL.Principal, Command], [Result_2], []),
    'admin_canister_change_status' : IDL.Func(
        [IDL.Principal, BackupStatus],
        [],
        [],
      ),
    'admin_canister_data' : IDL.Func([IDL.Principal, Command], [Result_2], []),
    'admin_canister_data_export' : IDL.Func(
        [
          IDL.Principal,
          IDL.Text,
          IDL.Nat64,
          IDL.Nat64,
          IDL.Opt(BackupExportFormat),
          IDL.Opt(IDL.Nat64),
        ],
        [Record],
        [],
      ),
    'admin_canister_data_import' : IDL.Func(
        [
          IDL.Principal,
          IDL.Text,
          IDL.Nat64,
          IDL.Nat64,
          IDL.Vec(IDL.Nat8),
          IDL.Text,
          IDL.Opt(BackupExportFormat),
        ],
        [],
        [],
      ),
    'admin_canister_get' : IDL.Func([IDL.Principal], [Result_3], []),
    'admin_canister_list' : IDL.Func([], [Result_4], []),
    'admin_canister_restore' : IDL.Func([IDL.Principal], [], []),
    'admin_canister_upgrade' : IDL.Func([IDL.Principal], [], []),
    'admin_canister_upgrade_status_get' : IDL.Func(
        [IDL.Principal],
        [UpgradeStatus],
        [],
      ),
    'admin_wallet_app_transfer' : IDL.Func(
        [IDL.Opt(IDL.Principal), IDL.Opt(IDL.Text), IDL.Principal],
        [Result],
        [],
      ),
    'backup_change_status' : IDL.Func([BackupStatus], [Result_5], []),
    'backup_data_reset' : IDL.Func([], [], []),
    'backup_info_get' : IDL.Func([], [Result_6], []),
    'backup_job_list' : IDL.Func([], [Result_7], []),
    'balance_get' : IDL.Func([], [Result_8], ['query']),
    'ego_app_info_get' : IDL.Func([], [Result_9], ['query']),
    'ego_app_info_update' : IDL.Func(
        [IDL.Opt(IDL.Principal), IDL.Text, Version],
        [],
        [],
      ),
    'ego_app_version_check' : IDL.Func([], [Result_9], []),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_5], []),
    'ego_canister_delete' : IDL.Func([], [Result_5], []),
    'ego_canister_list' : IDL.Func([], [Result_10], []),
    'ego_canister_remove' : IDL.Func([IDL.Text, IDL.Principal], [Result_5], []),
    'ego_canister_upgrade' : IDL.Func([], [Result_5], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_5], []),
    'ego_cycle_check' : IDL.Func([], [Result_5], []),
    'ego_cycle_estimate_set' : IDL.Func([IDL.Nat64], [Result_5], []),
    'ego_cycle_history' : IDL.Func([], [Result_11], []),
    'ego_cycle_info' : IDL.Func([], [Result_12], []),
    'ego_cycle_recharge' : IDL.Func([IDL.Nat], [Result_5], []),
    'ego_cycle_threshold_get' : IDL.Func([], [Result_8], []),
    'ego_is_op' : IDL.Func([], [Result_13], ['query']),
    'ego_is_owner' : IDL.Func([], [Result_13], ['query']),
    'ego_is_user' : IDL.Func([], [Result_13], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_14], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_op_list' : IDL.Func([], [Result_15], []),
    'ego_op_remove' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_5],
        [],
      ),
    'ego_owner_list' : IDL.Func([], [Result_15], []),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_5], []),
    'ego_runtime_cycle_threshold_get' : IDL.Func([], [Result_8], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_user_list' : IDL.Func([], [Result_15], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_5], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_5], []),
    'finish_backup' : IDL.Func([], [], []),
    'finish_restore' : IDL.Func([], [], []),
    'job_data_export' : IDL.Func(
        [
          IDL.Text,
          IDL.Nat64,
          IDL.Nat64,
          IDL.Opt(BackupExportFormat),
          IDL.Opt(IDL.Nat64),
        ],
        [Result_16],
        [],
      ),
    'job_data_import' : IDL.Func([ByteWriteRequest], [Result_13], []),
    'job_data_read' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64],
        [Result_13],
        [],
      ),
    'job_data_write' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64, IDL.Bool],
        [Result_13],
        [],
      ),
    'on_job_backup_init' : IDL.Func([], [], []),
    'on_job_restore_init' : IDL.Func([], [], []),
    'start_backup' : IDL.Func([], [], []),
    'start_restore' : IDL.Func([IDL.Vec(HJob)], [], []),
    'wallet_main_register' : IDL.Func([], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
