use chrono::{NaiveDate, Datelike};
use strum_macros::{EnumIter, EnumString};
use std::str::FromStr;
use std::error::Error;


#[derive(Debug, PartialEq, EnumString, Clone)]
pub enum ShipmentSize {
  L,
  M,
  S,
}

#[derive(EnumString, Debug, Clone, EnumIter, Copy)]
pub enum Provider {
  MR,
  LP,
  
}

impl Provider {
  pub fn get_price(&self, size: &ShipmentSize) -> f32 {
    match self {
        Provider::LP => {
          match size {
            ShipmentSize::L => 6.90,
            ShipmentSize::M => 4.90,
            ShipmentSize::S => 1.50,
        }
        },
        Provider::MR => {
          match size {
            ShipmentSize::L => 4.0,
            ShipmentSize::M => 3.0,
            ShipmentSize::S => 2.0,
        }
        },
    }
  }
}


#[derive(Debug, Clone)]
pub struct Shipment {
  pub date: NaiveDate,
  pub size: ShipmentSize,
  pub provider: Provider
}

impl Shipment {
  pub fn get_price(&self) -> f32 {
    self.provider.get_price(&self.size)
  }
}

// Expected line format:
// 2015-02-01 S MR
pub fn string_to_shipment(input: &str) -> Result<Shipment,  Box<dyn Error>>  {
  let values: Vec<&str> = input.split_whitespace().collect(); 
  // if values.len() != 3 { return Error("Invalid input string")}

  let date = NaiveDate::from_str(values[0])?;
  let size = ShipmentSize::from_str(values[1])?;
  let provider = Provider::from_str(values[2])?;

  Ok(Shipment { date, size, provider })
}

pub fn group_by_month(shipments: &Vec<Shipment>) -> Vec<Vec<Shipment>> {
  let mut shipments_by_month: Vec<Vec<Shipment>> = vec![];

  let mut current_month = shipments[0].date.month();
  let mut current_year = shipments[0].date.year();
  let mut month_shipments: Vec<Shipment> = vec![];

  for shipment in shipments {
    if shipment.date.month() == current_month && shipment.date.year() == current_year { 
      month_shipments.push(shipment.clone())
    } else {
      shipments_by_month.push(month_shipments);
      month_shipments = vec![shipment.clone()];
      current_month = shipment.date.month();
      current_year = shipment.date.year();
    }
  }

  if !month_shipments.is_empty() { shipments_by_month.push(month_shipments) }

  return shipments_by_month;
}
