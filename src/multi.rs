use alloc::{borrow::ToOwned, string::String};
use core::str::FromStr;

use nom::{
  branch::alt,
  character::complete::{anychar, char},
  combinator::value,
};
use serde::Serialize;

use crate::{
  nom::{alphanumeric_n, u16_n, u64_n},
  Action, Finger,
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
  /// Active.
  Active,
  /// Inactive.
  Inactive,
}

impl UserStatus {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((value(Some(Self::Active), char('1')), value(Some(Self::Inactive), char('2')), value(None, char('-'))))(input)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
  /// Key 1
  Key1,
  /// Key 2
  Key2,
  /// Key 2
  Key3,
  /// Key 2
  Key4,
}

impl Key {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((
      value(Some(Self::Key1), char('1')),
      value(Some(Self::Key2), char('2')),
      value(Some(Self::Key3), char('3')),
      value(Some(Self::Key4), char('4')),
      value(None, char('-')),
    ))(input)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalInput {
  /// Input 1
  Input1,
  /// Input 2
  Input2,
  /// Input 3
  Input3,
  /// Input 4
  Input4,
}

impl DigitalInput {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((
      value(Some(Self::Input1), char('1')),
      value(Some(Self::Input2), char('2')),
      value(Some(Self::Input3), char('3')),
      value(Some(Self::Input4), char('4')),
      value(None, char('-')),
    ))(input)
  }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Multi {
  user_id: u16,
  user_name: Option<String>,
  user_status: Option<UserStatus>,
  finger: Option<Finger>,
  key: Option<Key>,
  finger_scanner_serial: u64,
  finger_scanner_name: String,
  action: Action,
  input: Option<DigitalInput>,
}

impl Multi {
  pub fn user_id(&self) -> u16 {
    self.user_id
  }

  pub fn user_name(&self) -> Option<&str> {
    self.user_name.as_deref()
  }

  pub fn user_staus(&self) -> Option<UserStatus> {
    self.user_status
  }

  pub fn finger(&self) -> Option<Finger> {
    self.finger
  }

  pub fn key(&self) -> Option<Key> {
    self.key
  }

  pub fn finger_scanner_serial(&self) -> u64 {
    self.finger_scanner_serial
  }

  pub fn finger_scanner_name(&self) -> &str {
    &self.finger_scanner_name
  }

  pub fn action(&self) -> Action {
    self.action
  }

  pub fn input(&self) -> Option<DigitalInput> {
    self.input
  }

  fn nom(input: &str) -> nom::IResult<&str, Self> {
    let (input, _) = char('1')(input)?;
    let (input, separator) = anychar(input)?;
    let (input, user_id) = u16_n(4)(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, user_name) = alphanumeric_n(9)(input)?;
    let user_name = if user_name.starts_with('-') { None } else { Some(user_name.trim_end().to_owned()) };
    let (input, _) = char(separator)(input)?;
    let (input, user_status) = UserStatus::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, finger) = Finger::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, key) = Key::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, finger_scanner_serial) = u64_n(14)(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, finger_scanner_name) = alphanumeric_n(4)(input)?;
    let finger_scanner_name = finger_scanner_name.trim_end().to_owned();
    let (input, _) = char(separator)(input)?;
    let (input, action) = Action::nom(input)?;
    let (input, _) = char(separator)(input)?;
    let (input, digital_input) = DigitalInput::nom(input)?;

    Ok((
      input,
      Multi {
        user_id,
        user_name,
        user_status,
        finger,
        key,
        finger_scanner_serial: finger_scanner_serial.to_owned(),
        finger_scanner_name,
        action,
        input: digital_input,
      },
    ))
  }
}

impl FromStr for Multi {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use nom::{combinator::all_consuming, Finish};

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
    let packet = "1_0003_JOSEF    _1_7_2_80156809150025_GAR _1_-".parse::<Multi>().unwrap();

    assert_eq!(packet.user_id, 3);
    assert_eq!(packet.user_name, Some("JOSEF".to_owned()));
    assert_eq!(packet.user_status, Some(UserStatus::Active));
    assert_eq!(packet.finger, Some(Finger::RightPointer));
    assert_eq!(packet.key, Some(Key::Key2));
    assert_eq!(packet.finger_scanner_serial, 80156809150025);
    assert_eq!(packet.finger_scanner_name, "GAR");
    assert_eq!(packet.action, Action::Open);
    assert_eq!(packet.input, None);
  }
}
