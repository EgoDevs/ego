use ego_record_mod::record::Record;
use ego_record_mod::service::RecordService;
use ego_record_mod::state::EGO_RECORD;

fn set_up() {
  EGO_RECORD.with(|ego_record| {
    ego_record.borrow_mut().records.push(Record {
      id: 1,
      scope: "S1".to_string(),
      event: "E1".to_string(),
      message: "M1".to_string(),
      create_at: 10,
    });

    ego_record.borrow_mut().records.push(Record {
      id: 2,
      scope: "S2".to_string(),
      event: "E2".to_string(),
      message: "M2".to_string(),
      create_at: 20,
    });

    ego_record.borrow_mut().records.push(Record {
      id: 3,
      scope: "S3".to_string(),
      event: "E3".to_string(),
      message: "M3".to_string(),
      create_at: 30,
    });

    ego_record.borrow_mut().record_id = 3;
  });
}

#[test]
pub fn record_add() {
  set_up();

  assert_eq!(3, RecordService::record_amount());

  RecordService::record_add("S4", "E4", "M4", 40);

  assert_eq!(4, RecordService::record_amount());
}

#[test]
pub fn record_list() {
  set_up();

  let records = RecordService::record_list(3);

  assert_eq!(3, records.len());

  let record = records.get(0).unwrap();

  assert_eq!("S3", record.scope);
}

#[test]
pub fn record_retain() {
  set_up();

  RecordService::record_retain(1);

  let records = RecordService::record_list(3);

  assert_eq!(1, records.len());

  let record = records.get(0).unwrap();

  assert_eq!("S3", record.scope);
}

#[test]
pub fn record_retain_after() {
  set_up();

  RecordService::record_retain_after(15);

  let records = RecordService::record_list(3);

  assert_eq!(2, records.len());

  let record = records.get(0).unwrap();

  assert_eq!("S3", record.scope);
}
