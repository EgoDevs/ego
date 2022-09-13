import fs from 'fs';
import { randomBytes } from 'crypto';
import Mock from 'mockjs';
import { toHexString } from '@dfinity/candid';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import {
  getIdentityFromPhrase,
  getIdentityFromPhraseWithSeed,
} from '../settings/identity';
import { SignIdentity } from '@dfinity/agent';
const { Random } = Mock;
export interface MockIdentity {
  name: string;
  principal: string;
  derEncodedPublicKey: string;
  identity: SignIdentity;
  userGroup: number;
  seed: String;
}

function identityFactory(index: number, group: number) {
  const ids: Array<MockIdentity> = [];
  for (var i = 0; i < index; i += 1) {
    const seed = randomBytes(32);
    const seedArr = new Uint8Array(seed);
    const id = Ed25519KeyIdentity.generate(seedArr);
    ids.push({
      name: Random.name(),
      derEncodedPublicKey: toHexString(id.getPublicKey().toDer()),
      principal: id.getPrincipal().toText(),
      identity: id,
      userGroup: group,
      seed: seed.toString('hex'),
    });
  }
  return ids;
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

function generate() {
  fs.writeFileSync(
    `${process.cwd()}/clients/fixtures/identities.json`,
    JSON.stringify([
      ...identityFactory(2, 0),
      ...identityFactory(2, 1),
      ...identityFactory(2, 2),
      ...identityFactory(2, 3),
    ]),
  );
}

generate();

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

// generate2();
