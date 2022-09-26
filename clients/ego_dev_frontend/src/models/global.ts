import { MeResponse, User } from '@/canisters/ego_store';
import { InitialStateType } from '@/layout/UserLayout';
import { StoreConnection } from '@/services/connection/store';
import { createModel } from '@rematch/core';
import type { RootModel } from '../store/models';

type GlobalProps = {
  initialState: InitialStateType,
  user: User | null
};

export const global = createModel<RootModel>()({
  state: {
    initialState: {

    },
    user: null
  } as GlobalProps,
  reducers: {
    save(state, payload) {
      return {
        ...state,
        ...payload,
      };
    },
  },
  effects: dispatch => ({
    async getUser(payload, rootState) {
      const storeConnection: StoreConnection = payload.storeConnection ?? rootState.global.initialState.storeConnection;
      const result1 = await storeConnection?.me();
      console.log('result', result1)
      dispatch.global.save({
        user: (result1 as { 'Ok': MeResponse })['Ok']['user'],
      })
    },

  }),
});

