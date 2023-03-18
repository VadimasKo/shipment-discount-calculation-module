use chrono::{NaiveDate };
use strum_macros::{ EnumString, EnumIter, Display};

#[derive(PartialEq, EnumString, Clone, Display, Debug)]
pub enum ShipmentSize {
  L,
  M,
  S,
}

#[derive(Debug, PartialEq, EnumString, Clone, EnumIter, Display)]
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

#[derive(Clone, PartialEq, Debug)]
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

pub struct Config {
  pub file_path: String,
}

impl Config {
  pub fn build(args: &[String]) -> Result<Config, &'static str> {
    let file_path: String;
    
    if args.len() < 2 { 
      file_path = "input.txt".to_owned() 
    } else  { file_path = args[1].clone(); }

    Ok(Config { file_path })
  }
}
