use core::str::FromStr;

use nom::{
  branch::alt,
  combinator::{all_consuming, value},
  Finish,
};

use nom::character::complete::{anychar, char};
use serde::Serialize;

use crate::{
  nom::{u16_n, u64_n},
  Action, Finger,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Relay {
  /// Relay 1
  Relay1,
  /// Relay 2
  Relay2,
  /// Relay 3
  Relay3,
  /// Relay 4
  Relay4,
  /// Double relay.
  DoubleRelay,
}

impl Relay {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((
      value(Some(Self::Relay1), char('1')),
      value(Some(Self::Relay2), char('2')),
      value(Some(Self::Relay3), char('3')),
      value(Some(Self::Relay4), char('4')),
      value(Some(Self::DoubleRelay), char('d')),
      value(None, char('-')),
    ))(input)
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Home {
  user_id: u16,
  finger: Option<Finger>,
  finger_scanner_serial: u64,
  action: Action,
  relay: Option<Relay>,
}

impl Home {
  pub fn user_id(&self) -> u16 {
    self.user_id
  }

  pub fn finger(&self) -> Option<Finger> {
    self.finger
  }

  pub fn finger_scanner_serial(&self) -> u64 {
    self.finger_scanner_serial
  }

  pub fn action(&self) -> Action {
    self.action
  }

  pub fn relay(&self) -> Option<Relay> {
    self.relay
  }

  fn nom(input: &str) -> nom::IResult<&str, Self> {
    let (input, _) = char('1')(input)?;
    let (input, separator) = anychar(input)?;
    let (input, user_id) = u16_n(4)(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, finger) = Finger::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, finger_scanner_serial) = u64_n(14)(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, action) = Action::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, relay) = Relay::nom(input)?;

    Ok((input, Self { user_id, finger, finger_scanner_serial: finger_scanner_serial.to_owned(), action, relay }))
  }
}

impl FromStr for Home {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match all_consuming(Self::nom)(s).finish() {
      Ok((_, multi)) => Ok(multi),
      Err(_) => Err(()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse() {
    let packet = "1_0003_7_80156809150025_1_2".parse::<Home>().unwrap();

    assert_eq!(packet.user_id, 3);
    assert_eq!(packet.finger, Some(Finger::RightPointer));
    assert_eq!(packet.finger_scanner_serial, 80156809150025);
    assert_eq!(packet.action, Action::Open);
    assert_eq!(packet.relay, Some(Relay::Relay2));
  }
}
