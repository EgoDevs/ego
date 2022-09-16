import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type canister_id = Principal;
export interface canister_settings {
  freezing_threshold: [] | [bigint];
  controllers: [] | [Array<Principal>];
  memory_allocation: [] | [bigint];
  compute_allocation: [] | [bigint];
}
export interface definite_canister_settings {
  freezing_threshold: bigint;
  controllers: Array<Principal>;
  memory_allocation: bigint;
  compute_allocation: bigint;
}
export interface http_header {
  value: string;
  name: string;
}
export type http_request_error =
  | { dns_error: null }
  | { no_consensus: null }
  | { transform_error: null }
  | { unreachable: null }
  | { bad_tls: null }
  | { conn_timeout: null }
  | { invalid_url: null }
  | { timeout: null };
export interface http_response {
  status: bigint;
  body: Array<number>;
  headers: Array<http_header>;
}
export type user_id = Principal;
export type wasm_module = Array<number>;
export interface _SERVICE {
  canister_status: ActorMethod<
    [{ canister_id: canister_id }],
    {
      status: { stopped: null } | { stopping: null } | { running: null };
      memory_size: bigint;
      cycles: bigint;
      settings: definite_canister_settings;
      module_hash: [] | [Array<number>];
    }
  >;
  create_canister: ActorMethod<
    [{ settings: [] | [canister_settings] }],
    { canister_id: canister_id }
  >;
  delete_canister: ActorMethod<[{ canister_id: canister_id }], undefined>;
  deposit_cycles: ActorMethod<[{ canister_id: canister_id }], undefined>;
  http_request: ActorMethod<
    [
      {
        url: string;
        method: { get: null };
        body: [] | [Array<number>];
        transform: [] | [{ function: [Principal, string] }];
        headers: Array<http_header>;
      },
    ],
    { Ok: http_response } | { Err: [] | [http_request_error] }
  >;
  install_code: ActorMethod<
    [
      {
        arg: Array<number>;
        wasm_module: wasm_module;
        mode: { reinstall: null } | { upgrade: null } | { install: null };
        canister_id: canister_id;
      },
    ],
    undefined
  >;
  provisional_create_canister_with_cycles: ActorMethod<
    [{ settings: [] | [canister_settings]; amount: [] | [bigint] }],
    { canister_id: canister_id }
  >;
  provisional_top_up_canister: ActorMethod<
    [{ canister_id: canister_id; amount: bigint }],
    undefined
  >;
  raw_rand: ActorMethod<[], Array<number>>;
  start_canister: ActorMethod<[{ canister_id: canister_id }], undefined>;
  stop_canister: ActorMethod<[{ canister_id: canister_id }], undefined>;
  uninstall_code: ActorMethod<[{ canister_id: canister_id }], undefined>;
  update_settings: ActorMethod<
    [{ canister_id: Principal; settings: canister_settings }],
    undefined
  >;
}
