use crate::record::Record;

use crate::state::{EGO_RECORD};

pub struct RecordService {
    pub records: Vec<Record>,
    pub record_id: u64,
    pub record_st: u64,
}


impl RecordService {
    pub fn record_add(scope: &str, event: &str, message: &str, ts: u64) {
        EGO_RECORD.with(|ego_record| {
            ego_record.borrow_mut().record_add(scope, event, message, ts);
        });
    }

    pub fn record_amount() -> usize {
        EGO_RECORD.with(|ego_record| {
            ego_record.borrow().records.len()
        })
    }

    pub fn record_list(start: usize, mut end: usize) -> Vec<Record> {
        let total_amount = EGO_RECORD.with(|ego_record| {
            ego_record.borrow().records.len()
        });

        if end > total_amount {
            end = total_amount;
        }

        let records = EGO_RECORD.with(|ego_record| {
            ego_record.borrow().records[start..end].to_vec()
        });
        // records.reverse();
        records
    }

    pub fn record_retain(remain_amount: usize) {
        let total_amount = EGO_RECORD.with(|ego_record| {
            ego_record.borrow().records.len()
        });

        let mut end = 0;
        if remain_amount < total_amount {
            end = total_amount - remain_amount;
        }

        EGO_RECORD.with(|ego_record| {
            ego_record.borrow_mut().records.drain(0..end);
        });
    }

    pub fn record_retain_after(end_time: u64) {
        EGO_RECORD.with(|ego_record| {
            ego_record.borrow_mut().records.retain(|r| r.create_at > end_time)
        });
    }
}