use core::fmt;

use nom::{branch::alt, character::complete::char, combinator::value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Finger {
  /// Left little finger.
  LeftLittle,
  /// Left ring finger.
  LeftRing,
  /// Left middle finger.
  LeftMiddle,
  /// Left index finger.
  LeftIndex,
  /// Left thumb.
  LeftThumb,
  /// Right thumb.
  RightThumb,
  /// Right index finger.
  RightIndex,
  /// Right middle finger.
  RightMiddle,
  /// Right ring finger.
  RightRing,
  /// Right little finger.
  RightLittle,
  /// RFID
  Rfid,
}

impl fmt::Display for Finger {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::LeftRing => "left little finger",
      Self::LeftLittle => "left ring finger",
      Self::LeftMiddle => "left middle finger",
      Self::LeftIndex => "left index finger",
      Self::LeftThumb => "left thumb",
      Self::RightThumb => "right thumb",
      Self::RightIndex => "right index finger",
      Self::RightMiddle => "right middle finger",
      Self::RightRing => "right ring finger",
      Self::RightLittle => "right little finger",
      Self::Rfid => "RFID",
    }
    .fmt(f)
  }
}

impl Finger {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Option<Self>> {
    alt((
      value(Some(Self::LeftLittle), char('1')),
      value(Some(Self::LeftRing), char('2')),
      value(Some(Self::LeftMiddle), char('3')),
      value(Some(Self::LeftIndex), char('4')),
      value(Some(Self::LeftThumb), char('5')),
      value(Some(Self::RightThumb), char('6')),
      value(Some(Self::RightIndex), char('7')),
      value(Some(Self::RightMiddle), char('8')),
      value(Some(Self::RightRing), char('9')),
      value(Some(Self::RightLittle), char('0')),
      value(Some(Self::Rfid), char('R')),
      value(None, char('-')),
    ))(input)
  }
}
