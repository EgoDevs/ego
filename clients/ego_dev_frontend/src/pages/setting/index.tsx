import { PageContainer } from '@ant-design/pro-components';
import { useLocation, useIntl, history } from '@umijs/max';
import { Tabs } from 'antd';
import React from 'react';
import Account from './account';
import Developer from './developer';
import Wallet from './wallet';
// import styles from './index.less';
const { TabPane } = Tabs;

export default function Page() {
  const intl = useIntl()
  const location = useLocation();
  const tabsOption = [
    {
      title: intl.formatMessage({ id: 'menu.setting.account' }),
      route: '/setting/account',
      content: <Account />
    },
    {
      title: intl.formatMessage({ id: 'menu.setting.wallet' }),
      route: '/setting/wallet',
      content: <Wallet />
    },
    {
      title: intl.formatMessage({ id: 'menu.setting.developer' }),
      route: '/setting/developer',
      content: <Developer />
    }
  ]

  const onChange = (tab: string) => {
    history.push(tab)
  }

  return (
    <PageContainer
      ghost
      header={{
        title: 'Settings',
      }}
    >
      <Tabs 
        defaultActiveKey={'/setting/account'} 
        activeKey={location.pathname} 
        onChange={onChange}
      >
        {
          tabsOption.map(option => (
            <TabPane tab={option.title} key={option.route} >
              {option.content}
            </TabPane>
          ))
        }
      </Tabs>
    </PageContainer>
  );
}
