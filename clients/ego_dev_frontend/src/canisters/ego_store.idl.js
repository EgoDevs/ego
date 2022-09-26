export const idlFactory = ({ IDL }) => {
  const Version = IDL.Record({
    'major' : IDL.Nat32,
    'minor' : IDL.Nat32,
    'patch' : IDL.Nat32,
  });
  const ApproveAppVersionRequest = IDL.Record({
    'version' : Version,
    'app_id' : IDL.Text,
  });
  const ApproveAppVersionResponse = IDL.Record({ 'ret' : IDL.Bool });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({
    'Ok' : ApproveAppVersionResponse,
    'Err' : EgoError,
  });
  const CreateOrderRequest = IDL.Record({ 'app_id' : IDL.Text });
  const OrderStatus = IDL.Variant({ 'NEW' : IDL.Null, 'SUCCESS' : IDL.Null });
  const OrderType = IDL.Variant({ 'APP' : IDL.Null, 'RECHARGE' : IDL.Null });
  const Order = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'status' : OrderStatus,
    'from' : IDL.Vec(IDL.Nat8),
    'memo' : IDL.Nat64,
    'user_id' : IDL.Principal,
    'app_id' : IDL.Opt(IDL.Text),
    'order_type' : OrderType,
    'amount' : IDL.Float32,
  });
  const CreateOrderResponse = IDL.Record({ 'order' : Order });
  const Result_1 = IDL.Variant({
    'Ok' : CreateOrderResponse,
    'Err' : EgoError,
  });
  const CreateRechargeRequest = IDL.Record({ 'amount' : IDL.Float32 });
  const AppStatus = IDL.Variant({
    'NEW' : IDL.Null,
    'CLOSED' : IDL.Null,
    'RELEASED' : IDL.Null,
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
  const CanisterType = IDL.Variant({
    'BACKEND' : IDL.Null,
    'ASSET' : IDL.Null,
  });
  const Wasm = IDL.Record({
    'bucket_id' : IDL.Principal,
    'canister_id' : IDL.Opt(IDL.Principal),
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
    'price' : IDL.Float32,
    'versions' : IDL.Vec(AppVersion),
    'audit_version' : IDL.Opt(Version),
  });
  const CreatedAppResponse = IDL.Record({ 'apps' : IDL.Vec(App) });
  const Result_2 = IDL.Variant({ 'Ok' : CreatedAppResponse, 'Err' : EgoError });
  const GetAppRequest = IDL.Record({ 'app_id' : IDL.Text });
  const GetAppResponse = IDL.Record({ 'app' : App });
  const Result_3 = IDL.Variant({ 'Ok' : GetAppResponse, 'Err' : EgoError });
  const GetWalletResponse = IDL.Record({
    'wallet_id' : IDL.Opt(IDL.Principal),
  });
  const Result_4 = IDL.Variant({ 'Ok' : GetWalletResponse, 'Err' : EgoError });
  const InitStoreRequest = IDL.Record({
    'ego_crond_id' : IDL.Principal,
    'ego_bucket_id' : IDL.Principal,
    'ego_ledger_id' : IDL.Principal,
  });
  const InitStoreResponse = IDL.Record({ 'ret' : IDL.Bool });
  const Result_5 = IDL.Variant({ 'Ok' : InitStoreResponse, 'Err' : EgoError });
  const QueryParam = IDL.Variant({
    'ByCategory' : IDL.Record({ 'category' : Category }),
  });
  const ListAppRequest = IDL.Record({ 'query_param' : QueryParam });
  const AppInfo = IDL.Record({
    'name' : IDL.Text,
    'user_id' : IDL.Principal,
    'app_id' : IDL.Text,
    'release_version' : IDL.Opt(Version),
    'category' : Category,
    'price' : IDL.Float32,
  });
  const ListAppResponse = IDL.Record({ 'apps' : IDL.Vec(AppInfo) });
  const Result_6 = IDL.Variant({ 'Ok' : ListAppResponse, 'Err' : EgoError });
  const ListOrdersResponse = IDL.Record({ 'orders' : IDL.Vec(Order) });
  const Result_7 = IDL.Variant({ 'Ok' : ListOrdersResponse, 'Err' : EgoError });
  const ListUserRequest = IDL.Record({ 'name' : IDL.Text });
  const User = IDL.Record({
    'installed_apps' : IDL.Vec(IDL.Text),
    'orders' : IDL.Vec(IDL.Nat64),
    'name' : IDL.Text,
    'user_id' : IDL.Principal,
    'is_app_auditer' : IDL.Bool,
    'is_app_developer' : IDL.Bool,
    'created_apps' : IDL.Vec(IDL.Text),
    'wallet_id' : IDL.Opt(IDL.Principal),
    'is_manager' : IDL.Bool,
  });
  const ListUserResponse = IDL.Record({ 'users' : IDL.Vec(User) });
  const Result_8 = IDL.Variant({ 'Ok' : ListUserResponse, 'Err' : EgoError });
  const MeResponse = IDL.Record({ 'user' : User });
  const Result_9 = IDL.Variant({ 'Ok' : MeResponse, 'Err' : EgoError });
  const NewAppVersionRequest = IDL.Record({
    'version' : Version,
    'app_id' : IDL.Text,
  });
  const NotifyPaymentRequest = IDL.Record({ 'memo' : IDL.Nat64 });
  const RegisterAppRequest = IDL.Record({
    'name' : IDL.Text,
    'app_id' : IDL.Text,
    'category' : Category,
    'price' : IDL.Float32,
  });
  const SetFrontendAddressRequest = IDL.Record({
    'canister_id' : IDL.Principal,
    'version' : Version,
    'app_id' : IDL.Text,
  });
  const SetRoleRequest = IDL.Record({
    'user_id' : IDL.Principal,
    'is_app_auditer' : IDL.Bool,
    'is_app_developer' : IDL.Bool,
    'is_manager' : IDL.Bool,
  });
  const SetRoleResponse = IDL.Record({ 'ret' : IDL.Bool });
  const Result_10 = IDL.Variant({ 'Ok' : SetRoleResponse, 'Err' : EgoError });
  const UpgradeWalletRequest = IDL.Record({ 'version' : Version });
  return IDL.Service({
    'approve_app_version' : IDL.Func([ApproveAppVersionRequest], [Result], []),
    'create_app_order' : IDL.Func([CreateOrderRequest], [Result_1], []),
    'create_recharge_order' : IDL.Func([CreateRechargeRequest], [Result_1], []),
    'created_apps' : IDL.Func([], [Result_2], []),
    'get_app' : IDL.Func([GetAppRequest], [Result_3], ['query']),
    'get_wallet' : IDL.Func([], [Result_4], ['query']),
    'init_store' : IDL.Func([InitStoreRequest], [Result_5], []),
    'list_app' : IDL.Func([ListAppRequest], [Result_6], ['query']),
    'list_orders' : IDL.Func([], [Result_7], ['query']),
    'list_user' : IDL.Func([ListUserRequest], [Result_8], ['query']),
    'list_wait_for_audit_app' : IDL.Func([], [Result_2], []),
    'me' : IDL.Func([], [Result_9], ['query']),
    'new_app_version' : IDL.Func([NewAppVersionRequest], [Result_5], []),
    'notify_payment' : IDL.Func([NotifyPaymentRequest], [Result_5], []),
    'register_app' : IDL.Func([RegisterAppRequest], [Result_3], []),
    'register_developer' : IDL.Func([ListUserRequest], [Result_9], []),
    'register_user' : IDL.Func([ListUserRequest], [Result_9], []),
    'reject_app_version' : IDL.Func([NewAppVersionRequest], [Result_5], []),
    'release_app_version' : IDL.Func([NewAppVersionRequest], [Result_5], []),
    'revoke_app_version' : IDL.Func([NewAppVersionRequest], [Result_5], []),
    'set_frontend_address' : IDL.Func(
        [SetFrontendAddressRequest],
        [Result_5],
        [],
      ),
    'set_role' : IDL.Func([SetRoleRequest], [Result_10], []),
    'submit_app_version' : IDL.Func([NewAppVersionRequest], [Result_10], []),
    'upgrade_wallet' : IDL.Func([UpgradeWalletRequest], [Result_10], []),
  });
};
export const init = ({ IDL }) => { return []; };
