// import { getActor } from '@/settings/agent';
// import { getCanisterId } from '@/settings/utils';
// import { ActorSubclass } from '@dfinity/agent';
// import { idlFactory } from '@/idls/ego_wallet.idl';
// import { idlFactory as storeIDL } from '@/idls/ego_dev.idl';
// import { _SERVICE as StoreService } from '@/idls/ego_dev';
//
// import { BalanceResult, UserAppResponse, _SERVICE } from '@/idls/ego_wallet';
//
// import { Ed25519KeyIdentity } from '@dfinity/identity';
// import { Principal } from '@dfinity/principal';
// import { endUsers } from '@/fixtures/identities';
// import { idlFactory as bucketFactory } from '@/idls/ego_bucket.idl';
// import { identity } from '@/settings/identity';
// import { ManagementApi } from '@/settings/manager';
//
// export const store_id = getCanisterId('ego_store');
// export const bucket_id = getCanisterId('ego_bucket');
// export const wallet_id = getCanisterId('ego_wallet');
// export const crond_id = getCanisterId('ego_crond');
//
// export const walletActor = getActor<_SERVICE>(
//   Ed25519KeyIdentity.fromJSON(JSON.stringify(endUsers[0]?.identity)),
//   idlFactory,
//   wallet_id!,
// );
//
// export const walletDeployedActor = getActor<_SERVICE>(
//     identity,
//     idlFactory,
//     wallet_id!,
// );
//
//
// export const storeActor = getActor<StoreService>(identity, storeIDL, store_id!);
// export const user_principal = endUsers[0].principal;
// export const existing_app_id = 'app_1';
// export const ego_wallet = 'ego_wallet';
//
// export const existing_version = {
//   major: 1,
//   minor: 0,
//   patch: 0,
// };
//
// export const new_version = {
//   major: 1,
//   minor: 0,
//   patch: 1,
// };
//
// export const walletPostInstall = async () => {
//   console.log("identity deployed wallet is "+identity.getPrincipal().toText());
//
//   const walletDeployed = await walletDeployedActor;
//
//   walletDeployed.set_owner(Principal.fromText(user_principal!));
//
//   const wallet = await walletActor;
//   console.log(`
//     post install script of ego_wallet starts: \n
//   `);
//
//   console.log(`
//     1. init_wallet_canister ${store_id} : \n
//   `);
//   await wallet.init_wallet_canister({
//     cron_canister_id: Principal.fromText(crond_id!),
//     store_canister_id: Principal.fromText(store_id!),
//     wallet_version: existing_version,
//     // managers: [Principal.fromText(store_id!),identity.getPrincipal(),Principal.fromText(wallet_id!)],
//   });
//   console.log(`
//     1.1. init_wallet_canister end \n
//   `);
//   const balance = await wallet.wallet_balance();
//   console.log(
//     `
//     wallet balance is: \n
//   ` + (balance as { Ok: BalanceResult }).Ok.amount,
//   );
//
//   let resp = await (await storeActor).get_app({ app_id: 'app_1' });
//
//   console.log('get app from store:' + resp);
//
//   console.log(`
//     2. call install_app start: \n
//       `);
//
//   const installAppResponse = await wallet.install_app({
//     version: existing_version,
//     cycles: BigInt('910000000000'),
//     app_id: existing_app_id,
//   });
//   console.log(installAppResponse);
//
//   console.log(`
//     3. call upgrade_app start: \n
//       `);
//   const upgradeAppResponse = await wallet.upgrade_app({
//     version: new_version,
//     cycles: BigInt('910000000000'),
//     app_id: existing_app_id,
//   });
//   console.log(upgradeAppResponse);
//
//   console.log('dfx canister update-settings --all --add-controller');
//
//   await ManagementApi.updateSettings('ego_wallet', wallet_id!, {
//     freezing_threshold: [],
//     controllers: [
//       [
//         Principal.fromText(user_principal),
//         Principal.fromText(wallet_id!),
//       ],
//     ],
//     memory_allocation: [],
//     compute_allocation: [],
//   });
//
//   await wallet.add_store_of_controller();
//   console.log('add store as controller successfully!');
//   await wallet.remove_store_of_controller();
//   console.log('remove store as controller successfully!');
//
//   // console.log("enduser's principal is " + endUsers[0].principal);
//
//   // //get app from wallet
//   // let app_resp = await wallet.get_apps();
//   // let arr_names = app_resp.Ok.canisters
//   // for(var i = 0; i<arr_names.length; i++) {
//   //   console.log(arr_names[i])
//   // }
// };
