import React from "react";
import ReactDOM from "react-dom";
import { BrowserRouter } from "react-router-dom";
import { Provider } from 'react-redux';
import { store } from './store'

import "./index.less";
import RouterContainer from "./routes/Router";
import { AccessProvider } from "./components/Access/runtime";

import { ClientConnecttion } from '@kasumisk/sdk';

export const client = new ClientConnecttion({
  clientOptions: {
    identityProvider: 'http://localhost:8080/login#authorize',
    walletProviderUrl: 'http://localhost:8080/#transaction',
    signerProviderUrl: 'http://localhost:8080/#signer',
    delegationTargets: [
      process.env.EGO_BUCKET_CANISTERID!,
      process.env.EGO_STORE_CANISTERID!,
      process.env.EGO_WALLET_CANISTERID!,
    ],
  
  } 
});

ReactDOM.render(
  <React.StrictMode>
    <BrowserRouter>
      <Provider store={store}>
        <AccessProvider>
          <RouterContainer />
        </AccessProvider>
      </Provider>
    </BrowserRouter>
  </React.StrictMode>,
  document.getElementById("root")
);
