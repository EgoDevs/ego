import Guide from '@/components/Guide';
import { RootDispatch, RootState } from '@/store';
import { PageContainer } from '@ant-design/pro-components';
import { Button, Form, FormInstance, Input, message, Radio } from 'antd';
import { useForm } from 'antd/es/form/Form';
import { useRef } from 'react';
import { useDispatch } from 'react-redux';
import { useSelector } from 'react-redux';
import styles from './index.module.less';

const HomePage: React.FC = () => {
  console.log('homePage')
  const { storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const dispatch = useDispatch<RootDispatch>()
  const [form] = Form.useForm();
  const handleSubmit = async (values: { name: string, role: 'developer' | 'user'}) => {
    console.log('values', values)
    let result

    result = await storeConnection?.register_developer({name: values.name})
    dispatch.global.getUser({})
    console.log(result)
    message.success('register successfully.')
    // console.log()
    form.resetFields()
  }
  return (
    <PageContainer ghost>
      <Form form={form}  onFinish={handleSubmit}>
        <Form.Item
          label="Name"
          name="name"
          required
        >
          <Input />
        </Form.Item>
        <Form.Item
          label="Role"
          name="role"
          required
        >
          <Radio.Group value="developer">
            <Radio.Button value="developer">developer</Radio.Button>
            {/* <Radio.Button value="user">user</Radio.Button> */}
          </Radio.Group>
        </Form.Item>
        <Button type="primary" htmlType="submit">Register</Button>
      </Form>
    </PageContainer>
  );
};

export default HomePage;
