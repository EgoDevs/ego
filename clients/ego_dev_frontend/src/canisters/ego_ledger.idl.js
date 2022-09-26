export const idlFactory = ({ IDL }) => {
  const Tokens = IDL.Record({ 'e8s' : IDL.Nat64 });
  const AddPaymentRequest = IDL.Record({
    'to' : IDL.Vec(IDL.Nat8),
    'from' : IDL.Vec(IDL.Nat8),
    'memo' : IDL.Nat64,
    'amount' : Tokens,
  });
  const AddPaymentResponse = IDL.Record({
    'result' : IDL.Bool,
    'memo' : IDL.Nat64,
  });
  const EgoError = IDL.Record({ 'msg' : IDL.Text, 'code' : IDL.Nat16 });
  const Result = IDL.Variant({ 'Ok' : AddPaymentResponse, 'Err' : EgoError });
  const InitLedgerRequest = IDL.Record({
    'init_method_name' : IDL.Text,
    'cron_canister_id' : IDL.Principal,
    'start' : IDL.Nat64,
    'store_canister_id' : IDL.Principal,
    'length' : IDL.Nat64,
    'ledger_canister_id' : IDL.Principal,
  });
  const Result_1 = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : EgoError });
  return IDL.Service({
    'add_payment' : IDL.Func([AddPaymentRequest], [Result], []),
    'init_ledger' : IDL.Func([InitLedgerRequest], [Result_1], []),
    'match_payment_task' : IDL.Func([], [Result_1], []),
  });
};
export const init = ({ IDL }) => { return []; };
