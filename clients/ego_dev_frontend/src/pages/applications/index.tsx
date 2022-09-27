import { App } from '@/../../idls/ego_store';
import Createform, { FormItemProps } from '@/components/Createform';
import { CategoryEnum } from '@/services/connection/dev';
import { ProDescriptions } from '@ant-design/pro-components';
import { RootDispatch, RootState } from '@/store';
import { PlusOutlined } from '@ant-design/icons';
import { DrawerFormProps, PageContainer, ProColumns, ProTable } from '@ant-design/pro-components';
import { Button, Drawer, Table, Tabs } from 'antd';
import React, { useEffect, useState } from 'react';
import { useDispatch } from 'react-redux';
import { useSelector } from 'react-redux';
import Version from './Version';


const Applications: React.FC = (props) => {
  console.log('Applications props', props)
  const [createVisible, setCreateVisible] = useState(false)
  const { storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const { user } = useSelector((state: RootState) => state.global)
  const loading  = useSelector((state: RootState) => state.loading.models.app)
  const dispatch = useDispatch<RootDispatch>();
  const [versionVisible, setVersionVisible] = useState(false)
  const applist = useSelector((state: RootState) => state.app.applist);
  const [selectedApp, setSelectedApp] = useState<App>();
  console.log(applist)
  console.log('loading', loading)
  console.log('user', user)
  const columns: ProColumns[] = [
    {
      title: 'PackageId',
      dataIndex: 'app_id',
      search: false,
    },
    {
      title: 'Status',
      dataIndex: 'Application2',
      search: false,
    },
    {
      title: 'Updated date',
      dataIndex: 'Application3',
      search: false,
    },
    {
      title: 'Operation',
      dataIndex: 'option',
      search: false,
      render: (_: any, record: any) => (
        <>
          <a
            onClick={() => {
              console.log(_, record)
              setSelectedApp(record)
              setVersionVisible(true)
            }}
          >
            Configure
          </a>
        </>
      ),
    },
  ]

  const formItems: FormItemProps[] = [
    {
      type: 'group',
      group: [
        {
          type: 'text',
          itemProps: {
            label: 'App name',
            name: 'name',
            rules: [
              {
                required: true,
                message: 'Please input app name',
              },
              {
                max: 50,
                message: 'Max length is 50',
              }
            ],
          }
        },
        {
          type: 'text',
          itemProps: {
            label: 'App ID',
            name: 'app_id',
            rules: [
              {
                required: true,
                message: 'Please input app id',
              },
              // {
              //   pattern: /^([0-9]{1, 2})\.([0-9]{1, 2})\.([0-9]{1, 2})$/,
              //   message: 'version error',
              // }
            ]
          }
        },
      ]
    },
    {
      type: 'radio-group',
      itemProps: {
        label: 'App type',
        name: 'category',
        rules: [
          {
            required: true,
            message: 'Please select app type',
          },
        ],
        options: [
          {
            value: 'System',
            label: 'System'
          },
          {
            value: 'Vault',
            label: 'Vault'
          }
        ]
      }
    },
    {
      type: "radio-group",

      itemProps: {
        name: 'priceType',
        // layout="vertical",
        label: "Pricing",
        rules: [
          {
            required: true,
            message: 'Please select app type',
          },
        ],
        options: [
          {
            value: 0,
            label: 'Free'
          },
          {
            value: 1,
            label: 'Paid'
          }
        ]
      }
    },
    {
      type: "text",
      itemProps: {
        label: '',
        name: 'price',
        width: 100,
        rules: [
          {
            required: true,
            message: 'Please inpt price',
          },
        ],
      },
      dependencyName: 'priceType',
    }
  ]

  useEffect(() => {
    handleSearch()
  }, [])

  // const init = async () => {
  //   console.log('init', bucketActor)
  //   const result = await storeActor.list_app()
  //   console.log('result', result)
  // }

  const handleSearch = () => {
    dispatch.app.getApplist({})
  }

  const onCreate = async (values: any) => {
    console.log(values)
    const params = {
      category: values.category === 'System' ? CategoryEnum['System'] : CategoryEnum['Vault'],
      price: values.priceType === 0 ? 0 : Number(values.price),
      app_id: values.app_id.toLowerCase(),
      name: values.name.toLowerCase(),
      logo: 'this is logo',
      description: "this is description"
    }
    console.log(params)
    try {
      const result = await storeConnection?.developer_app_new(params)
      console.log('result', result)
      handleSearch()
      setCreateVisible(false)
    } catch (err) {
      console.log('err', err)
    }

  }

  return (
    <PageContainer
      ghost
      header={{
        title: 'Applications',
      }}
    >
      <ProTable
        loading={loading}
        dataSource={applist}
        toolBarRender={() => [
          <Button key="button" icon={<PlusOutlined />} type="primary" onClick={() => setCreateVisible(true)}>
            Create
          </Button>,
        ]}
        columns={columns}
      />

      <Createform
        type="drawer"
        formItems={formItems}
        formWraperProps={{
          visible: createVisible,
          onFinish: onCreate,
          drawerProps: {
            destroyOnClose: true,
            onClose: () => {
              setCreateVisible(false);
            }
          }

        } as DrawerFormProps}
      />
      <Drawer
        title="Application Configure"
        placement="right"
        width={720}
        destroyOnClose
        visible={versionVisible}
        onClose={() => {
          setVersionVisible(false)
        }}
      >
        <Tabs>
          <Tabs.TabPane tab="Version" key="version">
            <Version 
              app={selectedApp!} 
              handleVersionSuccess={() => {
                handleSearch()
                setVersionVisible(false)
              }}
            />
          </Tabs.TabPane>
          <Tabs.TabPane tab="Detail" key="detail">
            <ProDescriptions column={1} >
              <ProDescriptions.Item label="PackageId">
              </ProDescriptions.Item>
              <ProDescriptions.Item label="Status">
              </ProDescriptions.Item>
            </ProDescriptions>
          </Tabs.TabPane>
        </Tabs>
      </Drawer>
    </PageContainer>
  );
}


export default Applications;