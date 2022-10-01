import { CreateActorResult, getActor, getActor2 } from './settings/agent';
import {
  canister_settings,
  _SERVICE as ManagementService,
} from './idls/management';
import path from 'path';

import { _SERVICE as CycleWalletService } from './idls/cycle_wallet';
import { idlFactory as managementIdl } from './idls/management.idl';
import { idlFactory as cycleWalletIdl } from './idls/cycle_wallet.idl';
import { ActorSubclass } from '@dfinity/agent';
import fs from 'fs';
import { Principal } from '@dfinity/principal';
import { identity } from './settings/identity';
const managementCanisterId = '';
export const cycleWalletCanisterId = fs
  .readFileSync(
    path.join(process.cwd(), '/credentials', '/production_cycle_wallet.txt'),
    {
      encoding: 'utf8',
    },
  )
  .toString();

export async function managementActor(): Promise<
  CreateActorResult<ManagementService>
> {
  return await getActor2<ManagementService>(
    identity,
    managementIdl,
    managementCanisterId,
  );
}

export async function cycleWalletActor(): Promise<
  CreateActorResult<CycleWalletService>
> {
  return await getActor2<CycleWalletService>(
    identity,
    cycleWalletIdl,
    cycleWalletCanisterId,
  );
}

export function readWasm(packagePath: string): number[] {
  return Array.from(new Uint8Array(fs.readFileSync(packagePath)));
}

export interface EGOPackage {
  type: string;
  candid: string;
  wasm: string;
  build: string[];
}

export interface ConfigFile {
  LOCAL_CANISTERID?: string;
  PRODUCTION_CANISTERID?: string;
}

export function readEgoDfxJson(
  folder: string,
  packageName: string,
): EGOPackage {
  const dfxFile = fs.readFileSync(folder + '/dfx.json').toString();
  const jsonFile = JSON.parse(dfxFile);
  const pkg: EGOPackage = jsonFile['canisters'][packageName];
  return pkg;
}

export function readConfig(configPath: string): ConfigFile {
  return JSON.parse(fs.readFileSync(configPath).toString()) as ConfigFile;
}

export enum InstallMode {
  install,
  reinstall,
  upgrade,
}

export class ManagementApi {
  get actor() {
    return this._actor;
  }
  constructor(private _actor: ActorSubclass<ManagementService>) {}
  static async create() {
    const { actor } = await managementActor();
    return new ManagementApi(actor);
  }

  static async install({
    name,
    wasm_path,
    canister_id,
    installMode = InstallMode.install,
  }: {
    name: string;
    wasm_path: string;
    canister_id: string;
    installMode: InstallMode;
  }) {
    const manager = await ManagementApi.create();
    console.log(`installing ${name} to ${canister_id}`);
    let mode: { install: null } | { reinstall: null } | { upgrade: null };
    switch (installMode) {
      case InstallMode.install:
        mode = { install: null };
        break;
      case InstallMode.reinstall:
        mode = { reinstall: null };
        break;
      case InstallMode.upgrade:
        mode = { upgrade: null };
        break;
      default:
        mode = { install: null };
        break;
    }
    try {
      const wasm = readWasm(wasm_path);
      await manager.actor.install_code({
        arg: [],
        wasm_module: readWasm(wasm_path),
        mode,
        canister_id: Principal.fromText(canister_id),
      });
      console.log(`Success with wasm bytes length: ${wasm.length}`);
    } catch (error) {
      throw new Error(`
      Error: Failed to install ${name} to ${canister_id}
      Reason: ${(error as Error).message}
      `);
    }
  }

  static async updateSettings(
    name: string,
    canister_id: string,
    settings: canister_settings,
  ) {
    const manager = await ManagementApi.create();
    console.log(`update settings ${name} : ${canister_id}`);

    await manager.actor.update_settings({
      canister_id: Principal.fromText(canister_id),
      settings,
    });
  }
}
