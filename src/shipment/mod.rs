use std::str::FromStr;
use std::error::Error;
use chrono::{NaiveDate, Datelike};
use crate::types::{ShipmentSize, Shipment, Provider};


// Expected line format: 2015-02-01 S MR
pub fn string_to_shipment(input: &str) -> Result<Shipment,  Box<dyn Error>>  {
  let values: Vec<&str> = input.split_whitespace().collect(); 

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

//======== TESTS ========//
#[cfg(test)]
mod tests {
  use crate::{shipment::string_to_shipment, types::Shipment};
  use chrono::{NaiveDate};

  #[test]
  fn test_string_to_shipment() {
    let answer = Shipment{
      date: NaiveDate::from_ymd_opt(2015, 2, 1).unwrap(),
      size: crate::types::ShipmentSize::S,
      provider: crate::types::Provider::MR
    };
    
    let result = string_to_shipment("2015-02-01 S MR").unwrap();
  
    assert_eq!(result, answer);
  }
  
  
  #[test]
  #[should_panic]
  fn test_fail_string_to_shipment() {
    string_to_shipment("2015-02-01 SR MR").unwrap();
  }
}
