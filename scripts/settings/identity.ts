import fs from 'fs';
import path from 'path';
import { Secp256k1KeyIdentity } from '@dfinity/identity';
// import bip39 from 'bip39';
// import BIP32Factory from 'bip32';
// import { BIP32Interface } from 'bip32';
// import ecc from 'tiny-secp256k1';

const BIP32Factory = require('bip32');
const bip39 = require('bip39');
const ecc = require('tiny-secp256k1');
import { SignIdentity } from '@dfinity/agent';
import curve from 'starkbank-ecdsa';
import { isProduction } from '../env';

export function fromHexString(hexString: string): ArrayBuffer {
  return new Uint8Array(
    (hexString.match(/.{1,2}/g) ?? []).map(byte => parseInt(byte, 16)),
  ).buffer;
}

export const toHexString = (bytes: Uint8Array) =>
  bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, '0'), '');

export function getIdentityFromPem(): SignIdentity {
  const PrivateKey = curve.PrivateKey;
  const secretKey = PrivateKey.fromPem(
    fs
      .readFileSync(
        path.join(process.cwd(), '/credentials', '/production.pem'),
        { encoding: 'utf-8' },
      )
      .toString(),
  );
  return Secp256k1KeyIdentity.fromSecretKey(
    fromHexString(BigInt(secretKey.secret.toString()).toString(16)),
  );
}

export function getIdentityFromPhrase(phrase: string): SignIdentity {
  const seed = bip39.mnemonicToSeedSync(phrase);

  const ICP_PATH = "m/44'/223'/0'";
  const path = `${ICP_PATH}/0/0`;

  const bip32 = (BIP32Factory as any).default(ecc);

  const node = bip32.fromSeed(seed);

  const child = node.derivePath(path);

  return Secp256k1KeyIdentity.fromSecretKey(child.privateKey!);
  // return seed;
}

export function getIdentityFromPhraseWithSeed(phrase: string): {
  identity: SignIdentity;
  seed: Uint8Array;
} {
  const seed = bip39.mnemonicToSeedSync(phrase);

  const ICP_PATH = "m/44'/223'/0'";
  const path = `${ICP_PATH}/0/0`;

  const bip32 = (BIP32Factory as any).default(ecc);

  const node = bip32.fromSeed(seed);

  const child = node.derivePath(path);

  return {
    identity: Secp256k1KeyIdentity.fromSecretKey(child.privateKey!),
    seed: new Uint8Array(seed),
  };
  // return seed;
}

const seedPhrase = fs
  .readFileSync(path.join(process.cwd(), '/credentials', '/internal.txt'), {
    encoding: 'utf8',
  })
  .toString();

const identity = !isProduction
  ? getIdentityFromPhrase(seedPhrase)
  : getIdentityFromPem();

export { identity };
