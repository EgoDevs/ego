use ic_cdk::export::candid::CandidType;
use ic_cdk::export::candid::Deserialize;
use serde::{Serialize};
use std::collections::HashMap;

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
    pub fn next_number(&mut self, key: &str, current_max: u64) -> u64{
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
}