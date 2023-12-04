use nom::{branch::alt, character::complete::char, combinator::value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Finger {
  /// Left pinky finger.
  LeftPinky,
  /// Left ring finger.
  LeftRing,
  /// Left middle finger.
  LeftMiddle,
  /// Left pointer finger.
  LeftPointer,
  /// Left thumb.
  LeftThumb,
  /// Right thumb.
  RightThumb,
  /// Right pointer finger.
  RightPointer,
  /// Right middle finger.
  RightMiddle,
  /// Right ring finger.
  RightRing,
  /// Right pinky finger.
  RightPinky,
  /// RFID
  Rfid,
}

impl Finger {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((
      value(Some(Self::LeftPinky), char('1')),
      value(Some(Self::LeftRing), char('2')),
      value(Some(Self::LeftMiddle), char('3')),
      value(Some(Self::LeftPointer), char('4')),
      value(Some(Self::LeftThumb), char('5')),
      value(Some(Self::RightThumb), char('6')),
      value(Some(Self::RightPointer), char('7')),
      value(Some(Self::RightMiddle), char('8')),
      value(Some(Self::RightRing), char('9')),
      value(Some(Self::RightPinky), char('0')),
      value(Some(Self::Rfid), char('R')),
      value(None, char('-')),
    ))(input)
  }
}
