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
  const handleSubmit = async (values: { name: string}) => {
    console.log('values', values)
    const result = await storeConnection?.developer_main_register({name: values.name})
    await dispatch.global.getUser({})
    console.log(result)
    message.success('register successfully.')

    form.resetFields()
  }
  return (
    <PageContainer ghost>
      <Guide name={'Welcome to Ego center'}></Guide>
    </PageContainer>
  );
};

export default HomePage;
