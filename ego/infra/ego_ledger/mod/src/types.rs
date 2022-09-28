use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_ledger_types::{AccountIdentifier, BlockIndex, Memo, Tokens};
use ic_cdk::export::Principal;
use ego_types::ego_error::EgoError;



#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoLedgerErr {
    FailedAddPayment,
    FailedQueryBlocks,
    FailMatchedBlocks,
    FailNotifyPayment,
    LedgerError(String)
}


impl From<EgoLedgerErr> for EgoError{
    fn from(e:EgoLedgerErr) -> Self {
        match e {
            EgoLedgerErr::FailedAddPayment => EgoError::new(6001,"ego-ledger: fail to add payment"),
            EgoLedgerErr::FailedQueryBlocks => EgoError::new(6002,"ego-ledger: fail to query blocks"),
            EgoLedgerErr::FailMatchedBlocks => EgoError::new(6003,"ego-ledger: fail to match blocks"),
            EgoLedgerErr::FailNotifyPayment => EgoError::new(6006,"ego-ledger: fail to notify payment"),
            EgoLedgerErr::LedgerError(msg) => msg.into()
        }
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct LedgerMainInitRequest {
    pub start: BlockIndex
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct LedgerPaymentAddRequest {
    pub from: AccountIdentifier,
    pub to: AccountIdentifier,
    pub amount: Tokens,
    pub memo: Memo
}


#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminEgoStoreSetRequest {
    pub canister_id: Principal,
}