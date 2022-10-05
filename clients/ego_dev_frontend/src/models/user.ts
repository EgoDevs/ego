import { DevConnection } from '@/services/connection/dev';
import { createModel } from '@rematch/core';
import type { RootModel } from '../store/models';
import {Developer, Result_8} from "../../../idls/ego_dev";

type UserProps = {
  userList: Developer []
};

export const user = createModel<RootModel>()({
  state: {
    userList: []
  } as UserProps,
  reducers: {
    save(state, payload) {
      return {
        ...state,
        ...payload,
      };
    },
  },
  
  effects: dispatch => ({
    async getUserList(payload, rootState) {
      const storeConnection: DevConnection = payload.storeConnection ?? rootState.global.initialState.storeConnection;
      const result1 = await storeConnection?.user_main_list({name: payload.name});
      console.log('result', result1)
      dispatch.user.save({
        userList: result1.users,
      })
    },

  }),
});

