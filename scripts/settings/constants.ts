export const SUB_ACCOUNT_BYTE_LENGTH = 32;
export const CREATE_CANISTER_MEMO = BigInt(0x41455243); // CREA,
export const TOP_UP_CANISTER_MEMO = BigInt(0x50555054); // TPUP

export const TRANSACTION_FEE = BigInt(10_000);

export const NET_ID = {
  blockchain: 'Internet Computer',
  network: '00000000000000020101',
};

export const ROSETTA_URL = 'https://rosetta-api.internetcomputer.org';
export const IC_EXPLORER = 'https://dashboard.internetcomputer.org';
export const IC_ROCKS = 'https://ic.rocks';

export const MAX_TRANSACTION_DECISION_MILSECONDS = 120000;

export const PRINCIPAL_REGEX = /(\w{5}-){10}\w{3}/;
export const ALPHANUM_REGEX = /^[a-zA-Z0-9]+$/;
export const CANISTER_REGEX = /(\w{5}-){4}\w{3}/;
export const CANISTER_MAX_LENGTH = 27;

export const ADDRESS_TYPES = {
  PRINCIPAL: 'principal',
  ACCOUNT: 'accountId',
  CANISTER: 'canister',
  ERC20: 'erc20',
  UNKNOWN: 'unknown',
};
// import config from "../config";

// // @ts-ignore
// export const HOST = "HOST" in config ? config["HOST"] : undefined;
