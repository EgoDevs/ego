import { User } from '@/../../idls/ego_store';
import { InitialStateType } from '@/layout/UserLayout';
import { DevConnection } from '@/services/connection/dev';
import { createModel } from '@rematch/core';
import type { RootModel } from '../store/models';
import {Result_4} from "../../../idls/ego_dev";

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
      const storeConnection: DevConnection = payload.storeConnection ?? rootState.global.initialState.storeConnection;
      const result1 = await storeConnection?.developer_main_get();
      console.log('result', result1)
      dispatch.global.save({
        user: (result1 as { 'Ok': Result_4 })['Ok']['developer'],
      })
    },

  }),
});

