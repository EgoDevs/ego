use candid::{CandidType, Deserialize};

use ego_types::app::EgoError;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub enum EgoFileError {
  FidNotFound,
  FileExists,
  InvalidFileHash,
  CannotModifyFile,
  StorageFull,
  PermissionDenied,
  FileTooLarge,
  SystemError,
  UnknownError(String),
}

impl From<EgoFileError> for EgoError {
  fn from(e: EgoFileError) -> Self {
    match e {
      EgoFileError::FidNotFound => EgoError::new(5001, "ego_file: file not found"),
      EgoFileError::FileExists => EgoError::new(5002, "ego_file: file exists"),
      EgoFileError::InvalidFileHash => EgoError::new(5003, "ego_file: invalid file hash"),
      EgoFileError::CannotModifyFile => EgoError::new(5004, "ego_file: cannot modify"),
      EgoFileError::StorageFull => EgoError::new(5005, "ego_file: storage full"),
      EgoFileError::PermissionDenied => EgoError::new(5006, "ego_file: permission denied"),
      EgoFileError::FileTooLarge => EgoError::new(5007, "ego_file: file size exceeds 2MB"),
      EgoFileError::SystemError => EgoError::new(5008, "ego_file: system error"),
      EgoFileError::UnknownError(msg) => msg.into(),
    }
  }
}
