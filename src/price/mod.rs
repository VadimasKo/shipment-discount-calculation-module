use strum::IntoEnumIterator;
use crate::types::{ShipmentSize, Provider, Shipment};


pub fn get_month_receipt(month_shipments: &Vec<Shipment>) -> Vec<String> {
  let mut monthly_receipt: Vec<String> = vec![];
  let mut total_discount:f32 = 0.0;
  let mut large_counter: i16 = 0;

  for shipment in month_shipments {
    if shipment.size == ShipmentSize::L && shipment.provider == Provider::LP {
      large_counter += 1
    }

    let mut price: f32;
    let discount: f32;

    if large_counter == 3 && shipment.size == ShipmentSize::L {
      price = shipment.get_price();
      discount = get_max_discount(price, total_discount);
      price -= discount;

    } else {
      (price, discount) = calculate_price(shipment, total_discount)
    }

    total_discount += discount;

    monthly_receipt.push(format!("{} {} {} {:.2} {:.3}",
      shipment.date,
      shipment.size,
      shipment.provider,
      price,
      if discount > 0.0 { discount.to_string() } else { "-".to_string() } 
    ));
  }

  return monthly_receipt;
}

//======== Helper functions ========//
pub fn find_min_price(size: &ShipmentSize) -> f32 {
  return Provider::iter()
    .map(|provider| provider.get_price(size))
    .fold(f32::INFINITY, |a, b| a.min(b));
}

fn calculate_price(shipment: &Shipment, total_discount: f32) -> (f32, f32) {
  if shipment.size == ShipmentSize::S {
    let price = shipment.get_price();
    let discount = get_max_discount(price - find_min_price(&ShipmentSize::S), total_discount);

    return (price-discount, discount);
  }

  return (shipment.get_price(), 0.0);
}

fn get_max_discount(discount: f32, total_discount: f32) -> f32 {
  if total_discount + discount > 10.0 {
    return 10.0 - total_discount;
  }
  return discount;
}

//======== TESTS ========//
#[cfg(test)]
mod tests {
  use crate::{ types::Shipment, price::{calculate_price, get_month_receipt}};
  use chrono::{NaiveDate};

  #[test]
  fn test_calculate_price() {
    let shipment = Shipment{
      date: NaiveDate::from_ymd_opt(2015, 2, 1).unwrap(),
      size: crate::types::ShipmentSize::S,
      provider: crate::types::Provider::MR
    };
    
    let answer:(f32, f32) = (1.50, 0.50);
    let result = calculate_price(&shipment, 0.0);

    println!("{} {}",answer.0, result.0);
    assert_eq!(result, answer);
  }
  
  #[test]
  fn test_month_receipt() {
    let input: Vec<Shipment> = vec![];

    let result = get_month_receipt(&input);
    let answer: Vec<String> = vec![];
    assert_eq!(result, answer);
  }
}
