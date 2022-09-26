import { RootState } from '@/store';
import { Space } from 'antd';
// import { QuestionCircleOutlined } from '@ant-design/icons';
import React from 'react';
import { useSelector } from 'react-redux';
// import { useModel, SelectLang } from 'umi';
import Avatar from './AvatarDropdown';
import styles from './index.module.less';

export type SiderTheme = 'light' | 'dark';

const GlobalHeaderRight: React.FC = () => {
  const { initialState, user } = useSelector((state:RootState) => state.global);
  // const initialState =
  // console.log('rightContent')
  if (initialState === undefined || user === null) {
    return null;
  }
 
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
