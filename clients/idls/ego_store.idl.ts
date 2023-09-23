export const idlFactory = ({ IDL }) => {
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
    'wallet_id' : IDL.Opt(IDL.Principal),
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : UserApp, 'Err' : EgoError });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
  const AdminWalletCycleRechargeRequest = IDL.Record({
    'cycle' : IDL.Nat,
    'comment' : IDL.Text,
    'wallet_id' : IDL.Principal,
  });
  const Result_2 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  const AdminWalletProviderAddRequest = IDL.Record({
    'wallet_provider' : IDL.Principal,
    'wallet_app_id' : IDL.Text,
  });
  const WalletProvider = IDL.Record({
    'app_id' : IDL.Text,
    'wallet_provider' : IDL.Principal,
  });
  const Result_3 = IDL.Variant({
    'Ok' : IDL.Vec(WalletProvider),
    'Err' : EgoError,
  });
  const Result_4 = IDL.Variant({ 'Ok' : App, 'Err' : EgoError });
  const Result_5 = IDL.Variant({ 'Ok' : IDL.Vec(App), 'Err' : EgoError });
  const Wasm = IDL.Record({
    'canister_id' : IDL.Principal,
    'version' : Version,
    'app_id' : IDL.Text,
    'canister_type' : CanisterType,
  });
  const EgoStoreApp = IDL.Record({
    'app' : App,
    'wasm' : Wasm,
    'last_update' : IDL.Nat64,
  });
  const BackupStatus = IDL.Variant({
    'MAINTAINING' : IDL.Null,
    'RUNNING' : IDL.Null,
  });
  const Result_6 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const BackupInfo = IDL.Record({ 'state' : BackupStatus });
  const Result_7 = IDL.Variant({ 'Ok' : BackupInfo, 'Err' : IDL.Text });
  const BackupJob = IDL.Record({ 'name' : IDL.Text, 'amount' : IDL.Nat64 });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Vec(BackupJob), 'Err' : IDL.Text });
  const Result_9 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text });
  const Result_10 = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Vec(IDL.Principal))),
    'Err' : IDL.Text,
  });
  const CycleRecord = IDL.Record({ 'ts' : IDL.Nat64, 'balance' : IDL.Nat });
  const Result_11 = IDL.Variant({
    'Ok' : IDL.Vec(CycleRecord),
    'Err' : IDL.Text,
  });
  const CycleInfo = IDL.Record({
    'records' : IDL.Vec(CycleRecord),
    'estimate_remaining' : IDL.Nat64,
  });
  const Result_12 = IDL.Variant({ 'Ok' : CycleInfo, 'Err' : IDL.Text });
  const Result_13 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text });
  const LogEntry = IDL.Record({
    'ts' : IDL.Nat64,
    'msg' : IDL.Text,
    'kind' : IDL.Text,
  });
  const Result_14 = IDL.Variant({ 'Ok' : IDL.Vec(LogEntry), 'Err' : IDL.Text });
  const Result_15 = IDL.Variant({
    'Ok' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Principal, IDL.Text))),
    'Err' : IDL.Text,
  });
  const ByteReadResponse = IDL.Record({
    'total' : IDL.Nat64,
    'data' : IDL.Vec(IDL.Nat8),
    'hash' : IDL.Text,
    'name' : IDL.Text,
  });
  const Result_16 = IDL.Variant({
    'Ok' : IDL.Opt(ByteReadResponse),
    'Err' : IDL.Text,
  });
  const AppInstallRequest = IDL.Record({ 'app_id' : IDL.Text });
  const Result_17 = IDL.Variant({ 'Ok' : IDL.Vec(UserApp), 'Err' : EgoError });
  const AppReInstallRequest = IDL.Record({ 'canister_id' : IDL.Principal });
  const AppUpgradeRequest = IDL.Record({ 'wallet_id' : IDL.Principal });
  const Result_18 = IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : EgoError });
  const WalletCycleChargeRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'cycle' : IDL.Nat,
    'comment' : IDL.Text,
  });
  const WalletCycleChargeResponse = IDL.Record({ 'ret' : IDL.Bool });
  const Result_19 = IDL.Variant({
    'Ok' : WalletCycleChargeResponse,
    'Err' : EgoError,
  });
  const CashFlowType = IDL.Variant({
    'CHARGE' : IDL.Null,
    'RECHARGE' : IDL.Null,
  });
  const CashFlow = IDL.Record({
    'balance' : IDL.Nat,
    'operator' : IDL.Principal,
    'created_at' : IDL.Nat64,
    'comment' : IDL.Text,
    'cycles' : IDL.Nat,
    'cash_flow_type' : CashFlowType,
  });
  const Result_20 = IDL.Variant({ 'Ok' : IDL.Vec(CashFlow), 'Err' : EgoError });
  const Result_21 = IDL.Variant({ 'Ok' : IDL.Principal, 'Err' : EgoError });
  const OrderStatus = IDL.Variant({ 'NEW' : IDL.Null, 'SUCCESS' : IDL.Null });
  const Order = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'status' : OrderStatus,
    'from' : IDL.Vec(IDL.Nat8),
    'memo' : IDL.Nat64,
    'amount' : IDL.Float32,
    'last_update' : IDL.Nat64,
    'wallet_id' : IDL.Principal,
  });
  const Result_22 = IDL.Variant({ 'Ok' : IDL.Vec(Order), 'Err' : EgoError });
  const Result_23 = IDL.Variant({ 'Ok' : IDL.Nat64, 'Err' : EgoError });
  return IDL.Service({
    'admin_wallet_app_get' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result],
        [],
      ),
    'admin_wallet_app_transfer' : IDL.Func(
        [IDL.Principal, IDL.Principal],
        [Result_1],
        [],
      ),
    'admin_wallet_cycle_recharge' : IDL.Func(
        [AdminWalletCycleRechargeRequest],
        [Result_2],
        [],
      ),
    'admin_wallet_provider_add' : IDL.Func(
        [AdminWalletProviderAddRequest],
        [Result_1],
        [],
      ),
    'admin_wallet_provider_delete' : IDL.Func([IDL.Principal], [Result_1], []),
    'admin_wallet_provider_list' : IDL.Func([], [Result_3], []),
    'app_main_get' : IDL.Func([IDL.Text], [Result_4], []),
    'app_main_list' : IDL.Func([], [Result_5], []),
    'app_main_release' : IDL.Func([EgoStoreApp], [Result_2], []),
    'backup_change_status' : IDL.Func([BackupStatus], [Result_6], []),
    'backup_info_get' : IDL.Func([], [Result_7], []),
    'backup_job_list' : IDL.Func([], [Result_8], []),
    'balance_get' : IDL.Func([], [Result_9], ['query']),
    'ego_canister_add' : IDL.Func([IDL.Text, IDL.Principal], [Result_6], []),
    'ego_canister_list' : IDL.Func([], [Result_10], []),
    'ego_canister_remove' : IDL.Func([IDL.Text, IDL.Principal], [Result_6], []),
    'ego_controller_add' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_controller_remove' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_controller_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_6], []),
    'ego_cycle_check' : IDL.Func([], [Result_6], []),
    'ego_cycle_estimate_set' : IDL.Func([IDL.Nat64], [Result_6], []),
    'ego_cycle_history' : IDL.Func([], [Result_11], []),
    'ego_cycle_info' : IDL.Func([], [Result_12], []),
    'ego_cycle_recharge' : IDL.Func([IDL.Nat], [Result_6], []),
    'ego_cycle_threshold_get' : IDL.Func([], [Result_9], []),
    'ego_is_op' : IDL.Func([], [Result_13], ['query']),
    'ego_is_owner' : IDL.Func([], [Result_13], ['query']),
    'ego_is_user' : IDL.Func([], [Result_13], ['query']),
    'ego_log_list' : IDL.Func([IDL.Nat64], [Result_14], ['query']),
    'ego_op_add' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_op_list' : IDL.Func([], [Result_15], []),
    'ego_op_remove' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_owner_add' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_owner_add_with_name' : IDL.Func(
        [IDL.Text, IDL.Principal],
        [Result_6],
        [],
      ),
    'ego_owner_list' : IDL.Func([], [Result_15], []),
    'ego_owner_remove' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_owner_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_6], []),
    'ego_runtime_cycle_threshold_get' : IDL.Func([], [Result_9], []),
    'ego_user_add' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_user_list' : IDL.Func([], [Result_15], []),
    'ego_user_remove' : IDL.Func([IDL.Principal], [Result_6], []),
    'ego_user_set' : IDL.Func([IDL.Vec(IDL.Principal)], [Result_6], []),
    'job_data_backup' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64],
        [Result_16],
        [],
      ),
    'job_data_export' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Nat64, IDL.Nat64],
        [Result_16],
        [],
      ),
    'job_data_restore' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Nat8)],
        [Result_6],
        [],
      ),
    'wallet_app_install' : IDL.Func([IDL.Text], [Result], []),
    'wallet_app_install_v2' : IDL.Func([AppInstallRequest], [Result], []),
    'wallet_app_list' : IDL.Func([], [Result_17], []),
    'wallet_app_reinstall_by_wallet_v2' : IDL.Func(
        [AppReInstallRequest],
        [Result_1],
        [],
      ),
    'wallet_app_remove' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_app_upgrade' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_app_upgrade_by_wallet' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_app_upgrade_by_wallet_v2' : IDL.Func(
        [AppReInstallRequest],
        [Result_1],
        [],
      ),
    'wallet_app_upgrade_v2' : IDL.Func([AppUpgradeRequest], [Result_1], []),
    'wallet_canister_track' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_canister_track_self' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_canister_untrack' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_canister_untrack_self' : IDL.Func([IDL.Principal], [Result_1], []),
    'wallet_cycle_balance' : IDL.Func([], [Result_18], []),
    'wallet_cycle_charge' : IDL.Func(
        [WalletCycleChargeRequest],
        [Result_19],
        [],
      ),
    'wallet_cycle_list' : IDL.Func([], [Result_20], []),
    'wallet_main_new' : IDL.Func([IDL.Principal], [Result], []),
    'wallet_main_register' : IDL.Func([IDL.Principal], [Result_21], []),
    'wallet_order_list' : IDL.Func([], [Result_22], []),
    'wallet_order_new' : IDL.Func([IDL.Float32], [Result_23], []),
    'wallet_order_notify' : IDL.Func([IDL.Nat64], [Result_2], []),
    'wallet_tenant_get' : IDL.Func([], [Result_21], []),
  });
};
export const init = ({ IDL }) => { return []; };
