import { ListUserResponse, MeResponse, User } from '@/../../idls/ego_store';
import { InitialStateType } from '@/layout/UserLayout';
import { StoreConnection } from '@/services/connection/store';
import { createModel } from '@rematch/core';
import type { RootModel } from '../store/models';

type UserProps = {
  userList: User []
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
      const storeConnection: StoreConnection = payload.storeConnection ?? rootState.global.initialState.storeConnection;
      const result1 = await storeConnection?.list_user({name: payload.name});
      console.log('result', result1)
      dispatch.user.save({
        userList: (result1 as { 'Ok': ListUserResponse })['Ok']['users'],
      })
    },

  }),
});

