import { IRoute } from "./renderRoutes/renderRoutes";
import HomePage from "../pages/home";
import Login from '@/pages/user/Login';
import Component404 from '@/pages/404';
import UserLayout from "@/layout/UserLayout";

import Applications from "@/pages/applications";
import RolePage from "@/pages/role";
import UserRolePage from '@/pages/user/role';
import Register from "@/pages/user/Login/Register";

const routes: IRoute[] = [
  {
    path: '/user',
    routes: [
      {
        path: '/user',
        routes: [
          {
            name: 'login',
            path: '/user/login',
            component: Login,
          },
          {
            name: 'login',
            path: '/user/register',
            component: Register,
          },
        ],
      },
      
    ],
  },
  {
    path: '/',
    component: UserLayout,
    routes: [
      {
        name: 'home',
        path: '/home',
        component: HomePage,
      },
      {
        name: 'applications',
        path: '/applications',
        component: Applications,
      },
      {
        name: 'role',
        path: '/role',
        component: RolePage,
        access: 'canAdmin',
      },
      {
        name: 'user',
        path: '/user_manage',
        access: 'canAdmin',
        routes: [
          {
            name: 'role',
            path: '/user_manage/role',
            component: UserRolePage,
          }
        ]
      },
    ]
  },
  {
    path: '*',
    component: Component404,
  },
];

export default routes;
