use strum::IntoEnumIterator;
use crate::types::{ShipmentSize, Provider, Shipment};


pub fn get_month_receipt(month_shipments: &Vec<Shipment>) -> Vec<String> {
  let mut monthly_receipt: Vec<String> = vec![];
  let mut total_discount:f32 = 0.0;
  let mut large_counter: i16 = 0;

  for shipment in month_shipments {
    if shipment.size == ShipmentSize::L { large_counter += 1 }
    let mut price: f32;
    let discount: f32;

    // BUG in example or as it should be?
    if large_counter == 4 && shipment.size == ShipmentSize::L {
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
