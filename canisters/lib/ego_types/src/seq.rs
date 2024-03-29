use std::collections::HashMap;

use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Seq {
  seqs: HashMap<String, u64>,
}

impl Default for Seq {
  fn default() -> Self {
    Seq {
      seqs: Default::default(),
    }
  }
}

impl Seq {
  pub fn next_number(&mut self, key: &str, current_max: u64) -> u64 {
    let mut number = 1;
    if self.seqs.contains_key(key) {
      number = self.seqs.get(key).unwrap().clone();

      if number >= current_max {
        number += 1;
      } else {
        number = current_max + 1
      }
    }

    self.seqs.insert(key.to_string(), number);

    number
  }

  pub fn set_number(&mut self, key: &str, number: u64) {
    self.seqs.insert(key.to_string(), number);
  }

  pub fn get_number(&mut self, key: &str) -> Option<u64> {
    self.seqs.get(key).cloned()
  }
}