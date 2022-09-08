import fs from 'fs';
import { randomBytes } from 'crypto';
import Mock from 'mockjs';
import { toHexString } from '@dfinity/candid';
import { Ed25519KeyIdentity, Secp256k1KeyIdentity } from '@dfinity/identity';

const BIP32Factory = require('bip32');
const bip39 = require('bip39');
const ecc = require('tiny-secp256k1');

import { SignIdentity } from '@dfinity/agent';
const { Random } = Mock;

export interface MockIdentity {
  name: string;
  principal: string;
  derEncodedPublicKey: string;
  identity: Secp256k1KeyIdentity;
  userGroup: number;
  seed: String;
}

export function getIdentityFromPhraseWithSeed(phrase: string): {
  identity: Secp256k1KeyIdentity;
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

function identityFactoryWihPhrase(
  phrase: string,
  index: number,
  group: number,
) {
  const ids: Array<MockIdentity> = [];
  for (var i = 0; i < index; i += 1) {
    const { identity, seed } = getIdentityFromPhraseWithSeed(phrase);
    ids.push({
      name: Random.name(),
      derEncodedPublicKey: toHexString(identity.getPublicKey().toDer()),
      principal: identity.getPrincipal().toText(),
      identity: identity,
      userGroup: group,
      seed: toHexString(seed),
    });
  }
  return ids;
}

function generate2() {
  fs.writeFileSync(
    `${process.cwd()}/clients/fixtures/identities2.json`,
    JSON.stringify([
      ...identityFactoryWihPhrase(
        'drama catch young miss please high blanket animal armor outdoor capital page',
        1,
        0,
      ),
      // ...identityFactory(2, 1),
      // ...identityFactory(2, 2),
      // ...identityFactory(2, 3),
    ]),
  );
}

generate2();
