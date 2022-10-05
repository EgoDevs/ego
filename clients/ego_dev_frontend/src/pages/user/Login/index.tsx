import { Button, message } from 'antd';
import React, { useEffect, useState } from 'react';
import Footer from '@/components/Footer';
import styles from './index.module.less';
import { DevConnection } from '@/services/connection/dev';
import { useHistory } from 'react-router-dom';
import { useDispatch } from 'react-redux';
import { RootDispatch } from '@/store';
import { client } from '@/main';


const Login: React.FC = () => {
  console.log(process.env.EGO_DEV_CANISTERID)
  const history = useHistory();
  const dispatch = useDispatch<RootDispatch>()

  useEffect(() => {
    console.log("useEffect login")
  }, [])

  const handleSubmit = async () => {
    try {
      const result = await  client.connect()
      if(result) {
       const storeConnection = await DevConnection.create(client.identity);

       dispatch.global.save({
         initialState: {
           storeConnection,
           currentUser: client,
           isAuthenticated: true,
         }
       })
       const result = await dispatch.global.getUser({ storeConnection })
       console.log('register user', result)
       if(result.developer) {
          history.push('/home');
       } else {
          history.push('/user/register')
       }
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
        <h1>Ego developer center</h1>
        <p>Get all developer resources of AstroX right away.</p>
        <Button type="primary" onClick={handleSubmit}>Log in</Button>
      </div>
      <Footer />
    </div>
  );
};

export default Login;
