import React from 'react'
import ReactDOM from 'react-dom/client'
import './index.less'
import Home from '@/pages/applications'
import RouterContainer from './routes/Router'
import { Provider } from 'react-redux';
import { AccessProvider } from '@/components/Access/runtime';
import { store } from './store'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Provider store={store}>
      <AccessProvider>
        <RouterContainer />
      </AccessProvider>
    </Provider>
  </React.StrictMode>
)
