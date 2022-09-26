import { User } from '@/../../idls/ego_store';
import { RootState } from '@/store';
import React from 'react';
import { useSelector } from 'react-redux';

import { AccessContext } from './context';

export function accessFactory(initialState: { currentUser?: User | null }) {
  const { currentUser } = initialState || {};
  return {
    canAdmin: !!(currentUser && currentUser.is_manager),
    canAudit: !!(currentUser && currentUser.is_app_auditer),
    canDeveloper: !!(currentUser && currentUser.is_app_developer)
  };
}

export function AccessProvider(props: any) {
  const user = useSelector((state: RootState) => state.global.user);
  const access = React.useMemo(() => accessFactory({currentUser: user}), [user]);

  return (
    <AccessContext.Provider value={access}>
      {props.children}
    </AccessContext.Provider>
  );
}
