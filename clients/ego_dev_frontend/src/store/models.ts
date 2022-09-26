import { Models } from '@rematch/core';
import { app } from '../models/app';
import { global } from '../models/global';
import { user } from '../models/user';
export interface RootModel extends Models<RootModel> {
  app: typeof app;
  global: typeof global;
  user: typeof user;
}

export const models: RootModel = { app, global, user };
