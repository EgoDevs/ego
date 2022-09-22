use ic_ledger_types::{AccountIdentifier, BlockIndex, GetBlocksArgs, MAINNET_LEDGER_CANISTER_ID, Memo, Operation, query_blocks, Tokens, Transaction};
use ego_types::ego_error::EgoError;
use crate::c2c::ego_store::{EgoStore, TEgoStore};
use crate::payment::Payment;
use crate::state::{EGO_LEDGER, EGO_STORE_CANISTER_ID};
use crate::types::EgoLedgerErr;

pub struct EgoLedgerService {

}

impl EgoLedgerService {
    pub fn ledger_main_init(start: BlockIndex) {
        EGO_LEDGER.with(|ego_ledger| {
            ego_ledger.borrow_mut().ledger_main_init(start)
        })
    }

    pub fn ledger_payment_add(from: AccountIdentifier, to: AccountIdentifier, amount: Tokens, memo: Memo) {
        EGO_LEDGER.with(|ego_ledger| {
            let payment = Payment::new(from, to, amount, memo);
            ego_ledger.borrow_mut().ledger_payment_add(payment)
        })
    }

    pub async fn ledger_block_query() -> Result<(), EgoError> {
        ic_cdk::println!("==> 1.query blocks");
        let start = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().start);

        let blocks = match query_blocks(MAINNET_LEDGER_CANISTER_ID, GetBlocksArgs { start: start, length: 100 }).await {
            Ok(t) => {
                ic_cdk::println!("query block successful");
                Ok(t.blocks)
            }
            Err((code, detail)) => {
                ic_cdk::println!("query block failed with rejectionCode {:?} and detail {:?}", code, detail);
                Err(EgoError::from(EgoLedgerErr::FailedQueryBlocks))
            }
        }?;

        let length = blocks.len();

        ic_cdk::println!("==> 2.update length");
        EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow_mut().start += length as u64 );

        ic_cdk::println!("==> 3.add block to confirmed");
        EGO_LEDGER.with(|ego_ledger| {
            let mut e_l = ego_ledger.borrow_mut();
            for block in blocks {
                let trx: Transaction = block.clone().transaction;
                if trx.operation.is_some() {
                    match trx.operation.unwrap() {
                        Operation::Transfer { from, to, amount, fee: _ } => {
                            e_l.confirmed.entry(trx.memo).or_insert(Payment::new(from, to, amount, trx.memo));
                        }
                        _ => {}
                    }
                }
            }
        });

        ic_cdk::println!("==> 4.get matched memos");
        let matched_memos = EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow().ledger_payment_matched());

        ic_cdk::println!("==> 5.notify ego_store");
        let ego_store = EgoStore::new();
        let ego_store_canister_id = EGO_STORE_CANISTER_ID.with(|rc| rc.borrow().unwrap());

        let mut successes_memos = vec![];

        for matched_memo in matched_memos {
            ego_store.wallet_order_notify(ego_store_canister_id, matched_memo).await?;
            successes_memos.push(matched_memo);
        }

        ic_cdk::println!("==> 6.remove notify successes memos");
        EGO_LEDGER.with(|ego_ledger| ego_ledger.borrow_mut().ledger_payment_remove(successes_memos));

        Ok(())
    }
}