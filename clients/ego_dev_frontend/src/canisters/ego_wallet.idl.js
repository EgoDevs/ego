export const idlFactory = ({ IDL }) => {
  const SendCyclesArgs = IDL.Record({
    'canister' : IDL.Principal,
    'amount' : IDL.Nat,
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : IDL.Text });
  const CanisterStatusEnum = IDL.Variant({
    'stopped' : IDL.Null,
    'stopping' : IDL.Null,
    'running' : IDL.Null,
  });
  const CanisterSettings = IDL.Record({
    'controller' : IDL.Opt(IDL.Principal),
    'freezing_threshold' : IDL.Opt(IDL.Nat),
    'controllers' : IDL.Opt(IDL.Vec(IDL.Principal)),
    'memory_allocation' : IDL.Opt(IDL.Nat),
    'compute_allocation' : IDL.Opt(IDL.Nat),
  });
  const AppNativeStatus = IDL.Record({
    'status' : CanisterStatusEnum,
    'memory_size' : IDL.Nat,
    'cycles' : IDL.Nat,
    'settings' : CanisterSettings,
    'module_hash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const CanisterType = IDL.Variant({
    'BACKEND' : IDL.Null,
    'ASSET' : IDL.Null,
  });
  const CheckAppStatus = IDL.Record({
    'canister_id' : IDL.Principal,
    'native_status' : AppNativeStatus,
    'app_id' : IDL.Text,
    'canister_type' : CanisterType,
  });
  const CheckAppStatusResponse = IDL.Record({
    'app_status_result' : IDL.Vec(CheckAppStatus),
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result_1 = IDL.Variant({
    'Ok' : CheckAppStatusResponse,
    'Err' : EgoError,
  });
  const GetAppRequest = IDL.Record({ 'app_id' : IDL.Text });
  const AppStatus = IDL.Variant({
    'NEW' : IDL.Null,
    'CLOSED' : IDL.Null,
    'RELEASED' : IDL.Null,
  });
  const Version = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const Category = IDL.Variant({ 'System' : IDL.Null, 'Vault' : IDL.Null });
  const AppVersionStatus = IDL.Variant({
    'NEW' : IDL.Null,
    'REJECTED' : IDL.Null,
    'SUBMITTED' : IDL.Null,
    'REVOKED' : IDL.Null,
    'RELEASED' : IDL.Null,
    'APPROVED' : IDL.Null,
  });
  const Wasm = IDL.Record({
    'bucket_id' : IDL.Principal,
    'version' : Version,
    'app_id' : IDL.Text,
    'canister_type' : CanisterType,
    'file_id' : IDL.Text,
  });
  const AppVersion = IDL.Record({
    'status' : AppVersionStatus,
    'bucket_id' : IDL.Principal,
    'version' : Version,
    'app_id' : IDL.Text,
    'wasms' : IDL.Vec(Wasm),
  });
  const App = IDL.Record({
    'status' : AppStatus,
    'bucket_id' : IDL.Principal,
    'name' : IDL.Text,
    'user_id' : IDL.Principal,
    'app_id' : IDL.Text,
    'release_version' : IDL.Opt(Version),
    'category' : Category,
    'price' : IDL.Nat8,
    'versions' : IDL.Vec(AppVersion),
    'audit_version' : IDL.Opt(Version),
  });
  const GetAppResponse = IDL.Record({ 'app' : App });
  const Result_2 = IDL.Variant({ 'Ok' : GetAppResponse, 'Err' : EgoError });
  const Canister = IDL.Record({
    'canister_id' : IDL.Principal,
    'canister_type' : CanisterType,
  });
  const UserApp = IDL.Record({
    'version' : Version,
    'app_id' : IDL.Text,
    'canisters' : IDL.Vec(Canister),
  });
  const UserAppResponse = IDL.Record({ 'canisters' : IDL.Vec(UserApp) });
  const Result_3 = IDL.Variant({ 'Ok' : UserAppResponse, 'Err' : EgoError });
  const InitWalletCanister = IDL.Record({
    'wallet_version' : Version,
    'cron_canister_id' : IDL.Principal,
    'store_canister_id' : IDL.Principal,
  });
  const Result_4 = IDL.Variant({ 'Ok' : IDL.Null, 'Err' : EgoError });
  const AppInstallRequest = IDL.Record({
    'version' : Version,
    'cycles' : IDL.Nat,
    'app_id' : IDL.Text,
  });
  const AppInstallResponse = IDL.Record({
    'canister_ids' : IDL.Vec(IDL.Principal),
    'version' : Version,
  });
  const Result_5 = IDL.Variant({ 'Ok' : AppInstallResponse, 'Err' : EgoError });
  const BalanceResult = IDL.Record({ 'amount' : IDL.Nat64 });
  const Result_6 = IDL.Variant({ 'Ok' : BalanceResult, 'Err' : EgoError });
  const BalanceResult_1 = IDL.Record({ 'amount' : IDL.Nat });
  const Result_7 = IDL.Variant({ 'Ok' : BalanceResult_1, 'Err' : EgoError });
  const SendCyclesArgs_1 = IDL.Record({
    'canister' : IDL.Principal,
    'amount' : IDL.Nat64,
  });
  const Result_8 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  return IDL.Service({
    'app_deposit' : IDL.Func([SendCyclesArgs], [Result], []),
    'check_app_status' : IDL.Func([], [Result_1], ['query']),
    'get_app' : IDL.Func([GetAppRequest], [Result_2], []),
    'get_apps' : IDL.Func([], [Result_3], ['query']),
    'init_wallet_canister' : IDL.Func([InitWalletCanister], [Result_4], []),
    'install_app' : IDL.Func([AppInstallRequest], [Result_5], []),
    'uninstall_app' : IDL.Func([AppInstallRequest], [Result_5], []),
    'upgrade_app' : IDL.Func([AppInstallRequest], [Result_5], []),
    'wallet_balance' : IDL.Func([], [Result_6], ['query']),
    'wallet_balance128' : IDL.Func([], [Result_7], ['query']),
    'wallet_send' : IDL.Func([SendCyclesArgs_1], [Result_8], []),
  });
};
export const init = ({ IDL }) => { return []; };
