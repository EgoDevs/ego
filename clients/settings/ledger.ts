import { Actor, ActorSubclass, HttpAgent, SignIdentity } from '@dfinity/agent';
import { InterfaceFactory } from '@dfinity/candid/lib/cjs/idl';
import { idlFactory as ledgerIdl } from '@/idls/ledger.idl';
import {
  AccountIdentifier,
  Memo,
  SubAccount,
  TimeStamp,
  Tokens,
  TransferArgs,
  TransferResult,
  _SERVICE as ledgerService,
} from '@/idls/ledger';
import { Principal } from '@dfinity/principal';
import { getActor } from './agent';
import { fromHexString, getIdentityFromPhrase } from './identity';
import { getCanisterId } from './utils';
import {
  accountIdentifierToBytes,
  fromSubAccountId,
  principalToAccountIdentifier,
} from './converter';

export const ledgerCanister = getCanisterId('ledger_config');
export const localLedgerPhrase =
  'steel obey anxiety vast clever relax million girl cost pond elbow bridge hill health toilet desk sleep grid boost flavor shy cry armed mass';
export const ledgerIdentity = getIdentityFromPhrase(localLedgerPhrase);

export interface SendOpts {
  fee?: bigint;
  memo?: Memo;
  from_subaccount?: number;
  created_at_time?: Date;
}

export async function getICPFromLedger({
  principal,
  subaccount,
  memo,
  amount,
}: {
  principal: Principal;
  subaccount?: Uint8Array;
  memo: bigint;
  amount: bigint;
}): Promise<TransferResult> {
  const actor = await getActor<ledgerService>(
    ledgerIdentity,
    ledgerIdl,
    ledgerCanister!,
  );

  const receiver = principalToAccountIdentifier(principal, subaccount);

  return await actor.transfer({
    to: Array.from(new Uint8Array(fromHexString(receiver))),
    fee: { e8s: BigInt(10000) },
    memo: memo,
    from_subaccount: [],
    created_at_time: [],
    amount: { e8s: amount },
  });
}

export async function getBalance({
  principal,
  subaccount,
}: {
  principal: Principal;
  subaccount?: Uint8Array;
}): Promise<Tokens> {
  const actor = await getActor<ledgerService>(
    ledgerIdentity,
    ledgerIdl,
    ledgerCanister!,
  );
  const receiver = principalToAccountIdentifier(principal, subaccount);

  return await actor.account_balance({
    account: Array.from(new Uint8Array(fromHexString(receiver))),
  });
}

// export function transferArgsBuilder({
//   to,
//   amount,
//   sendOpts,
// }: {
//   to: string;
//   amount: bigint;
//   sendOpts?: SendOpts;
// }): TransferArgs {
//   const defaultFee = BigInt(10000);
//   const defaultMemo = BigInt(Math.floor(Math.random() * 10000 * 10000));
//   const subAccount =
//     sendOpts?.from_subaccount === undefined
//       ? ([] as [])
//       : (Array.from<SubAccount>([
//           fromSubAccountId(sendOpts?.from_subaccount),
//         ]) as [SubAccount]);

//   const createAtTime =
//     sendOpts?.created_at_time === undefined
//       ? ([] as [])
//       : (Array.from<TimeStamp>([
//           {
//             timestamp_nanos: BigInt(
//               sendOpts?.created_at_time?.getTime() * 1000000,
//             ),
//           },
//         ]) as [TimeStamp]);

//   return {
//     to: Array.from(accountIdentifierToBytes(to)),
//     fee: {
//       e8s: sendOpts?.fee ?? defaultFee,
//     },
//     amount: { e8s: amount },
//     memo: sendOpts?.memo ?? defaultMemo,
//     from_subaccount: subAccount,
//     created_at_time: createAtTime,
//   };
// }
