import { DevConnection } from '@/services/connection/dev';
import { createModel } from '@rematch/core';
import type { RootModel } from '../store/models';

type AppProps = {
  applist: []
};

export const app = createModel<RootModel>()({
  state: {
    applist: []
  } as AppProps,
  reducers: {
    save(state, payload) {
      return {
        ...state,
        ...payload,
      };
    },
  },
  effects: dispatch => ({
    async getApplist(payload, rootState) {
      const { initialState, user } = rootState.global;
      let result: any;
      if(user?.is_app_auditer) {
        result = await (initialState.storeConnection as DevConnection).app_version_wait_for_audit();
        console.log('auditer', result)
      } else {
        result = await initialState.storeConnection?.developer_app_list();
        console.log('developer', result)
      }
      dispatch.app.save({ applist: result['Ok']['apps'] });
    },
  }),
});

