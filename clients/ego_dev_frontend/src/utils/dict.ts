import { AppVersionStatus } from "@/../../idls/ego_store";
import { AppVersionStatusEnum } from "./types";

export const getStatusByAppStatus  = (status: AppVersionStatus) => {
  let statusKey = '';
  for(const key in status) { 
    statusKey = key
  }
  switch (statusKey) { 
    case AppVersionStatusEnum.REJECTED:
      return 'REJECTED';
    case AppVersionStatusEnum.NEW:
      return 'NEW';
    case AppVersionStatusEnum.SUBMITTED:
      return 'SUBMITTED';
    case AppVersionStatusEnum.REVOKED:
      return 'REVOKED';
    case AppVersionStatusEnum.RELEASED:
      return 'RELEASED';
    case AppVersionStatusEnum.APPROVED:
      return 'APPROVED';
    default:
      return 'UNKNOWN';
  }

}