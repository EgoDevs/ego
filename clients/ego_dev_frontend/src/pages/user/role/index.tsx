import { User } from '@/canisters/ego_store';
import Createform, { FormItemProps } from '@/components/Createform';
import Guide from '@/components/Guide';
import { RootDispatch, RootState } from '@/store';
import { PlusOutlined } from '@ant-design/icons';
import { ActionType, ModalFormProps, PageContainer, ProColumns, ProTable } from '@ant-design/pro-components';
import { Button } from 'antd';
import { useRef, useState } from 'react';
import { useDispatch, useSelector } from 'react-redux';

const RolePage: React.FC = () => {
  console.log('RolePage')
  const userList = useSelector((state: RootState) => state.user.userList);
  const loading = useSelector((state: RootState) => state.loading.models.user);
  const { bucketConnection, storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const dispatch = useDispatch<RootDispatch>()
  const [selectRow, setSelectRow] = useState<User>()
  const { user } = useSelector((state: RootState) => state.global)
  const [settingVisible, setSettingVisible] = useState(false)
  const tableRef = useRef<ActionType>();
  const [name, setName] = useState()
  const columns: ProColumns[] = [
    {
      title: 'UserName',
      dataIndex: 'name',
    },
    {
      title: 'is_app_auditer',
      dataIndex: 'is_app_auditer',
    },
    {
      title: 'is_manager',
      dataIndex: 'is_manager',
    },
    {
      title: 'is_app_developer',
      dataIndex: 'is_app_developer',
    },
    {
      title: 'Operation',
      dataIndex: 'option',
      search: false,
      render: (_: any, record: User) => (
        <>
          <a
            onClick={() => {
              console.log(_, record)
              setSelectRow(record)
              setSettingVisible(true)
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
      type: 'select',
      itemProps: {
        name: 'role',
        label: 'Role',
        mode: 'multiple',
        options: [
          { label: 'Auditer', value: 'Auditer' },
          { label: 'Manager', value: 'Manager' },
          { label: 'Developer', value: 'Developer' },
        ]
      }
    }
  ];

  const onSearch = (values: any) => {
    console.log(values)
    dispatch.user.getUserList({ name: values.name });
    setName(values.name)
  }

  const setRole = async (values: { role: 'Auditer' | 'Manager' | 'Developer' }) => {
    console.log('values', values)
    try {
      const result = await storeConnection?.set_role({
        user_id: selectRow?.user_id!,
        is_app_auditer: values.role.includes('Auditer'),
        is_manager: values.role.includes('Manager') ,
        is_app_developer: values.role.includes('Developer'),
      })
      console.log('result', result)
      dispatch.global.getUser({ storeConnection })
      setSettingVisible(false)
      onSearch({name})
    } catch (err) {
      console.log('err', err)
    }
  }

  const rolesValue = []
  if(selectRow?.is_manager) {
    rolesValue.push('Manager')
  }
  if(selectRow?.is_app_auditer) {
    rolesValue.push('Auditer')
  }
  if(selectRow?.is_app_developer) {
    rolesValue.push('Developer')
  }

  return (
    <PageContainer ghost>
      <ProTable
        loading={loading}
        dataSource={userList}
        columns={columns}
        onSubmit={(values) => onSearch(values)}
      />
      <Createform
        type="modal"
        formItems={formItems}
        formWraperProps={{
          visible: settingVisible,
          onFinish: async (values) => setRole(values as any),
          initialValues: {
            role: rolesValue.length > 0 ? rolesValue : undefined
          },
          modalProps: {
            destroyOnClose: true,
            width: 300,
            onCancel: () => {
              setSettingVisible(false);
            }
          }
        } as ModalFormProps}
      />
    </PageContainer>
  );
};

export default RolePage;
