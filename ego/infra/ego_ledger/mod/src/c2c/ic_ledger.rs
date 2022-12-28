use async_trait::async_trait;
use ic_cdk::export::Principal;
use ic_ledger_types::{Block, BlockIndex, GetBlocksArgs, query_blocks};

use ego_types::app::EgoError;

use crate::state::EGO_LEDGER;
use crate::types::EgoLedgerErr;

#[async_trait]
pub trait TIcLedger {
  async fn query_blocks(&self, start: BlockIndex) -> Result<Vec<Block>, EgoError>;
}

pub struct IcLedger {
  pub canister_id: Principal,
}

impl IcLedger {
  pub fn new(canister_id: Principal) -> Self {
    IcLedger { canister_id }
  }
}

#[async_trait]
impl TIcLedger for IcLedger {
  async fn query_blocks(&self, start: BlockIndex) -> Result<Vec<Block>, EgoError> {
    let blocks = match query_blocks(
      self.canister_id,
      GetBlocksArgs { start, length: 5 },
    ).await
    {
      Ok(t) => {
        ic_cdk::println!("==> query block success");
        Ok(t.blocks)
      }
      Err((code, detail)) => {
        ic_cdk::println!("==> query block failed with rejectionCode {:?} and detail {:?}", code, detail);
        Err(EgoError::from(EgoLedgerErr::FailedQueryBlocks))
      }
    }?;

    let length = blocks.len();
    EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow_mut().start += length as u64);

    Ok(blocks)
  }
}
