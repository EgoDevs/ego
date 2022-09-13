import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { getCanisterId, hasOwnProperty } from '@/settings/utils';
import { getActor } from '@/settings/agent';
import { identity } from '@/settings/identity';
import { idlFactory } from '@/idls/ego_cron.idl';
import { Principal } from '@dfinity/principal';
import {_SERVICE} from '@/idls/ego_cron';

const method = 'test_method';

export const cron_actor = getActor<_SERVICE>(
  identity,
  idlFactory,
  getCanisterId('ego_cron')!,
);

const cron_principal = Principal.fromText(getCanisterId('ego_cron')!!);

beforeAll(async () => {

});

describe('cron task operation', () => {
  test('task_main_add and task_main_cencel', async () => {
    let cron_canister = await cron_actor;

    // add task success
    let response1 = await cron_canister.task_main_add({
      canister_id: cron_principal,
      method: method,
      interval: {PerHour: null},
    });

    console.log(response1)
    expect(0 == response1.Ok.task_id)

    // same task get the same task_id
    let response2 = await cron_canister.task_main_add({
      canister_id: cron_principal,
      method: method,
      interval: {PerHour: null},
    });

    expect(0 == response2.Ok.task_id)

    // cancel not exists task
    let response3 = await cron_canister.task_main_cancel({
      task_id: 1
    });

    expect(2001 == response3.Err.code)

    // cancel not exists task
    let response4 = await cron_canister.task_main_cancel({
      task_id: 0
    });

    expect(response4.hasOwnProperty("Ok"))
  });
});
