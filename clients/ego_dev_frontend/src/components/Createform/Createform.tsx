import {
  ModalFormProps,
  DrawerFormProps,
  QueryFilterProps,
  ProFormDigit,
  ProFormUploadButton,
  ProFormSwitch,
} from '@ant-design/pro-form';
import { ProFormDraggerProps } from '@ant-design/pro-form/lib/components/UploadDragger';
import ProForm, {
  ProFormDateTimePicker,
  ProFormSelect,
  ProFormText,
  ProFormTextArea,
  ModalForm,
  DrawerForm,
  ProFormUploadDragger,
  QueryFilter,
} from '@ant-design/pro-form';
import { Upload, UploadProps } from 'antd';
import type { ProFormSelectProps } from '@ant-design/pro-form/lib/components/Select';
import type { ProFieldProps } from '@ant-design/pro-utils';
import type { ReactNode } from 'react';
import { ProFormDependency, ProFormRadio } from '@ant-design/pro-components';
import { InboxOutlined } from '@ant-design/icons';

export interface FormItemProps {
  type:
  | 'text'
  | 'number'
  | 'select'
  | 'switch'
  | 'radio-group'
  | 'textArea'
  | 'dateTime'
  | 'upload'
  | 'uploadDragger'
  | 'group'
  | 'password'
  | 'dependency'
  | 'render';
  itemProps?: ProFormSelectProps | ProFieldProps | FormItemProps | ProFormDraggerProps;
  dependencyName?: string;
  dependencyrequest?: (values: any) => void;
  group?: FormItemProps[];
  render?: ReactNode;
  children?: ReactNode;
}

interface CreateFormProps {
  formItems: FormItemProps[];
  type: 'modal' | 'drawer' | 'query';
  // onFinish: (values: any) => void;
  formWraperProps: ModalFormProps | DrawerFormProps | QueryFilterProps;
}

const CreateForm: React.FC<CreateFormProps> = (props) => {
  const { formItems, type, formWraperProps } = props;

  const renderField = (item: FormItemProps, index: number) => {
    if (item.type === 'textArea') {
      return <ProFormTextArea key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'number') {
      return <ProFormDigit key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'password') {
      return <ProFormText.Password key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'select') {
      return <ProFormSelect key={index} {...(item.itemProps as ProFormSelectProps)} />;
    } else if (item.type === 'switch') {
      return <ProFormSwitch key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'radio-group') {
      return <ProFormRadio.Group key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'dateTime') {
      return <ProFormDateTimePicker key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'group') {
      return (
        <ProForm.Group key={index}>
          {item.group?.map((groupItem, groupIndex) => renderField(groupItem, groupIndex))}
        </ProForm.Group>
      );
    }
    else if (item.dependencyName !== undefined) {
      console.log('dependencyName', item.dependencyName)

      return (
        <ProFormDependency name={[item.dependencyName!]}>
          {(values) => {
            console.log('values', values[item.dependencyName!], { ...item, dependencyName: undefined })
            if (values[item.dependencyName!]) {
              console.log('11', renderField({ ...item, dependencyName: undefined }, index))
              return renderField({ ...item, dependencyName: undefined }, index);
            }
            return null;
          }}
        </ProFormDependency>
      );
    }
    else if (item.type === 'upload') {
      return <ProFormUploadButton key={index} {...(item.itemProps as ProFieldProps)} />;
    } else if (item.type === 'uploadDragger') {
      return <Upload.Dragger key={index} {...(item.itemProps as UploadProps)} >
        <p className="ant-upload-drag-icon">
          <InboxOutlined />
        </p>
        <p className="ant-upload-text">Click or drag file to this area to upload</p>
        <p className="ant-upload-hint">
          Support for a single or bulk upload. Strictly prohibit from uploading company data or other
          band files
        </p>
      </Upload.Dragger>;
    } else if (item.type === 'text') {
      return <ProFormText key={index} {...(item.itemProps as ProFieldProps)} />;
    } else {
      return item.render;
    }
  };


  if (type === 'modal') {
    return (
      <ModalForm {...(formWraperProps as ModalFormProps)}>
        {formItems.map((item, index) => renderField(item, index))}
      </ModalForm>
    );
  } else if (type === 'drawer') {
    return (
      <DrawerForm {...(formWraperProps as DrawerFormProps)}>
        {formItems.map((item, index) => renderField(item, index))}
      </DrawerForm>
    );
  } else {
    return (
      <QueryFilter {...(formWraperProps as QueryFilterProps)}>
        {formItems.map((item, index) => renderField(item, index))}
      </QueryFilter>
    );
  }
};

export default CreateForm;
