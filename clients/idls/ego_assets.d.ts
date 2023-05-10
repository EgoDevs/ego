import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface AssetDetails {
  'key' : string,
  'encodings' : Array<AssetEncodingDetails>,
  'content_type' : string,
}
export interface AssetEncodingDetails {
  'modified' : bigint,
  'sha256' : [] | [Array<number>],
  'length' : bigint,
  'content_encoding' : string,
}
export type BatchOperation = { 'CreateAsset' : CreateAssetArguments } |
  { 'UnsetAssetContent' : UnsetAssetContentArguments } |
  { 'DeleteAsset' : DeleteAssetArguments } |
  { 'SetAssetContent' : SetAssetContentArguments } |
  { 'Clear' : {} };
export interface CommitBatchArguments {
  'batch_id' : bigint,
  'operations' : Array<BatchOperation>,
}
export interface CreateAssetArguments {
  'key' : string,
  'content_type' : string,
  'headers' : [] | [Array<[string, string]>],
  'max_age' : [] | [bigint],
}
export interface CreateBatchResponse { 'batch_id' : bigint }
export interface CreateChunkArg {
  'content' : Array<number>,
  'batch_id' : bigint,
}
export interface CreateChunkResponse { 'chunk_id' : bigint }
export interface DeleteAssetArguments { 'key' : string }
export interface EncodedAsset {
  'content' : Array<number>,
  'sha256' : [] | [Array<number>],
  'content_type' : string,
  'content_encoding' : string,
  'total_length' : bigint,
}
export interface GetArg { 'key' : string, 'accept_encodings' : Array<string> }
export interface GetChunkArg {
  'key' : string,
  'sha256' : [] | [Array<number>],
  'index' : bigint,
  'content_encoding' : string,
}
export interface GetChunkResponse { 'content' : Array<number> }
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Array<number>,
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Array<number>,
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export interface InitArg { 'init_caller' : [] | [Principal] }
export type Result = { 'Ok' : Array<Principal> } |
  { 'Err' : string };
export interface SetAssetContentArguments {
  'key' : string,
  'sha256' : [] | [Array<number>],
  'chunk_ids' : Array<bigint>,
  'content_encoding' : string,
}
export interface StoreArg {
  'key' : string,
  'content' : Array<number>,
  'sha256' : [] | [Array<number>],
  'content_type' : string,
  'content_encoding' : string,
}
export interface StreamingCallbackHttpResponse {
  'token' : [] | [GetChunkArg],
  'body' : Array<number>,
}
export type StreamingStrategy = {
    'Callback' : { 'token' : GetChunkArg, 'callback' : [Principal, string] }
  };
export interface UnsetAssetContentArguments {
  'key' : string,
  'content_encoding' : string,
}
export interface _SERVICE {
  'authorize' : ActorMethod<[Principal], undefined>,
  'clear' : ActorMethod<[], undefined>,
  'commit_batch' : ActorMethod<[CommitBatchArguments], undefined>,
  'create_asset' : ActorMethod<[CreateAssetArguments], undefined>,
  'create_batch' : ActorMethod<[], CreateBatchResponse>,
  'create_chunk' : ActorMethod<[CreateChunkArg], CreateChunkResponse>,
  'delete_asset' : ActorMethod<[DeleteAssetArguments], undefined>,
  'drain_authorize' : ActorMethod<[], undefined>,
  'get' : ActorMethod<[GetArg], EncodedAsset>,
  'get_chunk' : ActorMethod<[GetChunkArg], GetChunkResponse>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'http_request_streaming_callback' : ActorMethod<
    [GetChunkArg],
    StreamingCallbackHttpResponse,
  >,
  'list' : ActorMethod<[], Array<AssetDetails>>,
  'list_authorize' : ActorMethod<[], Result>,
  'retrieve' : ActorMethod<[string], Array<number>>,
  'set_asset_content' : ActorMethod<[SetAssetContentArguments], undefined>,
  'store' : ActorMethod<[StoreArg], undefined>,
  'unset_asset_content' : ActorMethod<[UnsetAssetContentArguments], undefined>,
}
