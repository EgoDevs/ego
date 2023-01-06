import fs from 'fs';
import crypto, { BinaryLike } from 'crypto';

import { getCanisterId, hasOwnProperty } from '@/settings/utils';
import { admins, developers, operators } from '@/fixtures/identities';
import { getActor } from '@/settings/agent';
import { ActorSubclass } from '@dfinity/agent';
import {_SERVICE, EgoError, Result_6} from '@/idls/ego_dev';
import { Principal } from '@dfinity/principal';
import { Ed25519KeyIdentity } from '@dfinity/identity';
import { idlFactory } from '@/idls/ego_dev.idl';

function random_str(length) {
    let result = '';
    const characters =
        'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    for (let i = 0; i < length; i++) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
    }
    return result;
}

function random_version() {
    function randInt(limit) {
        return Math.floor(Math.random() * limit);
    }

    return {
        major: randInt(100000),
        minor: randInt(100000),
        patch: randInt(100000),
    };
}

const is_same_version = (v1, v2) => {
    return v1.major == v2.major && v1.minor == v2.minor && v1.patch == v2.patch;
};

const app_operators: Array<ActorSubclass<_SERVICE>> = [];
const app_developers: Array<ActorSubclass<_SERVICE>> = [];
const app_managers: Array<ActorSubclass<_SERVICE>> = [];
const exist_app_id = 'app_1';
const app_id = 'app_2';

beforeAll(async () => {

    for (let i = 0; i < 2; i += 1) {
        let storeActor = getActor<_SERVICE>(
            Ed25519KeyIdentity.fromJSON(JSON.stringify(operators[i]?.identity)),
            idlFactory,
            getCanisterId('ego_dev')!,
        );

        let actor = await storeActor;

        app_operators.push(actor);
    }

    for (let i = 0; i < 2; i += 1) {
        let storeActor = getActor<_SERVICE>(
            Ed25519KeyIdentity.fromJSON(JSON.stringify(developers[i]?.identity)),
            idlFactory,
            getCanisterId('ego_dev')!,
        );

        let actor = await storeActor;

        app_developers.push(actor);
    }

    let storeActor = getActor<_SERVICE>(
        Ed25519KeyIdentity.fromJSON(JSON.stringify(admins[0]?.identity)),
        idlFactory,
        getCanisterId('ego_dev')!,
    );

    let actor = await storeActor;
    app_managers.push(actor);
});

// describe('manager', () => {
//     test('list user', async () => {
//         const resp = await store_managers[0].list_user({ name: 'user' });
//         console.log((resp as { Ok: ListUserResponse }).Ok.users);
//         expect((resp as { Ok: ListUserResponse }).Ok.users[0].name).toBe('user');
//     });
//
//     test('set_role ', async () => {
//         const principal = Ed25519KeyIdentity.fromJSON(JSON.stringify(developers[1]?.identity)).getPrincipal();
//         const resp = await store_managers[0].set_role({
//             user_id: principal,
//             is_app_auditer: false,
//             is_app_developer: true,
//             is_manager: false
//         });
//         console.log('set_role', (resp as { Ok: SetRoleResponse }));
//         expect((resp as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
//     });
// });

//
// describe('app auditer', () => {
//     const exist_app_id = 'app_1';
//
//     const exist_version: Version = { major: 1, minor: 0, patch: 0 };
//
//     let new_version;
//
//     beforeEach(async () => {
//         new_version = random_version();
//         const resp = await app_developers[0].new_app_version({
//             app_id: exist_app_id,
//             version: new_version,
//         });
//         console.log(`new app version `, resp)
//     });
//
//     test('approve version', async () => {
//         // 1.submit new version
//         const resp1 = await app_developers[0].submit_app_version({
//             app_id: exist_app_id,
//             version: new_version,
//         });
//         console.log(`submit_app_version `, resp1)
//
//         expect((resp1 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
//
//         // 2.check audit version
//         const resp2 = await app_developers[0].get_app({ app_id: exist_app_id });
//         expect((resp2 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
//             1,
//         );
//
//         // 3.approve the version
//         const resp3 = await app_operators[0].approve_app_version({
//             app_id: exist_app_id,
//             version: new_version,
//         });
//
//         expect((resp3 as { Ok: ApproveAppVersionResponse }).Ok.ret).toBe(true);
//
//         // 4.owner check audit version and release version
//         const resp4 = await app_operators[0].get_app({ app_id: exist_app_id });
//         expect((resp4 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
//             0,
//         );
//
//         // 5.confirm app version status to be approved
//         const app_versions = (resp4 as { Ok: GetAppResponse }).Ok.app.versions;
//         app_versions.forEach(app_version => {
//             if (app_version.version == new_version) {
//                 expect(hasOwnProperty(app_version.status, 'APPROVED')).toBeTruthy();
//             }
//         });
//     });
//
//     test('reject version', async () => {
//         // 1.submit new version
//         const resp1 = await app_developers[0].submit_app_version({
//             app_id: exist_app_id,
//             version: new_version,
//         });
//
//         expect((resp1 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
//
//         // 2.check audit version
//         const resp2 = await app_developers[0].get_app({ app_id: exist_app_id });
//         expect((resp2 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
//             1,
//         );
//
//         // 3.approve the version
//         const resp3 = await app_operators[0].reject_app_version({
//             app_id: exist_app_id,
//             version: new_version,
//         });
//
//         expect((resp3 as { Ok: InitStoreResponse }).Ok.ret).toBe(true);
//
//         // 4.owner check audit version and release version
//         const resp4 = await app_operators[0].get_app({ app_id: exist_app_id });
//         expect((resp4 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
//             0,
//         );
//
//         // 5.confirm app version status to be approved
//         const app_versions = (resp4 as { Ok: GetAppResponse }).Ok.app.versions;
//         app_versions.forEach(app_version => {
//             if (app_version.version == new_version) {
//                 expect(hasOwnProperty(app_version.status, 'REJECTED')).toBeTruthy();
//             }
//         });
//     });
// });

const app_1_wasm = fs.readFileSync(
    `${[process.cwd()]}` + '/clients/fixtures/app_1.wasm',
);
const fileMd5 = crypto
    .createHash('md5')
    .update(app_1_wasm as BinaryLike)
    .digest('hex');

describe('app developer', () => {
    let app_version = random_version();

    test('register developer', async () => {
        let resp1 = await app_developers[0].developer_main_register('developer 1');
        let developer = resp1.Ok;
        console.log("resp1", resp1);
        expect(developer.developer_id.toString()).toBe(developers[0].principal)
        expect(developer.name).toBe('developer 1')

        let resp2 = await app_developers[1].developer_main_register('developer 2');
        developer = resp2.Ok;
        console.log("resp2", resp2);
        expect(developer.developer_id.toString()).toBe(developers[1].principal)
        expect(developer.name).toBe('developer 2')

        let resp3 = await app_developers[0].developer_main_get();
        developer = resp3.Ok;
        expect(developer.developer_id.toString()).toBe(developers[0].principal)
        expect(developer.name).toBe('developer 1')
    });

    // test('register app with not exists app_id', async () => {
    //     const registerResponse = await app_developers[0].register_app({
    //         app_id: app_id,
    //         name: 'app name',
    //         category: { Vault: null },
    //         price: 1,
    //     });
    //     expect((registerResponse as { Ok: GetAppResponse }).Ok.app.app_id).toBe(
    //         app_id,
    //     );
    // });
    //
    // test('register app with exists app_id', async () => {
    //     const registerResponse = await app_developers[1].developer_app_new({
    //         app_id: app_id,
    //         name: 'app name',
    //         category: { Vault: null },
    //         price: 1,
    //     });
    //
    //     expect((registerResponse as { Err: EgoError }).Err.code).toBe(1001);
    // });
    //
    // test('get created app', async () => {
    //     const response = await app_developers[0].created_apps();
    //     console.log(`created_apps`, response)
    //     let apps = (response as { Ok: CreatedAppResponse }).Ok.apps
    //     expect(apps).toBeTruthy();
    //     let app_len = apps.length;
    //     for (let i = 0; i < app_len; i++) {
    //         let app = apps[i]
    //         if (app.app_id == 'exist_app_id')
    //             expect(app.app_id).toBe(exist_app_id);
    //     }
    // });
    //
    // test('create not exists version', async () => {
    //     const resp = await app_developers[0].new_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     expect((resp as { Ok: InitStoreResponse }).Ok.ret).toBeTruthy();
    // });
    //
    // test('create exists version', async () => {
    //     const resp = await app_developers[0].new_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     expect((resp as { Err: EgoError }).Err.code).toBe(1003);
    // });
    //
    // test('submit version', async () => {
    //     // 1.submit new version
    //     const resp1 = await app_developers[0].submit_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //     expect((resp1 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
    //
    //     // 2.check audit version is the newly submited version
    //     const resp2 = await app_developers[0].get_app({ app_id: app_id });
    //     expect((resp2 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         1,
    //     );
    //
    //     const version = (resp2 as { Ok: GetAppResponse }).Ok.app.audit_version[0];
    //     expect(is_same_version(version, app_version)).toBeTruthy();
    //
    //     // 3.check audit version can not be see by others
    //     const resp3 = await end_users[1].get_app({ app_id: exist_app_id });
    //     expect((resp3 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         0,
    //     );
    //
    //     // 4.confirm app version status to be submited
    //     const app_versions = (resp2 as { Ok: GetAppResponse }).Ok.app.versions;
    //
    //     app_versions.forEach(app_version_inst => {
    //         if (is_same_version(app_version_inst.version, app_version)) {
    //             expect(
    //                 hasOwnProperty(app_version_inst.status, 'SUBMITTED'),
    //             ).toBeTruthy();
    //         }
    //     });
    // });
    //
    // test('revoke version', async () => {
    //     // 1.submit new version
    //     const resp1 = await app_developers[0].submit_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     expect((resp1 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
    //
    //     // 2.check audit version
    //     const resp2 = await app_developers[0].get_app({ app_id: app_id });
    //     expect((resp2 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         1,
    //     );
    //
    //     // 3.revoke the version
    //     const resp3 = await app_developers[0].revoke_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     expect((resp3 as { Ok: InitStoreResponse }).Ok.ret).toBe(true);
    //
    //     // 4.confirm audit version not exists
    //     const resp4 = await app_developers[0].get_app({ app_id: app_id });
    //     expect((resp4 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         0,
    //     );
    //
    //     // 5.confirm app version status to be revoked
    //     const app_versions = (resp4 as { Ok: GetAppResponse }).Ok.app.versions;
    //     app_versions.forEach(app_version_inst => {
    //         if (is_same_version(app_version_inst.version, app_version)) {
    //             expect(hasOwnProperty(app_version_inst.status, 'REVOKED')).toBeTruthy();
    //         }
    //     });
    // });
    //
    // test('release version', async () => {
    //     // 1.submit new version
    //     const resp1 = await app_developers[0].submit_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //     console.log(`app_version`, app_version);
    //     console.log(`release version: submit_app_version`, (resp1 as { Ok: SetRoleResponse }).Ok);
    //     expect((resp1 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
    //
    //     // 2.check audit version
    //     const resp2 = await app_developers[0].get_app({ app_id: app_id });
    //     expect((resp2 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         1,
    //     );
    //
    //     // 3.approve the version
    //     const resp3 = await app_operators[0].approve_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     // 4. upload file
    //     let backend_wasm;
    //     let asset_wasm;
    //     let getAppResponse = await app_developers[0].get_app({
    //         app_id: app_id,
    //     });
    //     console.log(`get app response: `, getAppResponse);
    //     (getAppResponse as { Ok: GetAppResponse }).Ok.app.versions.forEach(ver => {
    //         if (is_same_version(ver.version, app_version)) {
    //             ver.wasms.forEach(wasm => {
    //                 if (wasm.canister_type.hasOwnProperty('BACKEND')) {
    //                     backend_wasm = wasm;
    //                 }
    //                 if (wasm.canister_type.hasOwnProperty('ASSET')) {
    //                     asset_wasm = wasm;
    //                 }
    //             });
    //         }
    //     });
    //     const uploadFileResponse = await developer[0].upload_file({
    //         fid: backend_wasm.file_id!,
    //         appid: app_id,
    //         data: Array.from(app_1_wasm),
    //         hash: fileMd5,
    //         version: `${app_version.major}.${app_version.minor}.${app_version.patch}`,
    //     });
    //
    //     console.log('fid is: ', backend_wasm.file_id!);
    //     console.log(`file upload success by developer[0]`, uploadFileResponse);
    //     expect((uploadFileResponse as { Ok: UploadFileResponse }).Ok.appid).toBe(
    //         app_id,
    //     );
    //
    //     // 5.release the version
    //     const resp4 = await app_developers[0].release_app_version({
    //         app_id: app_id,
    //         version: app_version,
    //     });
    //
    //     console.log(resp4);
    //     expect((resp4 as { Ok: ApproveAppVersionResponse }).Ok.ret).toBe(true);
    //
    //     // 6.owner check audit version and release version
    //     const resp5 = await app_developers[0].get_app({ app_id: app_id });
    //     expect((resp5 as { Ok: GetAppResponse }).Ok.app.audit_version.length).toBe(
    //         0,
    //     );
    //     expect(
    //         (resp5 as { Ok: GetAppResponse }).Ok.app.release_version.length,
    //     ).toBe(1);
    //
    //     // 7.app version status to be approved
    //     const app_versions = (resp5 as { Ok: GetAppResponse }).Ok.app.versions;
    //     app_versions.forEach(app_version_inst => {
    //         if (is_same_version(app_version_inst.version, app_version)) {
    //             expect(
    //                 hasOwnProperty(app_version_inst.status, 'RELEASED'),
    //             ).toBeTruthy();
    //         }
    //     });
    //
    //     // 8.other people can see the released version
    //     const resp6 = await end_users[0].get_app({ app_id: app_id });
    //     expect(
    //         (resp6 as { Ok: GetAppResponse }).Ok.app.release_version.length,
    //     ).toBe(1);
    // });
    //
    // test.only('test update ego-assets', async () => {
    //     let canister_id = Principal.fromText(getCanisterId('assets_storage')!)
    //
    //     // 1.submit new version
    //     const resp1 = await app_developers[0].new_app_version({
    //         app_id: exist_app_id,
    //         version: app_version,
    //     });
    //
    //     expect((resp1 as { Ok: InitStoreResponse }).Ok.ret).toBeTruthy();
    //
    //     // 2.set frontend address
    //     const resp2 = await app_developers[0].set_frontend_address({
    //         app_id: exist_app_id,
    //         version: app_version,
    //         canister_id: canister_id
    //     });
    //     console.log(resp2)
    //
    //     // 3.submit version
    //     const resp3 = await app_developers[0].submit_app_version({
    //         app_id: exist_app_id,
    //         version: app_version,
    //     });
    //     console.log(resp3)
    //     expect((resp3 as { Ok: SetRoleResponse }).Ok.ret).toBe(true);
    //
    // });
});

// describe('ledger callback ', () => {
//
//     test('notify payment success order is exists', async () => {
//         const resp1 = await end_users[0].create_app_order({
//             app_id: exist_app_id,
//         });
//         let order = (resp1 as { Ok: CreateOrderResponse }).Ok.order;
//         console.log(order);
//         expect(order.app_id[0]).toEqual(exist_app_id);
//         expect(order.user_id).toEqual(Principal.fromText(endUsers[0].principal));
//
//         const resp = await end_users[0].notify_payment({ memo: order.memo });
//         console.log(`notify_payment`, resp);
//         expect(resp.Ok.ret).toBe(true);
//     });
//
//     test('notify payment fail order not exists', async () => {
//         const resp = await end_users[0].notify_payment({ memo: BigInt(1008910) });
//         console.log(`notify_payment`, resp);
//         expect(resp.Err.code).toBe(1009);
//     });
// });