import file from 'fs';
import shell from 'shelljs';
import yargs from 'yargs';
import fs from 'fs';
import path from 'path';
import { Secp256k1KeyIdentity } from '@dfinity/identity';
import { appsConfig, infraConfig, dfxConfigTemplate, Configs } from './config';
import {
  cycleWalletActor,
  cycleWalletCanisterId,
  managementActor,
  readConfig,
  readEgoDfxJson,
  readWasm,
} from './manager';
import { Actor, getManagementCanister } from '@dfinity/agent';
import { Principal } from '@dfinity/principal';
import { isProduction } from './env';
// import curve from 'starkbank-ecdsa';
import { identity } from './settings/identity';
import { hasOwnProperty } from './settings/utils';
import { IDL } from '@dfinity/candid';

// const BIP32Factory = require('bip32');
// const bip39 = require('bip39');
// const ecc = require('tiny-secp256k1');

// function getIdentityFromPhrase(phrase: string) {
//   const seed = bip39.mnemonicToSeedSync(phrase);

//   const ICP_PATH = "m/44'/223'/0'";
//   const path = `${ICP_PATH}/0/0`;

//   const bip32 = BIP32Factory.default(ecc);

//   let node = bip32.fromSeed(seed);

//   let child = node.derivePath(path);

//   return Secp256k1KeyIdentity.fromSecretKey(child.privateKey);
//   // return seed;
// }

// const seedPhrase = fs
//   .readFileSync(
//     path.join(
//       process.cwd(),
//       '/credentials',
//       !isProduction ? '/internal.txt' : '/production.txt',
//     ),
//     {
//       encoding: 'utf8',
//     },
//   )
//   .toString();

// export const identity = getIdentityFromPhrase(seedPhrase);

interface ThisArgv {
  [x: string]: unknown;
  infra: boolean | undefined;
  apps: boolean | undefined;
  clean: boolean | undefined;
  create: boolean | undefined;
  install: boolean | undefined;
  reinstall: boolean | undefined;
  upgrade: boolean | undefined;
  project: string | undefined;
  remove: string | undefined;
  _: (string | number)[];
  $0: string;
}

const argv = yargs
  .option('clean', {
    alias: 'c',
    description: 'clean .dfx/ folder',
    type: 'boolean',
  })
  .option('create', {
    alias: 'n',
    description: 'create only',
    type: 'boolean',
  })
  .option('install', {
    alias: 'i',
    description: 'install only',
    type: 'boolean',
  })
  .option('reinstall', {
    alias: 'r',
    description: 'reinstall only',
    type: 'boolean',
  })
  .option('upgrade', {
    alias: 'u',
    description: 'upgrade only',
    type: 'boolean',
  })
  .option('project', {
    alias: 'p',
    description: 'upgrade only',
    type: 'string',
  })
  .help()
  .alias('help', 'h').argv;

function getEgos(): Configs {
  let egos: Configs = [];
  if ((argv as ThisArgv).infra) {
    egos = infraConfig;
  }
  if ((argv as ThisArgv).apps) {
    egos = appsConfig;
  }
  if ((argv as ThisArgv).project) {
    const project = (argv as ThisArgv).project;
    const ego = [...infraConfig, ...appsConfig].find(
      e => e.package === project,
    );
    if (ego) {
      if (
        (argv as ThisArgv).install ||
        (argv as ThisArgv).reinstall ||
        (argv as ThisArgv).upgrade
      ) {
        egos = [{ ...ego, no_deploy: false }];
      } else {
        egos = [ego];
      }
    }
  }

  if (
    !(argv as ThisArgv).infra &&
    !(argv as ThisArgv).apps &&
    !(argv as ThisArgv).project
  ) {
    egos = [...infraConfig, ...appsConfig];
  }
  return egos;
}

function runClean() {
  for (const f of getEgos()) {
    const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;
    // const dfx_sh = dfx_folder + '/dfx.sh';
    shell.exec(`rm -rf ${dfx_folder}`);
  }
}
function checkAndArtifacts() {
  for (const ego of getEgos()) {
    let folder_exist = true;
    try {
      folder_exist = file.existsSync(
        `${process.cwd()}/artifacts/${ego.package}`,
      );
    } catch (error) {
      folder_exist = false;
    }

    if (!folder_exist) {
      shell.exec(`mkdir ${process.cwd()}/artifacts/${ego.package}`);
    }
  }
}

function generateDFXJson() {
  for (const ego of getEgos()) {
    let shouldSaveName = `${process.cwd()}/artifacts/${ego.package}/dfx.json`;
    shell.exec(`rm -rf ${shouldSaveName}`);
    const packageItem = {};

    packageItem[ego.package] = {
      type: 'custom',
      candid: `${ego.package}.did`,
      wasm: `${ego.package}_opt.wasm`,
      build: [],
    };
    // dfxConfigTemplate.canisters
    dfxConfigTemplate['canisters'] = packageItem;
    // const target = Object.assign(dfxConfigTemplate.canisters, packageItem);
    file.writeFileSync(shouldSaveName, JSON.stringify(dfxConfigTemplate));
  }
}

async function runCreate() {
  const { actor } = await managementActor();
  const walletActor = (await cycleWalletActor()).actor;

  for (const f of getEgos()) {
    const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;

    if (!f.no_deploy) {
      let canister_id;
      if (!isProduction) {
        canister_id = (
          await actor.provisional_create_canister_with_cycles({
            settings: [
              {
                freezing_threshold: [],
                controllers: [[identity.getPrincipal()]],
                memory_allocation: [],
                compute_allocation: [],
              },
            ],
            amount: [],
          })
        ).canister_id;
      } else {
        const walletCreateResult = await walletActor.wallet_create_canister({
          cycles: BigInt(200_000_000_000),
          settings: {
            freezing_threshold: [],
            controller: [],
            controllers: [
              [
                identity.getPrincipal(),
                Principal.fromText(cycleWalletCanisterId),
              ],
            ],
            memory_allocation: [],
            compute_allocation: [],
          },
        });
        if (hasOwnProperty(walletCreateResult, 'Ok')) {
          canister_id = walletCreateResult.Ok.canister_id;
        } else {
          throw new Error(
            `canister id create failed : ${walletCreateResult.Err}`,
          );
        }
        // canister_id =
      }

      if (!isProduction) {
        const localCanisterId = canister_id.toText();
        console.log(`Creating canister ${f.package}...`);
        console.log(
          `${f.package} canister created with canister id: ${localCanisterId}`,
        );

        let configJson = JSON.stringify({});
        try {
          configJson = file.readFileSync(f.config).toString('utf8');
        } catch (error) {
          file.writeFileSync(f.config, JSON.stringify({}));
        }

        const configObject = {
          ...JSON.parse(configJson),
          LOCAL_CANISTERID: localCanisterId,
        };

        if (f.url) {
          Object.assign(configObject, {
            LOCAL_URL: `http://${localCanisterId}.localhost:8000`,
          });
        }

        file.writeFileSync(f.config, JSON.stringify(configObject));
      } else {
        const productionId = canister_id.toText();
        console.log(`Creating canister ${f.package}...`);
        console.log(
          `${f.package} canister created with canister id: ${productionId}`,
        );

        let configJson = JSON.stringify({});
        try {
          configJson = file.readFileSync(f.config).toString('utf8');
        } catch (error) {
          file.writeFileSync(f.config, JSON.stringify({}));
        }

        const configObject = {
          ...JSON.parse(configJson),
          PRODUCTION_CANISTERID: productionId,
        };

        if (f.url) {
          Object.assign(configObject, {
            PRODUCTION_URL: `https://${productionId}.ic0.app`,
          });
        }

        const canister_ids_json = {};
        canister_ids_json[`${f.package}`] = { ic: productionId };

        file.writeFileSync(f.config, JSON.stringify(configObject));
        file.writeFileSync(
          `./artifacts/${f.package}/canister_ids.json`,
          JSON.stringify(canister_ids_json),
        );
      }
    }
  }
}

async function runInstall() {
  const { actor } = await managementActor();

  for (const f of getEgos()) {
    const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;
    // const dfx_sh = dfx_folder + '/dfx.sh';
    if (!f.no_deploy) {
      if (f.custom_deploy) {
        if (typeof f.custom_deploy === 'string') {
          shell.exec(`cd ${dfx_folder} && ${f.custom_deploy}`);
        } else {
          (f.custom_deploy as () => void)();
        }
      } else {
        const pkg = readEgoDfxJson(dfx_folder, f.package);
        const wasm = readWasm(dfx_folder + '/' + pkg.wasm);
        const config = readConfig(
          process.cwd() + '/configs/' + f.package + '.json',
        );

        if (!isProduction) {
          try {
            console.log(
              `installing ${f.package} to ${config.LOCAL_CANISTERID!}`,
            );
            const initArgs = Array.from(
              new Uint8Array(
                IDL.encode(
                  [IDL.Record({ init_caller: IDL.Opt(IDL.Principal) })],
                  [
                    {
                      init_caller: [identity.getPrincipal()],
                    },
                  ],
                ),
              ),
            );
            await actor.install_code({
              arg: initArgs,
              wasm_module: wasm,
              mode: { install: null },
              canister_id: Principal.fromText(config.LOCAL_CANISTERID!),
            });
            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        } else {
          try {
            console.log(
              `installing ${f.package} to ${config.PRODUCTION_CANISTERID!}`,
            );
            const walletActor = (await cycleWalletActor()).actor;
            const wasm_module = IDL.Vec(IDL.Nat8);
            const idl = IDL.Record({
              arg: IDL.Vec(IDL.Nat8),
              wasm_module: wasm_module,
              mode: IDL.Variant({
                reinstall: IDL.Null,
                upgrade: IDL.Null,
                install: IDL.Null,
              }),
              canister_id: IDL.Principal,
            });

            // IDL.Tuple()
            const initArgs = Array.from(
              new Uint8Array(
                IDL.encode(
                  [IDL.Record({ init_caller: IDL.Opt(IDL.Principal) })],
                  [
                    {
                      init_caller: [identity.getPrincipal()],
                    },
                  ],
                ),
              ),
            );

            const buf = IDL.encode(
              [idl],
              [
                {
                  arg: initArgs,
                  wasm_module: wasm,
                  mode: { install: null },
                  canister_id: Principal.fromText(
                    config.PRODUCTION_CANISTERID!,
                  ),
                },
              ],
            );
            const args = Array.from(new Uint8Array(buf));

            const result = await walletActor.wallet_call({
              canister: Principal.fromHex(''),
              cycles: BigInt(200_000_000_000),
              method_name: 'install_code',
              args,
            });

            if (hasOwnProperty(result, 'Ok')) {
              console.log(result.Ok.return);
            } else {
              throw new Error(result.Err);
            }

            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        }
      }
    }
  }
}

async function runReInstall() {
  const { actor } = await managementActor();

  for (const f of getEgos()) {
    const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;
    // const dfx_sh = dfx_folder + '/dfx.sh';
    if (!f.no_deploy) {
      if (f.custom_deploy) {
        if (typeof f.custom_deploy === 'string') {
          shell.exec(`cd ${dfx_folder} && ${f.custom_deploy}`);
        } else {
          (f.custom_deploy as () => void)();
        }
      } else {
        const pkg = readEgoDfxJson(dfx_folder, f.package);
        const wasm = readWasm(dfx_folder + '/' + pkg.wasm);
        const config = readConfig(
          process.cwd() + '/configs/' + f.package + '.json',
        );

        if (!isProduction) {
          try {
            console.log(
              `reinstalling ${f.package} to ${config.LOCAL_CANISTERID!}`,
            );
            await actor.install_code({
              arg: [],
              wasm_module: wasm,
              mode: { reinstall: null },
              canister_id: Principal.fromText(config.LOCAL_CANISTERID!),
            });

            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        } else {
          try {
            console.log(
              `reinstalling ${f.package} to ${config.PRODUCTION_CANISTERID!}`,
            );
            const walletActor = (await cycleWalletActor()).actor;
            const wasm_module = IDL.Vec(IDL.Nat8);
            const idl = IDL.Record({
              arg: IDL.Vec(IDL.Nat8),
              wasm_module: wasm_module,
              mode: IDL.Variant({
                reinstall: IDL.Null,
                upgrade: IDL.Null,
                install: IDL.Null,
              }),
              canister_id: IDL.Principal,
            });

            // IDL.Tuple()

            const initArgs = Array.from(
              new Uint8Array(
                IDL.encode(
                  [IDL.Record({ init_caller: IDL.Opt(IDL.Principal) })],
                  [
                    {
                      init_caller: [identity.getPrincipal()],
                    },
                  ],
                ),
              ),
            );

            const buf = IDL.encode(
              [idl],
              [
                {
                  arg: initArgs,
                  wasm_module: wasm,
                  mode: { reinstall: null },
                  canister_id: Principal.fromText(
                    config.PRODUCTION_CANISTERID!,
                  ),
                },
              ],
            );
            const args = Array.from(new Uint8Array(buf));

            const result = await walletActor.wallet_call({
              canister: Principal.fromHex(''),
              cycles: BigInt(200_000_000_000),
              method_name: 'install_code',
              args,
            });

            if (hasOwnProperty(result, 'Ok')) {
              console.log(result.Ok.return);
            } else {
              throw new Error(result.Err);
            }
            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        }
      }
    }
  }
}

async function runUpgrade() {
  const { actor } = await managementActor();

  for (const f of getEgos()) {
    const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;
    // const dfx_sh = dfx_folder + '/dfx.sh';
    if (!f.no_deploy) {
      if (f.custom_deploy) {
        if (typeof f.custom_deploy === 'string') {
          shell.exec(`cd ${dfx_folder} && ${f.custom_deploy}`);
        } else {
          (f.custom_deploy as () => void)();
        }
      } else {
        const pkg = readEgoDfxJson(dfx_folder, f.package);
        const wasm = readWasm(dfx_folder + '/' + pkg.wasm);
        const config = readConfig(
          process.cwd() + '/configs/' + f.package + '.json',
        );
        if (!isProduction) {
          try {
            console.log(
              `upgrading ${f.package} to ${config.LOCAL_CANISTERID!}`,
            );
            await actor.install_code({
              arg: [],
              wasm_module: wasm,
              mode: { upgrade: null },
              canister_id: Principal.fromText(config.LOCAL_CANISTERID!),
            });
            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        } else {
          try {
            console.log(
              `upgrading ${f.package} to ${config.PRODUCTION_CANISTERID!}`,
            );
            const walletActor = (await cycleWalletActor()).actor;
            const wasm_module = IDL.Vec(IDL.Nat8);
            const idl = IDL.Record({
              arg: IDL.Vec(IDL.Nat8),
              wasm_module: wasm_module,
              mode: IDL.Variant({
                reinstall: IDL.Null,
                upgrade: IDL.Null,
                install: IDL.Null,
              }),
              canister_id: IDL.Principal,
            });

            // IDL.Tuple()
            const initArgs = Array.from(
              new Uint8Array(
                IDL.encode(
                  [IDL.Record({ init_caller: IDL.Opt(IDL.Principal) })],
                  [
                    {
                      init_caller: [identity.getPrincipal()],
                    },
                  ],
                ),
              ),
            );

            const buf = IDL.encode(
              [idl],
              [
                {
                  arg: initArgs,
                  wasm_module: wasm,
                  mode: { upgrade: null },
                  canister_id: Principal.fromText(
                    config.PRODUCTION_CANISTERID!,
                  ),
                },
              ],
            );
            const args = Array.from(new Uint8Array(buf));

            const result = await walletActor.wallet_call({
              canister: Principal.fromHex(''),
              cycles: BigInt(200_000_000_000),
              method_name: 'install_code',
              args,
            });

            if (hasOwnProperty(result, 'Ok')) {
              console.log(result.Ok.return);
            } else {
              throw new Error(result.Err);
            }
            console.log(`Success with wasm bytes length: ${wasm.length}`);
          } catch (error) {
            console.log((error as Error).message);
          }
        }
      }
    }
  }
}

// async function runRemove() {
//   const { actor } = await managementActor();
//   const walletActor = (await cycleWalletActor()).actor;
//   for (const f of getEgos()) {
//     const dfx_folder = process.cwd() + '/' + 'artifacts' + '/' + f.package;
//     // const dfx_sh = dfx_folder + '/dfx.sh';
//     if (!f.no_deploy) {
//       if (f.custom_deploy) {
//         if (typeof f.custom_deploy === 'string') {
//           shell.exec(`cd ${dfx_folder} && ${f.custom_deploy}`);
//         } else {
//           (f.custom_deploy as () => void)();
//         }
//       } else {
//         const pkg = readEgoDfxJson(dfx_folder, f.package);
//         const wasm = readWasm(dfx_folder + '/' + pkg.wasm);
//         const config = readConfig(
//           process.cwd() + '/configs/' + f.package + '.json',
//         );
//         if (!isProduction) {
//           try {
//             console.log(
//               `upgrading ${f.package} to ${config.LOCAL_CANISTERID!}`,
//             );
//             await actor.stop_canister({
//               canister_id: Principal.fromText(config.LOCAL_CANISTERID!),
//             });
//             await actor.delete_canister({
//               canister_id: Principal.fromText(config.LOCAL_CANISTERID!),
//             });
//             console.log(`Success with wasm bytes length: ${wasm.length}`);
//           } catch (error) {
//             console.log((error as Error).message);
//           }
//         } else {
//           try {
//             console.log(
//               `upgrading ${f.package} to ${config.PRODUCTION_CANISTERID!}`,
//             );
//             const wasm_module = IDL.Vec(IDL.Nat8);
//             const idl = IDL.Record({
//               arg: IDL.Vec(IDL.Nat8),
//               wasm_module: wasm_module,
//               mode: IDL.Variant({
//                 reinstall: IDL.Null,
//                 upgrade: IDL.Null,
//                 install: IDL.Null,
//               }),
//               canister_id: IDL.Principal,
//             });

//             // IDL.Tuple()
//             const initArgs = Array.from(
//               new Uint8Array(
//                 IDL.encode(
//                   [IDL.Opt(IDL.Principal)],
//                   [[identity.getPrincipal()]],
//                 ),
//               ),
//             );

//             const buf = IDL.encode(
//               [idl],
//               [
//                 {
//                   arg: initArgs,
//                   wasm_module: wasm,
//                   mode: { upgrade: null },
//                   canister_id: Principal.fromText(
//                     config.PRODUCTION_CANISTERID!,
//                   ),
//                 },
//               ],
//             );
//             const args = Array.from(new Uint8Array(buf));

//             const result = await walletActor.wallet_call({
//               canister: Principal.fromHex(''),
//               cycles: BigInt(0),
//               method_name: 'install_code',
//               args,
//             });

//             if (hasOwnProperty(result, 'Ok')) {
//               console.log(result.Ok.return);
//             } else {
//               throw new Error(result.Err);
//             }
//             console.log(`Success with wasm bytes length: ${wasm.length}`);
//           } catch (error) {
//             console.log((error as Error).message);
//           }
//         }
//       }
//     }
//   }
// }

checkAndArtifacts();
generateDFXJson();

if ((argv as ThisArgv).clean) {
  // console.log('clean');
  runClean();
}

if ((argv as ThisArgv).create) {
  // console.log('create');
  runCreate();
}

if ((argv as ThisArgv).install) {
  // console.log('install');
  runInstall();
}

if ((argv as ThisArgv).reinstall) {
  // console.log('reinstall');
  runReInstall();
}

if ((argv as ThisArgv).upgrade) {
  // console.log('upgrade');
  runUpgrade();
}
