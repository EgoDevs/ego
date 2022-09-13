import { Principal } from '@dfinity/principal';
import { sha224 } from 'js-sha256';
import { Buffer } from 'buffer';
import crc from 'crc';
import {
  ALPHANUM_REGEX,
  CANISTER_MAX_LENGTH,
  SUB_ACCOUNT_BYTE_LENGTH,
} from './constants';
import { AccountIdentifier, Balance, SubAccount } from './common/types';

export const uint8ArrayToBigInt = (array: Uint8Array): bigint => {
  const view = new DataView(array.buffer, array.byteOffset, array.byteLength);
  if (typeof view.getBigUint64 === 'function') {
    return view.getBigUint64(0);
  } else {
    const high = BigInt(view.getUint32(0));
    const low = BigInt(view.getUint32(4));

    return (high << BigInt(32)) + low;
  }
};

const TWO_TO_THE_32 = BigInt(1) << BigInt(32);
export const bigIntToUint8Array = (value: bigint): Uint8Array => {
  const array = new Uint8Array(8);
  const view = new DataView(array.buffer, array.byteOffset, array.byteLength);
  if (typeof view.setBigUint64 === 'function') {
    view.setBigUint64(0, value);
  } else {
    view.setUint32(0, Number(value >> BigInt(32)));
    view.setUint32(4, Number(value % TWO_TO_THE_32));
  }

  return array;
};

export const arrayBufferToArrayOfNumber = (
  buffer: ArrayBuffer,
): Array<number> => {
  const typedArray = new Uint8Array(buffer);
  return Array.from(typedArray);
};

export const arrayOfNumberToUint8Array = (
  numbers: Array<number>,
): Uint8Array => {
  return new Uint8Array(numbers);
};

export const arrayOfNumberToArrayBuffer = (
  numbers: Array<number>,
): ArrayBuffer => {
  return arrayOfNumberToUint8Array(numbers).buffer;
};

export const arrayBufferToNumber = (buffer: ArrayBuffer): number => {
  const view = new DataView(buffer);
  return view.getUint32(view.byteLength - 4);
};

export const numberToArrayBuffer = (
  value: number,
  byteLength: number,
): ArrayBuffer => {
  const buffer = new ArrayBuffer(byteLength);
  new DataView(buffer).setUint32(byteLength - 4, value);
  return buffer;
};

export const asciiStringToByteArray = (text: string): Array<number> => {
  return Array.from(text).map(c => c.charCodeAt(0));
};

export const toSubAccountId = (subAccount: Array<number>): number => {
  const bytes = arrayOfNumberToArrayBuffer(subAccount);
  return arrayBufferToNumber(bytes);
};

export const fromSubAccountId = (subAccountId: number): Array<number> => {
  const buffer = numberToArrayBuffer(subAccountId, SUB_ACCOUNT_BYTE_LENGTH);
  return arrayBufferToArrayOfNumber(buffer);
};

export const accountIdentifierToBytes = (
  accountIdentifier: AccountIdentifier,
): Uint8Array => {
  return Uint8Array.from(Buffer.from(accountIdentifier, 'hex')).subarray(4);
};

export const accountIdentifierFromBytes = (
  accountIdentifier: Uint8Array,
): AccountIdentifier => {
  return Buffer.from(accountIdentifier).toString('hex');
};

export const principalToAccountIdentifier = (
  principal: Principal,
  subAccount?: Uint8Array,
): string => {
  // Hash (sha224) the principal, the subAccount and some padding
  const padding = asciiStringToByteArray('\x0Aaccount-id');

  const shaObj = sha224.create();
  shaObj.update([
    ...padding,
    ...principal.toUint8Array(),
    ...(subAccount ?? Array(32).fill(0)),
  ]);
  const hash = new Uint8Array(shaObj.array());

  // Prepend the checksum of the hash and convert to a hex string
  const checksum = calculateCrc32(hash);
  const bytes = new Uint8Array([...checksum, ...hash]);
  return toHexString(bytes);
};

export const principalToSubAccount = (principal: Principal): SubAccount => {
  const bytes = principal.toUint8Array();
  const subAccount = new Uint8Array(32);
  subAccount[0] = bytes.length;
  subAccount.set(bytes, 1);
  return subAccount;
};

export const stringToAccountIdentifier = (
  str: string,
): AccountIdentifier | undefined => {
  try {
    if (str.length === 64) {
      return str;
    }
    if (str.length === 63) {
      return principalToAccountIdentifier(Principal.fromText(str));
    }
    return undefined;
  } catch (error) {
    return undefined;
  }
};

const toHexString = (bytes: Uint8Array) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

// 4 bytes
export const calculateCrc32 = (bytes: Uint8Array): Uint8Array => {
  const checksumArrayBuf = new ArrayBuffer(4);
  const view = new DataView(checksumArrayBuf);
  view.setUint32(0, crc.crc32(Buffer.from(bytes)), false);
  return Buffer.from(checksumArrayBuf);
};

export const E8S_PER_ICP = 100_000_000;

export enum TokenSymbol {
  ICP = 'ICP',
}

export const getDecimalFromSymbol = (sym: string) => {
  switch (sym) {
    case TokenSymbol.ICP:
      return 8;
    default:
      return 8;
  }
};

export interface TokenMapItem {
  [key: string]: {
    amount: number;
    symbol: string;
    balanceString: BalanceString;
  };
}

export const formatAssetBySymbol = (
  _amount: bigint,
  symbol: string,
):
  | { amount: number; symbol: string; balanceString: BalanceString }
  | undefined => {
  const balanceString = balanceToString(_amount, getDecimalFromSymbol(symbol));
  const amount = Number(balanceString.total);
  const tokenMap: TokenMapItem[] = [
    {
      ICP: {
        amount: amount,
        balanceString,
        symbol: 'ICP',
      },
    },
  ];

  const found = tokenMap.find(v => v[symbol] !== undefined);
  return found?.[symbol];
};

export const parseBalance = (balance: Balance): string => {
  return (parseInt(balance.value, 10) / 10 ** balance.decimals).toString();
};

export const balanceFromString = (balance: string, decimal = 8): bigint => {
  const list = balance.split('.');
  const aboveZero = list[0];
  const aboveZeroBigInt = BigInt(aboveZero) * BigInt(1 * 10 ** decimal);
  let belowZeroBigInt = BigInt(0);
  const belowZero = list[1];
  if (belowZero !== undefined) {
    belowZeroBigInt = BigInt(
      belowZero.substring(0, decimal).padEnd(decimal, '0'),
    );
  }
  return aboveZeroBigInt + belowZeroBigInt;
};

export interface BalanceString {
  total: string;
  aboveZero: string;
  belowZero: string;
  formatAboveZero: string;
  formatTotal: string;
  formatTotalTo8: string;
  formatTotalTo4: string;
}

export const balanceToString = (
  balance: bigint,
  decimal = 8,
): BalanceString => {
  const balanceString = balance.toString(10);
  const balanceStringLength = balanceString.length;
  let aboveZero = '0';
  let belowZero = '0'.padEnd(decimal, '0');
  if (balanceStringLength > decimal) {
    belowZero = balanceString.substring(
      balanceStringLength - decimal,
      balanceStringLength,
    );
    aboveZero = balanceString.substring(0, balanceStringLength - decimal);
  } else {
    belowZero = balanceString.padStart(decimal, '0');
  }
  const formatAboveZero = String(aboveZero).replace(
    /\B(?=(\d{3})+(?!\d))/g,
    ',',
  );

  return {
    total: aboveZero + '.' + belowZero,
    aboveZero,
    belowZero,
    formatAboveZero,
    formatTotal:
      formatAboveZero +
      '.' +
      (parseFloat('0.' + belowZero)
        .toString()
        .split('.')[1] ?? '0'),
    formatTotalTo8: formatAboveZero + '.' + belowZero,
    formatTotalTo4:
      formatAboveZero +
      '.' +
      (parseFloat('0.' + belowZero)
        .toFixed(4)
        .toString()
        .split('.')[1] ?? '0'),
  };
};

export const validateAccountId = (text: string): boolean =>
  text.length === 64 && ALPHANUM_REGEX.test(text);

export const validatePrincipalId = (text: string) => {
  try {
    return text === Principal.fromText(text).toString();
  } catch (e) {
    return false;
  }
};

export const validateCanisterId = text => {
  try {
    return text.length <= CANISTER_MAX_LENGTH && validatePrincipalId(text);
  } catch (e) {
    return false;
  }
};

export enum AddressType {
  PRINCIPAL = 'principal',
  ACCOUNT = 'accountId',
  CANISTER = 'canister',
  ERC20 = 'erc20',
  INVALID = 'invalid',
}

export const getAddressType = (text: string) => {
  try {
    if (validateAccountId(text)) {
      return AddressType.ACCOUNT;
    } else if (validatePrincipalId(text)) {
      return AddressType.PRINCIPAL;
    } else if (validateCanisterId(text)) {
      return AddressType.CANISTER;
    }
    return AddressType.INVALID;
  } catch (error) {
    throw error;
  }
};
