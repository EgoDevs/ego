export const idlFactory = ({ IDL }) => {
  const CronInterval = IDL.Variant({
    'PerHour' : IDL.Null,
    'PerDay' : IDL.Null,
    'PerMinute' : IDL.Null,
    'PerSecond' : IDL.Null,
  });
  const AddCronTaskRequest = IDL.Record({
    'method' : IDL.Text,
    'interval' : CronInterval,
    'canister_id' : IDL.Principal,
  });
  const AddCronTaskResponse = IDL.Record({
    'task_id' : IDL.Nat64,
    'duration_nano' : IDL.Nat64,
  });
  const EgoCronError = IDL.Variant({
    'AlreadyHasTask' : IDL.Null,
    'UnknownError' : IDL.Text,
    'TaskNotFound' : IDL.Null,
    'CancelFail' : IDL.Nat64,
  });
  const Result = IDL.Variant({
    'Ok' : AddCronTaskResponse,
    'Err' : EgoCronError,
  });
  const AddManagersRequest = IDL.Record({
    'managers' : IDL.Vec(IDL.Principal),
  });
  const AddManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const Result_1 = IDL.Variant({
    'Ok' : AddManagersResponse,
    'Err' : EgoCronError,
  });
  const CancelCronTaskRequest = IDL.Record({ 'canister_id' : IDL.Principal });
  const CancelCronTaskResponse = IDL.Record({
    'task_id' : IDL.Nat64,
    'interval' : CronInterval,
  });
  const Result_2 = IDL.Variant({
    'Ok' : CancelCronTaskResponse,
    'Err' : EgoCronError,
  });
  const GetCronTaskRequest = IDL.Record({ 'canister_id' : IDL.Principal });
  const GetCronTaskResponse = IDL.Record({
    'method' : IDL.Text,
    'task_id' : IDL.Nat64,
    'interval' : CronInterval,
    'canister_id' : IDL.Principal,
  });
  const Result_3 = IDL.Variant({
    'Ok' : GetCronTaskResponse,
    'Err' : EgoCronError,
  });
  const RmManagersRequest = IDL.Record({ 'managers' : IDL.Vec(IDL.Principal) });
  const RmManagersResponse = IDL.Record({ 'manager_count' : IDL.Nat64 });
  const Result_4 = IDL.Variant({
    'Ok' : RmManagersResponse,
    'Err' : EgoCronError,
  });
  return IDL.Service({
    'add_cron_task' : IDL.Func([AddCronTaskRequest], [Result], []),
    'add_managers' : IDL.Func([AddManagersRequest], [Result_1], []),
    'cancel_cron_task' : IDL.Func([CancelCronTaskRequest], [Result_2], []),
    'get_cron_task' : IDL.Func([GetCronTaskRequest], [Result_3], ['query']),
    'is_manager' : IDL.Func([IDL.Principal], [IDL.Text], ['query']),
    'remove_managers' : IDL.Func([RmManagersRequest], [Result_4], []),
  });
};
export const init = ({ IDL }) => { return []; };
