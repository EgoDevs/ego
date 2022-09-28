import Createform, { FormItemProps } from "@/components/Createform";
import { RootState } from "@/store";
import { PlusOutlined } from "@ant-design/icons";
import { Version, AppVersion, App, Wasm  } from "@/../../idls/ego_dev";
import { DrawerFormProps, ModalFormProps, PageContainer, ProColumns, ProTable } from "@ant-design/pro-components";
import { ProFormDraggerProps } from '@ant-design/pro-form/lib/components/UploadDragger';
import { Button, message, Spin, Table, Tabs, UploadProps } from "antd";
import { useEffect, useRef, useState } from "react";
import { useSelector } from "react-redux";
import { file2Buffer, fileMd5, hasOwnProperty } from "@/utils";
import { Buffer } from "buffer";
import { AppVersionStatusEnum } from "@/utils/types";
import { getStatusByAppStatus } from "@/utils/dict";
import { Access, useAccess } from "@/components/Access";
import { Principal } from "@dfinity/principal";

type VersionProps = {
  appProps: App;
  handleVersionSuccess: () => void;
}

type TabKeyType = 'create' | 'audit' | 'release';

const VersionPage: React.FC<VersionProps> = (props) => {
  const { storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const { appProps, handleVersionSuccess } = props;
  const applist = useSelector((state: RootState) => state.app.applist);
  const app = applist.find((app:App) => app.app_id === appProps.app_id);
  const [createVisible, setCreateVisible] = useState(false)
  const [uploadVisible, setUploadVisible] = useState(false)
  const [settingAssetVisible, setSettingAssetVisible] = useState(false)
  const [file, setFile] = useState<File>();
  const [curTab, setCurTab] = useState<TabKeyType>('create');
  const [selectedWasms, setSelectedWasms] = useState<Wasm>();
  const [selectRecord, setSelectRecord] = useState<AppVersion>()
  const [loading, setLoading] = useState(false);
  const access = useAccess();
  console.log('app', app)
  console.log('applist', applist)
  if(!app) return null
  const getColumns = (tab: TabKeyType): ProColumns[] => {
    return [
      {
        title: 'Version',
        dataIndex: 'name',
        render: (_: any, record: any) => (
          <>
            {record.version.major}.
            {record.version.minor}.
            {record.version.patch}
          </>
        ),
      },
      {
        title: 'uploadWASM',
        dataIndex: 'wasms',
        search: false,
        render: (_: any, record: AppVersion) => (
          <>
            <a
              onClick={() => {
                console.log(_, record)
                setSelectedWasms(record.backend)
                setUploadVisible(true)
              }}
            >
              upload wasm
            </a>
          </>
        ),
      },
      {
        title: 'setting assets address',
        dataIndex: 'wasms',
        search: false,
        render: (_: any, record: AppVersion) => (
          <>
            <a
              onClick={() => {
                console.log(_, record)
                setSelectRecord(record)
                setSettingAssetVisible(true)
              }}
            >
              setting assets address
            </a>
          </>
        ),
      },
      {
        title: 'Status',
        dataIndex: 'status',
        render: (_: any, record: AppVersion) => (
          <>
            {getStatusByAppStatus(record.status)}
          </>
        ),
      },
      {
        title: 'Operation',
        dataIndex: 'option',
        search: false,
        render: (_: any, record: AppVersion) => (
          <>
            {
              tab === 'create' ? (
                <>
                  <Access accessible={true}>
                    <Spin spinning={loading} wrapperClassName="spin-inline-block">
                      <a
                        onClick={() => {
                          handleApply(record.app_id, record.version)
                        }}
                      >
                        Submit
                      </a>
                    </Spin>
                  </Access>
                </>) : tab === 'audit' ? (
                  <>
                    <Access accessible={access.canAudit}>
                      <Spin spinning={loading} wrapperClassName="spin-inline-block">
                        <a
                          onClick={() => {
                            handleApprove(record.app_id, record.version)
                          }}
                          style={{ marginRight: '10px' }}
                        >
                          Approve
                        </a>
                      </Spin>
                    </Access>
                    <Access accessible={access.canAudit}>
                      <Spin spinning={loading} wrapperClassName="spin-inline-block">
                        <a
                          onClick={() => {
                            handleReject(record.app_id, record.version)
                          }}
                          style={{ marginRight: '10px' }}
                        >
                          Reject
                        </a>
                      </Spin>
                    </Access>

                    <Spin spinning={loading} wrapperClassName="spin-inline-block">
                      <a
                        onClick={() => {
                          handleRevoke(record.app_id, record.version)
                        }}
                        style={{ marginRight: '10px' }}
                      >
                        Revoke
                      </a>
                    </Spin>

                  </>
                ) : (
                <>
                  {
                    hasOwnProperty(record.status, AppVersionStatusEnum.APPROVED) ? (
                      <Spin spinning={loading} wrapperClassName="spin-inline-block">
                        <a
                          onClick={() => {
                            handleRelease(record.app_id, record.version)
                          }}
                        >
                          Release
                        </a>
                      </Spin>
                    ) : null
                  }
                </>
              )
            }
          </>
        ),
      },
    ];
  }

  const formItems: FormItemProps[] = [
    {
      type: 'text',
      itemProps: {
        name: 'version',
        label: 'VersionNumber',
      }
    }
  ];

  const formAssetsItems: FormItemProps[] = [
    {
      type: 'render',
      render: (
        <div>
          <p>1.create canister and deploy assets.</p>
          <p>2.Execute the script added principal to the controller.</p>
          <code>
            dfx canister update-settings --add-controller {process.env.EGO_DEV_CANISTERID!} --all
          </code>
          <p>3.Execute the script added principal to authorize store.</p>
          <code>
            dfx canister call assets authorize '(principal "{process.env.EGO_DEV_CANISTERID!}")'
          </code>
          <p>dfx </p>
          <p>3.submit canisterId to store.</p>
        </div>
      )
    },
    {
      type: 'text',
      itemProps: {
        name: 'canister_id',
        label: 'CanisterId',
      }
    }
  ];

  const uploadItems: FormItemProps[] = [
    {
      type: "uploadDragger",
      itemProps: {
        name: 'backend',
        label: 'Backend',
        beforeUpload: async (file: any) => {
          // setFileList([...fileList, file]);
          console.log('beforeUpload', file)
          setFile(file)
          const md5 = await fileMd5(file);
          console.log('md5', md5)
          return false;
        },
        accept: '.wasm',
        customRequest: (options: any) => {
          console.log('customRequest', options)
        },
        max: 1,
        onDrop: (e: any) => {
          console.log('onDrop', e.dataTransfer.files)
        },
      } as UploadProps
    },
    // {
    //   type: "uploadDragger",
    //   itemProps: {
    //     name: 'backend',
    //     label: 'Backend',
    //     beforeUpload: (file: any) => {
    //       setFileList([...fileList, file]);
    //       return false;
    //     },
    //     onChange: (info: any) => {
    //       console.log(info)
    //     },
    //     fileList,
    //   } as ProFormDraggerProps
    // },
  ]

  const handleApply = async (app_id: string, version: Version) => {
    setLoading(true)
    try {
      const result = await storeConnection?.app_version_submit({ version, app_id });
      console.log(result)
      handleRefresh()
    } catch (e) {
      console.warn(e)
      setLoading(false)
    }
  }

  const handleReject = async (app_id: string, version: Version) => {
    setLoading(true)
    try {
      const result = await storeConnection?.app_version_reject({ version, app_id });
      console.log(result)
      handleRefresh()
    } catch (e) {
      console.warn(e)
      setLoading(false)
    }
  }

  const handleApprove = async (app_id: string, version: Version) => {
    setLoading(true)
    try {
      const result = await storeConnection?.app_version_approve({ version, app_id });
      console.log(result)
      handleRefresh()
    } catch (e) {
      console.warn(e)
      setLoading(false)
    }
  }
  const handleRelease = async (app_id: string, version: Version) => {
    setLoading(true)
    try {
      const result = await storeConnection?.app_version_release({ version, app_id });
      console.log(result)
      handleRefresh()
    } catch (e) {
      console.warn(e)
      setLoading(false)
    }
  }

  const handleRevoke = async (app_id: string, version: Version) => {
    setLoading(true)
    try {
      const result = await storeConnection?.app_version_revoke({ version, app_id });
      console.log(result)
      handleRefresh()
    } catch (e) {
      console.warn(e)
      setLoading(false)
    }
  }

  const handleRefresh = () => {
    handleVersionSuccess()
    message.success('operation success')
    setCreateVisible(false)
    setLoading(false)
  }


  const onCreate = async (values: any) => {
    try {
      console.log(values)
      const version = {
        'major': Number(values.version.split('.')[0]),
        'minor': Number(values.version.split('.')[1]),
        'patch': Number(values.version.split('.')[2]),
      }
      const result = await storeConnection?.app_version_new({ version, app_id: app.app_id });
      console.log('create result====', result)
      handleVersionSuccess()
      setCreateVisible(false)
    }
    catch (e) {
      console.log(e)
    }
  }

  const onUpload = async () => {
    if (!file) return;
    try {
      const fileBuffer = await file2Buffer(file);
      const md5 = await fileMd5(file);
      const data = Array.from(new Uint8Array(fileBuffer))

      const result = await storeConnection?.app_version_upload_wasm({
        app_id: (selectRecord as AppVersion).app_id,
        version: (selectRecord as AppVersion).version,
        hash: md5,
        data,
      })
      console.log('result', result)
      handleRefresh()
    } catch (e) {
      console.log(e)
    }
  }

  const settingAssetsCanister = async (values: any) => {
    try {
      const result = await storeConnection?.app_version_set_frontend_address({
        app_id: (selectRecord as AppVersion).app_id,
        version: (selectRecord as AppVersion).version,
        canister_id: Principal.fromText(values.canister_id),
      })
      console.log(result)
      handleRefresh()
    } catch (e) { 
      console.log(e)
    }
    
  }

  return (<>
    <Tabs defaultActiveKey="create" onChange={(tab) => {
      console.log(tab)
      setCurTab(tab as TabKeyType)
    }}>
      <Tabs.TabPane tab="Create" key="create">
        <ProTable
          search={false}
          dataSource={app.versions.filter((v) =>
            hasOwnProperty(v.status, AppVersionStatusEnum.NEW) ||
            hasOwnProperty(v.status, AppVersionStatusEnum.REJECTED) ||
            hasOwnProperty(v.status, AppVersionStatusEnum.REVOKED)
          )}
          toolBarRender={() => [
            <Button key="button" icon={<PlusOutlined />} type="primary" onClick={() => setCreateVisible(true)}>
              Create
            </Button>,
          ]}
          columns={getColumns(curTab)}
        />
      </Tabs.TabPane>
      <Tabs.TabPane tab="Audit" key="audit">
        <ProTable
          search={false}
          dataSource={app.versions.filter((v) =>
            hasOwnProperty(v.status, AppVersionStatusEnum.SUBMITTED) ||
            hasOwnProperty(v.status, AppVersionStatusEnum.SUBMITTED)
          )}
          toolBarRender={false}
          columns={getColumns(curTab)}
        />
      </Tabs.TabPane>
      <Tabs.TabPane tab="Release" key="release">
        <ProTable
          search={false}
          dataSource={
            app.versions.filter((v) =>
              hasOwnProperty(v.status, AppVersionStatusEnum.RELEASED) ||
              hasOwnProperty(v.status, AppVersionStatusEnum.APPROVED)
            )
          }
          toolBarRender={false}
          columns={getColumns(curTab)}
        />
      </Tabs.TabPane>
    </Tabs>

    <Createform
      type="modal"
      formItems={formItems}
      formWraperProps={{
        visible: createVisible,
        onFinish: onCreate,
        modalProps: {
          destroyOnClose: true,
          width: 300,
          onCancel: () => {
            setCreateVisible(false);
          }
        }

      } as ModalFormProps}
    />
    <Createform
      type="modal"
      formItems={uploadItems}
      formWraperProps={{
        visible: uploadVisible,
        onFinish: onUpload,
        modalProps: {
          destroyOnClose: true,
          width: 300,
          onCancel: () => {
            setUploadVisible(false);
          }
        }

      } as ModalFormProps}
    />
    <Createform
      type="modal"
      formItems={formAssetsItems}
      formWraperProps={{
        visible: settingAssetVisible,
        onFinish: settingAssetsCanister,
        modalProps: {
          destroyOnClose: true,
          width: 300,
          onCancel: () => {
            setSettingAssetVisible(false);
          }
        }

      } as ModalFormProps}
    />
  </>)
}


export default VersionPage