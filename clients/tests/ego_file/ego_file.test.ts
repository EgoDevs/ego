import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { getCanisterId, hasOwnProperty, getActor, identity as defaultIdentity } from '@ego-js/utils';

import { idlFactory } from '@/idls/ego_file.idl';

import { _SERVICE } from '@/idls/ego_file';

const identity = defaultIdentity();

const file_id = 'app_1';

export const file_actor = getActor<_SERVICE>(identity, idlFactory, getCanisterId('ego_file')!);

beforeAll(async () => {});

const app_1_wasm = fs.readFileSync(`${[process.cwd()]}` + '/clients/fixtures/app_1.wasm');
const fileMd5 = crypto
  .createHash('md5')
  .update(app_1_wasm as BinaryLike)
  .digest('hex');

describe('ego_file', () => {
  test('file_main_write and file_main_read', async () => {
    let file_canister = await file_actor;

    let response1 = await file_canister.file_main_write(file_id, fileMd5, Array.from(app_1_wasm));
    console.log(`file_main_write response: `, response1);

    let response2 = await file_canister.file_main_read(file_id);

    let data = (response2 as any).Ok.data;

    let ret_hash = crypto
      .createHash('md5')
      .update(data as BinaryLike)
      .digest('hex');

    console.log(`file_main_read fileMd5:[%s] data hash:[%s]`, fileMd5, ret_hash);
    expect(fileMd5 == ret_hash);

    let response3 = await file_canister.state_persist();
    console.log('state_persist: ', response3);

    let response4 = await file_canister.state_restore();
    console.log('state_restore: ', response4);

    let response5 = await file_canister.file_main_read(file_id);

    let data5 = (response5 as any).Ok.data;

    let ret_hash5 = crypto
      .createHash('md5')
      .update(data5 as BinaryLike)
      .digest('hex');

    console.log(`file_main_read after state persist fileMd5:[%s] data hash:[%s]`, fileMd5, ret_hash);
    expect(fileMd5 == ret_hash5);
  });
});
