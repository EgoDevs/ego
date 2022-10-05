import { Button, Form, Input, message } from 'antd';
import Footer from '@/components/Footer';
import styles from './index.module.less';
import { useDispatch, useSelector } from 'react-redux';
import { RootDispatch, RootState } from '@/store';
import { useHistory } from 'react-router';


const Register: React.FC = () => {
  console.log('homePage')
  const { storeConnection } = useSelector((state: RootState) => state.global.initialState)
  const dispatch = useDispatch<RootDispatch>()
  const history = useHistory();
  const [form] = Form.useForm();
  const handleSubmit = async (values: { name: string}) => {
    console.log('values', values)
    const result = await storeConnection?.developer_main_register({name: values.name})
    await dispatch.global.getUser({})
    console.log(result)
    message.success('register successfully.')
    history.push('/home')
  }

  return (
    <div className={styles.container}>
      <div className={styles.content}>
        <h1>Register Ego developer</h1>
        <Form form={form}  onFinish={handleSubmit}>
        <Form.Item
          label="Name"
          name="name"
          required
        >
          <Input />
        </Form.Item>
        <Button type="primary" htmlType="submit">Register</Button>
      </Form>
      </div>
      <Footer />
    </div>
  );
};

export default Register;
