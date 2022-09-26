import { Button, message } from 'antd';
import React, { useEffect, useState } from 'react';
import Footer from '@/components/Footer';
import { IC } from '@astrox/connection';
import styles from './index.module.less';
import { IcObject } from '@/utils';
import { StoreConnection } from '@/services/connection/store';
import { BucketConnection } from '@/services/connection/bucket';
import { idlFactory as storeIdl } from "@/../../idls/ego_store.idl";
import { idlFactory as bucketIdl } from "@/../../idls/ego_bucket.idl";
import { MeResponse, Result_8, _SERVICE as storeService } from '@/../../idls/ego_store';
import { _SERVICE as bucketService } from '@/../../idls/ego_bucket';
import { PermissionsType } from '@astrox/connection/lib/cjs/types';
import { useHistory } from 'react-router-dom';
import { useDispatch } from 'react-redux';
import { RootDispatch } from '@/store';
import { Principal } from '@dfinity/principal';
import { client } from '@/main';
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


  useEffect(() => {
    console.log("useEffect login")
  }, [])

  const handleSubmit = async () => {
    try {
      const result = await  client.connect()
      if(result) {
       const storeConnection = await StoreConnection.create(client.identity);
       const bucketConnection = await BucketConnection.create(client.identity);
       // const walletConnection = await WalletConnection.create(client.identity);
       dispatch.global.save({
         initialState: {
           storeConnection,
           bucketConnection,
           // walletConnection,
           currentUser: client,
           isAuthenticated: true,
         }
       })
       dispatch.global.getUser({ storeConnection })
       history.push('/home');
       console.log('register user', result)
      }
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
