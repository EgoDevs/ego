import { IRoute } from "./renderRoutes/renderRoutes";
import HomePage from "../pages/home";
import Login from '@/pages/user/Login';
import Component404 from '@/pages/404';
import LoginLayout from "@/layout/LoginLayout";
import UserLayout from "@/layout/UserLayout";

import Applications from "@/pages/applications";
import RolePage from "@/pages/role";
import UserRolePage from '@/pages/user/role';
import AccessPage from "@/pages/access";

const routes: IRoute[] = [
  {
    path: '/user',
    component: LoginLayout,
    routes: [
      {
        path: '/user',
        routes: [
          {
            name: 'login',
            path: '/user/login',
            component: Login,
          },
        ],
      },
      {
        component: Component404,
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
      // {
      //   name: 'setting',
      //   path: '/setting',
      //   component: './setting',
      //   routes: [
      //     {
      //       name: 'account',
      //       path: '/setting/account',
      //       component: './setting/account',
      //     },
      //     {
      //       name: 'wallet',
      //       path: '/setting/wallet',
      //       component: './setting/wallet',
      //     },
      //     {
      //       name: 'developer',
      //       path: '/setting/developer',
      //       component: './setting/developer',
      //     },
      //   ]
      // },
      // {
      //   name: 'access',
      //   path: '/access',
      //   component:  AccessPage,
      // },
      // {
      //   name: 'table',
      //   path: '/table',
      //   component: './table',
      // },
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
];

export default routes;
