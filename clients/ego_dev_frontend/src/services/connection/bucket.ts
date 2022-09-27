// import { AppCreateRequest, AppFilesRequest, AppInfoRequest, AppNewVersionRequest, AppOperatorsResponse, LoadFileRequest, Permission, Result_1, Result_11, Result_13, Result_2, Result_3, Result_4, Result_5, Result_6, Result_7, Result_8, Result_9, UploadFileRequest, _SERVICE } from "@/../../idls/ego_bucket";
// import { idlFactory as bucketIdl } from "@/../../idls/ego_bucket.idl";
// import { BaseConnection } from "./base";
// import { ActorSubclass, HttpAgent, Identity, SignIdentity } from "@dfinity/agent";
// import { getActor } from "./base";
// import { Principal } from "@dfinity/principal";
//
//
//
// export class BucketConnection extends BaseConnection<_SERVICE> {
//   constructor(
//     public identity: Identity,
//     public actor: ActorSubclass<_SERVICE>,
//     public agent: HttpAgent,
//   ) {
//     super(identity, process.env.EGO_BUCKET_CANISTERID!, bucketIdl, actor, agent);
//   }
//   static async create(identity: Identity): Promise<BucketConnection> {
//     const { actor, agent } = await getActor<_SERVICE>(bucketIdl, process.env.EGO_BUCKET_CANISTERID!, identity);
//     return new BucketConnection(identity, actor, agent);
//   }
//
//   async add_managers(
//     managers: Principal[],
//   ) {
//     const result = await this._actor.add_managers({ managers });
//     return result;
//   }
//
//   async app_create({
//     appid,
//     caller,
//   }: AppCreateRequest ): Promise<Result_1> {
//     const result = await this._actor.app_create({ appid, caller });
//     return result;
//   }
//
//   // async app_files({ appid, release }: AppFilesRequest): Promise<Result_2> {
//   //   const result = await this._actor.app_files({ appid, release });
//   //   return result;
//   // }
//
//   async app_info({
//     appid
//   }:AppInfoRequest): Promise<Result_3> {
//     const result = await this._actor.app_info({ appid });
//     return result;
//   }
//
//   async app_list(): Promise<Result_4> {
//     const result = await this._actor.app_list();
//     return result;
//   }
//
//   async app_new_version({ appid, fids, version }: AppNewVersionRequest ): Promise<Result_5> {
//     const result = await this._actor.app_new_version({ appid, fids, version });
//     return result;
//   }
//
//
//   async app_operators({appid}: AppInfoRequest): Promise<Result_6> {
//     const result = await this._actor.app_operators({ appid });
//     return result;
//   }
//
//   async app_remove({appid}: AppInfoRequest): Promise<Result_7> {
//     const result = await this._actor.app_remove({ appid });
//     return result;
//   }
//
//   async app_set_operators({ appid, operators }: AppOperatorsResponse): Promise<Result_7> {
//     const result = await this._actor.app_set_operators({ appid, operators });
//     return result;
//   }
//
//   // async app_set_stable({ appid, version }: AppSetMainRequest): Promise<Result_5> {
//   //   const result = await this._actor.app_set_stable({ appid, version });
//   //   return result;
//   // }
//
//   // async file_info({ fid}: FileInfoRequest): Promise<Result_8> {
//   //   const result = await this._actor.file_info({ fid });
//   //   return result;
//   // }
//
//   // async load_file({ appid, release, fid }: LoadFileRequest): Promise<Result_9> {
//   //   const result = await this._actor.load_file({ appid, release, fid });
//   //   return result;
//   // }
//
//   async upload_file({ fid, appid, data, hash, version }: UploadFileRequest): Promise<Result_13> {
//     const result = await this._actor.upload_file({ fid, appid, data, hash, version });
//     return result;
//   }
// }