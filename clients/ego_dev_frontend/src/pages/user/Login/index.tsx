import { Button, message } from 'antd';
import React, { useState } from 'react';
import Footer from '@/components/Footer';
import { IC } from '@astrox/connection';
import styles from './index.module.less';
import { IcObject } from '@/utils';
import { StoreConnection } from '@/services/connection/store';
import { BucketConnection } from '@/services/connection/bucket';
import { idlFactory as storeIdl } from "@/canisters/ego_store.idl";
import { idlFactory as bucketIdl } from "@/canisters/ego_bucket.idl";
import { MeResponse, Result_8, _SERVICE as storeService } from '@/canisters/ego_store';
import { _SERVICE as bucketService } from '@/canisters/ego_bucket';
import { PermissionsType } from '@astrox/connection/lib/cjs/types';
import { useHistory } from 'react-router-dom';
import { useDispatch } from 'react-redux';
import { RootDispatch } from '@/store';
import { Principal } from '@dfinity/principal';
// const LoginMessage: React.FC<{
//   content: string;
// }> = ({ content }) => (
//   <Alert
//     style={{
//       marginBottom: 24,
//     }}
//     message={content}
//     type="error"
//     showIcon
//   />
// );



const Login: React.FC = () => {
  console.log(process.env.EGO_STORE_CANISTERID)
  const history = useHistory();
  const dispatch = useDispatch<RootDispatch>()
  const handleSubmit = async () => {
    try {
      // 登录
      IcObject.connect({
        // dev: true,
        useFrame: !(window.innerWidth < 768),
        signerProviderUrl: process.env.isProduction!
          ? 'https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/signer'
          : 'http://localhost:8080/signer',
        walletProviderUrl: process.env.isProduction!
          ? 'https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/transaction'
          : 'http://localhost:8080/transaction', // 'http://localhost:8080/transaction', // "https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/transaction",
        identityProvider: process.env.isProduction!
          ? 'https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/login#authorize'
          : 'http://localhost:8080/login#authorize', // 'http://localhost:8080/login#authorize', // 'https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/login#authorize',
        permissions: [PermissionsType.identity, PermissionsType.wallet],
        delegationTargets: ['qsgjb-riaaa-aaaaa-aaaga-cai', 'qhbym-qaaaa-aaaaa-aaafq-cai'],
        onAuthenticated: async (icInstance: IC) => {
          console.log('icInstance.identity===', icInstance.identity)
          // const bucketActor = icInstance.createActor<bucketService>(
          //   bucketIdl,
          //   process.env.EGO_BUCKET_CANISTERID!,
          // );
          // console.log(process.env.EGO_STORE_CANISTERID)
          // const storeActor = icInstance.createActor<storeService>(
          //   storeIdl,
          //   process.env.EGO_STORE_CANISTERID!,
          // );
          // console.log('storeActor', storeActor)
          // console.log('store',  process.env.EGO_STORE_CANISTERID!)
          const storeConnection = await StoreConnection.create(window.ic.astrox.identity);
          const bucketConnection = await BucketConnection.create(window.ic.astrox.identity);
          dispatch.global.save({
            initialState: {
              storeConnection,
              bucketConnection,
              currentUser: window.ic.astrox,
              isAuthenticated: true,
            }
          })
          dispatch.global.getUser({ storeConnection })
          history.push('/home');
        },
      })
    } catch (error) {
      message.error('登录失败，请重试');
    }
  };

  return (
    <div className={styles.container}>
      <div className={styles.lang} data-lang>
        {/* {SelectLang !== undefined && <SelectLang />} */}
      </div>
      <div className={styles.content}>
        <h1>AstroX Developer Center</h1>
        <p>Get all developer resources of AstroX right away.</p>
        <Button type="primary" onClick={handleSubmit}>Log in</Button>
      </div>
      <Footer />
    </div>
  );
};

export default Login;
