use ego_types::seq::Seq;

#[test]
fn not_exsits_test() {
  let mut seq = Seq::default();

  let num1 = seq.next_number("key1", 0);
  assert_eq!(num1, 1);

  let num2 = seq.next_number("key2", 0);
  assert_eq!(num2, 1);
}

#[test]
fn exsits_test() {
  let mut seq = Seq::default();

  let num1 = seq.next_number("key1", 0);
  assert_eq!(num1, 1);

  // current_max less then case
  let num2 = seq.next_number("key1", 0);
  assert_eq!(num2, 2);

  // current_max equal case
  let num3 = seq.next_number("key1", num2);
  assert_eq!(num3, num2 + 1);

  // current_max greater then case
  let num4 = seq.next_number("key1", 10);
  assert_eq!(num4, 11);
}