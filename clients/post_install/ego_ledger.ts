// import {getActor} from '@/settings/agent';
// import {getCanisterId,} from '@/settings/utils';
// import {idlFactory as ledgerFactory} from '@/idls/ego_ledger.idl';
// import {_SERVICE} from '@/idls/ego_ledger';
// import {Principal} from '@dfinity/principal';
// import {identity} from "@/settings/identity";
//
// export const ledgerActor = getActor<_SERVICE>(
//     identity,
//     ledgerFactory,
//     getCanisterId('ego_ledger')!,
// );
//
// export const store_id = getCanisterId('ego_store');
// export const ledger_id = getCanisterId('ledger_config');
// export const crond_id = getCanisterId('ego_crond');
//
// export const ledgerPostInstall = async () => {
//     const ledger = await ledgerActor;
//
//     console.log(`=== post install script of ego_ledger starts: ==\n`);
//
//     console.log(`1. init_ledger_canister store_id:${store_id},ledger_id:${ledger_id} ,crond_id:${crond_id}\n`);
//
//     let resp1 = await ledger.init_ledger({
//         store_canister_id: Principal.fromText(store_id!),
//         ledger_canister_id: Principal.fromText(ledger_id!),
//         cron_canister_id: Principal.fromText(crond_id!),
//         start: BigInt(0),
//         length: BigInt(500),
//         init_method_name: 'get_balance'
//     });
//     console.log(resp1);
// };
