mod shipment;
pub mod types;
use price::{get_month_receipt};
use shipment::*;
use types::{Shipment, Config};
use std::error::Error;
use std::{fs, vec};
mod price;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_content = fs::read_to_string(config.file_path)?;
  let mut ignored_lines: Vec<(usize, String)> = vec![];
  let mut shipments: Vec<Shipment> = vec![];

  for (index, line) in file_content.lines().enumerate() {
    match shipment::string_to_shipment(line) {
      Ok(shipment) => shipments.push(shipment),
      Err(_) => ignored_lines.push((index, format!("{line} Ignored") )),
    }
  }

  let shipments_by_month = group_by_month(&shipments);

  let mut final_receipt: Vec<String> = vec![];

  for month_shipments in shipments_by_month {
    let monthly_receipt = get_month_receipt(&month_shipments);
    final_receipt = [final_receipt, monthly_receipt].concat();
  }

  for (index, line) in ignored_lines {
    final_receipt.insert(index, line.to_owned())
  }

  for line in final_receipt {
    println!("{line}");
  }

  Ok(())
}
