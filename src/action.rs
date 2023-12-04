use nom::{branch::alt, character::complete::char, combinator::value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Action {
  /// Open.
  Open,
  /// Refused unknown finger.
  RefusedUnknownFinger,
  /// Refused timeframe A.
  RefusedTimeframeA,
  /// Refused timeframe B.
  RefusedTimeframeB,
  /// Refused inactive.
  RefusedInactive,
  /// Refused only always users.
  RefusedOnlyAlwaysUsers,
  /// Finger scanner not paired.
  FingerScannerNotPaired,
  /// Digital input.
  DigitalInput,
  /// One minute code lock.
  OneMinuteCodeLock,
  /// Fifteen minute code lock.
  FifteenMinuteCodeLock,
}

impl Action {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Self> {
    alt((
      value(Self::Open, char('1')),
      value(Self::RefusedUnknownFinger, char('2')),
      value(Self::RefusedTimeframeA, char('3')),
      value(Self::RefusedTimeframeB, char('4')),
      value(Self::RefusedInactive, char('5')),
      value(Self::RefusedOnlyAlwaysUsers, char('6')),
      value(Self::FingerScannerNotPaired, char('7')),
      value(Self::DigitalInput, char('8')),
      value(Self::OneMinuteCodeLock, char('A')),
      value(Self::FifteenMinuteCodeLock, char('B')),
    ))(input)
  }
}
