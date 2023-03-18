mod shipment;
pub mod types;
use price::{print_month_receipt};
use shipment::*;
use types::{Shipment, Config};
use std::error::Error;
use std::{fs, vec};
mod price;



pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(config.file_path)?;
  let mut ignored_lines: Vec<(usize, &str)> = vec![];
  let mut shipments: Vec<Shipment> = vec![];

  for (index, line) in file_content.lines().enumerate() {
    match shipment::string_to_shipment(line) {
      Ok(shipment) => shipments.push(shipment),
      Err(_) => ignored_lines.push((index, line)),
    }
  }

  let shipments_by_month = group_by_month(&shipments);

  for month_shipments in shipments_by_month {
    print_month_receipt(&month_shipments);
  } 


  // for (index, line) in ignored_lines {
  //   println!("{index} {line}")
  // }

  Ok(())
}

