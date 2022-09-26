import { AntDesignOutlined, CrownOutlined, SmileOutlined, TabletOutlined } from '@ant-design/icons';

export default {
  route: {
    path: '/',
    routes: [
      {
        path: '/welcome',
        name: '欢迎',
      },
      {
        path: '/admin',
        name: '管理页',
        component: './Admin',
        routes: [
          {
            path: '/admin/sub-page1',
            name: '一级页面',
          },
          {
            path: '/admin/sub-page2',
            name: '二级页面',
          },
          {
            path: '/admin/sub-page3',
            name: '三级页面',
          },
        ],
      },
      {
        name: '列表页',
        path: '/list',
        routes: [
          {
            path: '/list/sub-page',
            name: '一级列表页面',
            routes: [
              {
                path: 'sub-sub-page1',
                name: '一一级列表页面',
              },
              {
                path: 'sub-sub-page2',
                name: '一二级列表页面',
              },
              {
                path: 'sub-sub-page3',
                name: '一三级列表页面',
              },
            ],
          },
          {
            path: '/list/sub-page2',
            name: '二级列表页面',
          },
          {
            path: '/list/sub-page3',
            name: '三级列表页面',
          },
        ],
      },
      {
        path: 'https://ant.design',
        name: 'Ant Design 官网外链',
      },
    ],
  },
  location: {
    pathname: '/',
  },
};