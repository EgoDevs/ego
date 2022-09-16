import shell from 'shelljs';
import { getCanisterId } from '@/settings/utils';
import { idlFactory as cronIdl } from '@/idls/ego_cron.idl';
import { _SERVICE as cronService } from '@/idls/ego_cron';
import { identity } from '@/settings/identity';
import { getActor } from '@/settings/agent';
import { Principal } from '@dfinity/principal';

export const crondActor = getActor<cronService>(
  identity,
  cronIdl,
  getCanisterId('ego_crond')!,
);

export const cronPostInstall = async () => {
  let crond = await crondActor

  console.log(`=== post install script of ego_crond starts: ===\n`);

  console.log(`1.add ego_store as ego_crond manager`);
  let resp1 = await crond.add_managers({
    managers: [Principal.fromText(getCanisterId('ego_store')!)],
  });
  console.log(resp1);

  console.log(`2.add ego_ledger as ego_crond canister`);
  let resp2 = await crond.add_canisters({
    canisters: [Principal.fromText(getCanisterId('ego_ledger')!)],
  });
  console.log(resp2);

  console.log(`3.add ego_wallt as ego_crond canister`);
  let resp3 = await crond.add_canisters({
    canisters: [Principal.fromText(getCanisterId('ego_wallet')!)],
  });
  console.log(resp3);
};
