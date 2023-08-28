import { idlFactory as Idl } from '../idls/ego_tenant.idl';
import { _SERVICE as Service } from '../idls/ego_tenant';

import { getActor, identity, getCanisterId } from '@ego-js/utils';
import fs from "fs";
import {Principal} from "@dfinity/principal";


describe('reset_next_check_time', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('reset_next_check_time', async () => {
    const actor = await egoActor;

    const response = await actor.reset_next_check_time();

    console.log(response)
  });
});

describe('tenant_logs', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('logs', async () => {
    const actor = await egoActor;

    const response = await actor.ego_log_list(BigInt(20));
    response.Ok.forEach(log => {
      let date = new Date(Number(log.ts))
      console.log(date + ' / ' + log.msg)
    })

  });
});

describe('export_tasks', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('export', async () => {
    const actor = await egoActor;

    const response = await actor.job_data_export('tasks', []);
    const data = response.Ok[0].data
    const json = JSON.parse(Buffer.from(data).toString()) as { [key: string]: any }[];
    fs.writeFileSync('/tmp/tasks.json', JSON.stringify(json));

  });
});

describe('admin_task_check', () => {
  const egoActor = getActor<Service>(identity(), Idl, getCanisterId('ego_tenant')!);

  it('admin_task_check', async () => {
    const actor = await egoActor;

    const response = await actor.admin_task_check(Principal.fromText('ws5yv-piaaa-aaaah-ac6tq-cai'));
    console.log(response)
  });
});
