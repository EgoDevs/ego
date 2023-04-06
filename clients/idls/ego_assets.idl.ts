export const idlFactory = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  const CreateAssetArguments = IDL.Record({
    'key' : IDL.Text,
    'content_type' : IDL.Text,
    'headers' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
    'max_age' : IDL.Opt(IDL.Nat64),
  });
  const UnsetAssetContentArguments = IDL.Record({
    'key' : IDL.Text,
    'content_encoding' : IDL.Text,
  });
  const DeleteAssetArguments = IDL.Record({ 'key' : IDL.Text });
  const SetAssetContentArguments = IDL.Record({
    'key' : IDL.Text,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'chunk_ids' : IDL.Vec(IDL.Nat),
    'content_encoding' : IDL.Text,
  });
  const BatchOperation = IDL.Variant({
    'CreateAsset' : CreateAssetArguments,
    'UnsetAssetContent' : UnsetAssetContentArguments,
    'DeleteAsset' : DeleteAssetArguments,
    'SetAssetContent' : SetAssetContentArguments,
    'Clear' : IDL.Record({}),
  });
  const CommitBatchArguments = IDL.Record({
    'batch_id' : IDL.Nat,
    'operations' : IDL.Vec(BatchOperation),
  });
  const CreateBatchResponse = IDL.Record({ 'batch_id' : IDL.Nat });
  const CreateChunkArg = IDL.Record({
    'content' : IDL.Vec(IDL.Nat8),
    'batch_id' : IDL.Nat,
  });
  const CreateChunkResponse = IDL.Record({ 'chunk_id' : IDL.Nat });
  const GetArg = IDL.Record({
    'key' : IDL.Text,
    'accept_encodings' : IDL.Vec(IDL.Text),
  });
  const EncodedAsset = IDL.Record({
    'content' : IDL.Vec(IDL.Nat8),
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'content_type' : IDL.Text,
    'content_encoding' : IDL.Text,
    'total_length' : IDL.Nat,
  });
  const GetChunkArg = IDL.Record({
    'key' : IDL.Text,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'index' : IDL.Nat,
    'content_encoding' : IDL.Text,
  });
  const GetChunkResponse = IDL.Record({ 'content' : IDL.Vec(IDL.Nat8) });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : GetChunkArg,
      'callback' : IDL.Func([], [], []),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const StreamingCallbackHttpResponse = IDL.Record({
    'token' : IDL.Opt(GetChunkArg),
    'body' : IDL.Vec(IDL.Nat8),
  });
  const AssetEncodingDetails = IDL.Record({
    'modified' : IDL.Int,
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'length' : IDL.Nat,
    'content_encoding' : IDL.Text,
  });
  const AssetDetails = IDL.Record({
    'key' : IDL.Text,
    'encodings' : IDL.Vec(AssetEncodingDetails),
    'content_type' : IDL.Text,
  });
  const Result = IDL.Variant({
    'Ok' : IDL.Vec(IDL.Principal),
    'Err' : IDL.Text,
  });
  const StoreArg = IDL.Record({
    'key' : IDL.Text,
    'content' : IDL.Vec(IDL.Nat8),
    'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'content_type' : IDL.Text,
    'content_encoding' : IDL.Text,
  });
  return IDL.Service({
    'authorize' : IDL.Func([IDL.Principal], [], []),
    'clear' : IDL.Func([], [], []),
    'commit_batch' : IDL.Func([CommitBatchArguments], [], []),
    'create_asset' : IDL.Func([CreateAssetArguments], [], []),
    'create_batch' : IDL.Func([], [CreateBatchResponse], []),
    'create_chunk' : IDL.Func([CreateChunkArg], [CreateChunkResponse], []),
    'delete_asset' : IDL.Func([DeleteAssetArguments], [], []),
    'drain_authorize' : IDL.Func([], [], []),
    'get' : IDL.Func([GetArg], [EncodedAsset], ['query']),
    'get_chunk' : IDL.Func([GetChunkArg], [GetChunkResponse], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'http_request_streaming_callback' : IDL.Func(
        [GetChunkArg],
        [StreamingCallbackHttpResponse],
        ['query'],
      ),
    'list' : IDL.Func([], [IDL.Vec(AssetDetails)], ['query']),
    'list_authorize' : IDL.Func([], [Result], ['query']),
    'retrieve' : IDL.Func([IDL.Text], [IDL.Vec(IDL.Nat8)], ['query']),
    'set_asset_content' : IDL.Func([SetAssetContentArguments], [], []),
    'store' : IDL.Func([StoreArg], [], []),
    'unset_asset_content' : IDL.Func([UnsetAssetContentArguments], [], []),
  });
};
export const init = ({ IDL }) => {
  const InitArg = IDL.Record({ 'init_caller' : IDL.Opt(IDL.Principal) });
  return [InitArg];
};
