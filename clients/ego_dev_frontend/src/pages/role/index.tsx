import { MeResponse } from "@/../../idls/ego_store";
import { RootDispatch, RootState } from "@/store";
import { PageContainer } from "@ant-design/pro-components"
import { Button, Form, Select } from "antd";
import { type } from "os";
import { useDispatch, useSelector } from "react-redux";


type PageState = { 
  ma: string,
  foo: number,
}

type PageDispatch = { 
  dispatch: () => void;
  ma: string;
}

type PageState1<T> = { 
  ma: string,
  foo: number,
  aaa: T
}
// type CurPageStata = keyof PageState;
// type State1 = Omit<PageState, 'ma'>;
// type State2 = Pick<PageState, 'ma'>;
// type State3 = PageState1<PageDispatch>;
// type State4 = Exclude<PageDispatch, PageState>;
// type State5 = Extract<PageState, PageDispatch>;
// type State6 = Readonly<PageState1<PageDispatch>>;
// type State7 = Record<keyof PageState, PageDispatch>;

// const aaa: State2 = {
//   ma: '1',
// }
// const bbb: State3 = { 
//   ma: '1',
//   foo: 0,
//   aaa: {
//     dispatch: () => {},
//     ma: '1'
//   } 
// }

// const ccc: State4 = {
//   dispatch: () => {},
//   ma: '1',
// }

// const ddd: State5 = []
const RolePage = () => {
  const { bucketConnection, storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const dispatch = useDispatch<RootDispatch>()
  const { user } = useSelector((state: RootState) => state.global)

  const setRole = async (values: { role: 'Auditer' | 'Manager' | 'Developer' }) => {
    console.log('values', values)
    try {
      const result = await storeConnection?.set_role({
        user_id: user?.user_id!,
        is_app_auditer: values.role === 'Auditer',
        is_manager: values.role === 'Manager',
        is_app_developer: values.role === 'Developer',
      })
      console.log('result', result)
      dispatch.global.getUser({ storeConnection })
    } catch (err) {
      console.log('err', err)
    }
  }

  
  return (
    <PageContainer>
      <Form
        name="basic"
        labelCol={{ span: 8 }}
        wrapperCol={{ span: 16 }}
        initialValues={{ remember: true }}
        onFinish={setRole}
        autoComplete="off"
      >
        <Form.Item
          label="Role"
          name="role"
          rules={[{ required: true, message: 'Please input your username!' }]}
        >
          <Select options={[
            { label: 'Auditer', value: 'Auditer' },
            { label: 'Manager', value: 'Manager' },
            { label: 'Developer', value: 'Developer' },
          ]} />
        </Form.Item>
        <Form.Item wrapperCol={{ offset: 8, span: 16 }}>
          <Button type="primary" htmlType="submit">
            Submit
          </Button>
        </Form.Item>
      </Form>
    </PageContainer>
  )
}

export default RolePage;