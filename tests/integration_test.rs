use std::io::{self, Write};
use discount_calculation_module::{run, types::Config};

#[test]
fn test_module() {
  // Don't know how to assert if terminal output is the same :/
  // So this can be used to visualy confirm the answer
  // let expected_answer = "2015-02-01 S MR 1.50 0.50
  // 2015-02-01 S MR 1.50 0.50
  // 2015-02-02 S MR 1.50 0.50
  // 2015-02-03 L LP 6.90 -
  // 2015-02-05 S LP 1.50 -
  // 2015-02-06 S MR 1.50 0.50
  // 2015-02-06 L LP 6.90 -
  // 2015-02-07 L MR 4.00 -
  // 2015-02-08 M MR 3.00 -
  // 2015-02-09 L LP 0.00 6.90
  // 2015-02-10 L LP 6.90 -
  // 2015-02-10 S MR 1.50 0.50
  // 2015-02-10 S MR 1.50 0.50
  // 2015-02-11 L LP 6.90 -
  // 2015-02-12 M MR 3.00 -
  // 2015-02-13 M LP 4.90 -
  // 2015-02-15 S MR 1.50 0.50
  // 2015-02-17 L LP 6.90 -
  // 2015-02-17 S MR 1.90 0.10
  // 2015-02-24 L LP 6.90 -
  // 2015-02-29 CUSPS Ignored
  // 2015-03-01 S MR 1.50 0.50

  let buf = Vec::new();
  io::stdout().write_all(&buf).unwrap();

  run(Config{ file_path: "./tests/testInput.txt".to_owned() }).unwrap();
}
