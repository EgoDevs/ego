import { useState } from 'react'
import reactLogo from './assets/react.svg'
import './App.css'
import { PermissionsType } from '@astrox/connection/lib/cjs/types'
import { _SERVICE as storeService } from './canisters/ego_store'
import { idlFactory as storeIdl } from './canisters/ego_store.idl'
import { IcObject } from './utils'
import { IC } from '@astrox/connection'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src="/vite.svg" className="logo" alt="Vite logo" />
        </a>
        <a href="https://reactjs.org" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Vite + React</h1>
      <div className="card">
        <button onClick={() => setCount((count) => count + 1)}>
          count is {count}
        </button>
        <button onClick={() => {
          console.log(process.env.EGO_DEV_CANISTERID)
          IcObject.connect({
            // dev: true,
            useFrame: !(window.innerWidth < 768),
            signerProviderUrl: 'http://localhost:8080/signer',
            walletProviderUrl: 'http://localhost:8080/transaction', // 'http://localhost:8080/transaction', // "https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/transaction",
            identityProvider: 'http://localhost:8080/login#authorize', // 'http://localhost:8080/login#authorize', // 'https://i3z5x-xaaaa-aaaah-aa2za-cai.raw.ic0.app/login#authorize',
            permissions: [PermissionsType.identity, PermissionsType.wallet],
            onAuthenticated: async (icInstance: IC) => {
              console.log('icInstance', icInstance)
              const storeActor = icInstance.createActor<storeService>(
                storeIdl,
                process.env.EGO_DEV_CANISTERID!,
              );
              console.log('storeActor', storeActor)
              try {
                const wasmFiles = await storeActor.list_app({
                  query_param: { ByCategory: { category: { Vault: null } } },
                });
                console.log('registerResponse', wasmFiles)
              } catch (error) {
                console.log('error', error)
              }
              try {
                const registerResponse = await storeActor.register_app({
                  app_id: 'app_id_12121',
                  name: 'app name',
                  category: { Vault: null },
                  price: 100,
                });
                console.log('registerResponse', registerResponse)
              } catch (error) {
                console.log('error', error)
              }
            },
          })
        }}>
          login
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
      <p className="read-the-docs">
        Click on the Vite and React logos to learn more
      </p>
    </div>
  )
}

export default App
