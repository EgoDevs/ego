use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use std::str::FromStr;

#[derive(
    CandidType, Serialize, Deserialize, Clone, Copy, Debug, Default, Ord, PartialOrd, Eq, PartialEq,
)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl Version {
    pub fn new(major: u32, minor: u32, patch: u32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }

    pub fn min() -> Version {
        Version {
            major: 0,
            minor: 0,
            patch: 0,
        }
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(format!("Unable to parse version: {}", s));
        }

        let major = u32::from_str(parts[0]).map_err(|e| e.to_string())?;
        let minor = u32::from_str(parts[1]).map_err(|e| e.to_string())?;
        let patch = u32::from_str(parts[2]).map_err(|e| e.to_string())?;

        Ok(Version {
            major,
            minor,
            patch,
        })
    }
}

impl ToString for Version {
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}
