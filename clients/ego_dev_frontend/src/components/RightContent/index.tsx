import { Space } from 'antd';
// import { QuestionCircleOutlined } from '@ant-design/icons';
import React from 'react';
// import { useModel, SelectLang } from 'umi';
import Avatar from './AvatarDropdown';
import styles from './index.module.less';

export type SiderTheme = 'light' | 'dark';

const GlobalHeaderRight: React.FC = () => {
  // const { initialState } = useModel('@@initialState');
  // const initialState =
  // console.log('rightContent')
  // if (initialState === undefined) {
  //   return null;
  // }
 
  return (
    <Space className={styles.right}>
      {/* <span
        className={styles.action}
        onClick={() => {
          window.open('https://pro.ant.design/docs/getting-started');
        }}
      >
        <QuestionCircleOutlined />
      </span> */}
      <Avatar />
      {/* <SelectLang className={styles.action} /> */}
    </Space>
  );
};
export default GlobalHeaderRight;
