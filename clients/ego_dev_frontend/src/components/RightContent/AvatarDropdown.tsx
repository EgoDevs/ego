import React, { useCallback } from 'react';
import { LogoutOutlined } from '@ant-design/icons';
import { Avatar, Menu, message, Spin } from 'antd';
// import { history, useModel } from 'umi';
import styles from './index.module.less';
import type { MenuInfo } from 'rc-menu/lib/interface';
import HeaderDropdown from '../HeaderDropdown';
import { useHistory } from 'react-router-dom';
import { InitialStateType } from '@/layout/UserLayout';
import { useSelector } from 'react-redux';
import { RootState } from '@/store';
// @ts-ignore
import { useCopyToClipboard } from 'react-use-copy-to-clipboard';
import { User } from '@/../../idls/ego_store';
import { client } from '@/main';

export type GlobalHeaderRightProps = {
  menu?: boolean;
};


const AvatarDropdown: React.FC<GlobalHeaderRightProps> = ({ menu }) => {
  const { initialState, user }: { initialState: InitialStateType | undefined, user: User | null } = useSelector((state: RootState) => state.global);
  const history = useHistory();
  /**
   * 退出登录，并且将当前的 url 保存
   */
  const loginOut = async () => {
    const { search, pathname } = history.location;
    client.disconnect();
    // Note: There may be security issues, please note
    if (window.location.pathname !== '/user/login') {
      history.replace({
        pathname: '/user/login',

      });
    }
  };
  const onMenuClick = useCallback(
    (event: MenuInfo) => {
      const { key } = event;
      if (key === 'logout') {
        // setInitialState((s) => ({ ...s, currentUser: undefined }));
        loginOut();
        return;
      }
      history.push(`/account/${key}`);
    },
    [],
  );

  const loading = (
    <span className={`${styles.action} ${styles.account}`}>
      <Spin
        size="small"
        style={{
          marginLeft: 8,
          marginRight: 8,
        }}
      />
    </span>
  );
  if (initialState === undefined) {
    return loading;
  }

  const { currentUser } = initialState;
  const principal = currentUser?.principal.toText();
  if (currentUser === undefined || currentUser === undefined) {
    return loading;
  }

  const menuHeaderDropdown = (
    <Menu className={styles.menu} selectedKeys={[]} onClick={onMenuClick}>
      <Menu.Item key="logout">
        <LogoutOutlined />
        退出登录
      </Menu.Item>
    </Menu>
  );
  // const copyRef = useCopyToClipboard(
  //   principal,
  //   () => {
  //     message.success('Copied');
  //   },
  //   () =>
  //     message.error({
  //       icon: 'fail',
  //       content: 'Unable to copy!',
  //     }),
  // );
  return (
    <HeaderDropdown overlay={menuHeaderDropdown}>
      <span className={`${styles.action} ${styles.account}`}>
        <Avatar size="small" className={styles.avatar} alt="avatar" />
        <span style={{ marginRight: 20 }}>Roles:{user?.is_app_auditer ? 'Auditer' : ''} {user?.is_app_developer ? 'Developer' : ''} {user?.is_manager ? 'Manager' : ''}</span>
        <span style={{ marginRight: 10 }}>{user?.name}</span>
        <span className={`${styles.name} anticon`}>{`${principal?.slice(0, 4)}...${principal?.slice(-6)}`}</span>

      </span>
    </HeaderDropdown>
  );
};

export default AvatarDropdown;
