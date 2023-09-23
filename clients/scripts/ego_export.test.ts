import { idlFactory as Idl } from '../idls/ego_store.idl';
import { _SERVICE as Service } from '../idls/ego_store';

import { getActor, identity, getCanisterId } from '@ego-js/utils';


test('export_datas', async () => {
  const actor =
    // getActor use idl types
    await getActor<Service>(
      // use credential identity, owner of canister
      identity(),
      // use idlFactory from generated file
      Idl,
      // get canister ID for 'ego_deployer', `configs/ego_deployer.json` is generated
      getCanisterId('ego_dev')!,
    );

  const data = await actor.job_data_export('developers', 0, 2, 0);
  const json = JSON.parse(Buffer.from(data.Ok[0].data).toString()) as { [key: string]: any }[];
  console.log(json);
});