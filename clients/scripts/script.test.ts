import { getActor } from '@/settings/agent';
import { _SERVICE } from '@/idls/ego_dev';
import { identity } from '@/settings/identity';
import { idlFactory } from '@/idls/ego_dev.idl';
import { getCanisterId } from '@/settings/utils';

export const deployerActor = getActor<_SERVICE>(
  identity,
  idlFactory,
  getCanisterId('ego_dev')!,
);

describe('scripts', () => {
  test('set_auditor', async () => {
    const deployer = await deployerActor;

    let user_names = ['aaa', 'neeboo'];

    for (const user_name of user_names) {
      console.log(`\t\t set role for ${user_name}\n`);
      let resp1 = await deployer.user_main_list({ name: user_name });
      for (const user of (resp1 as any).Ok.users) {
        let resp = await deployer.user_role_set({
          user_id: user.user_id,
          is_app_auditor: true,
          is_manager: true,
        });
        console.log(resp);
      }
    }
  });

  // test('notify_payment', async () => {
  //   const deployer = await deployerActor;

  //   let memo = 3;
  //   let resp = await deployer.notify_payment({ memo: BigInt(memo) });
  //   console.log(resp);
  // });
});
