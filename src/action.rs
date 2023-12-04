use core::fmt;

use nom::{branch::alt, character::complete::char, combinator::value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum Action {
  /// Open.
  Open,
  /// Refuse unrecognized finger.
  RefuseUnrecognizedFinger,
  /// Refuse time slot A user.
  RefuseTimeSlotA,
  /// Refuse time slot B user.
  RefuseTimeSlotB,
  /// Refuse disabled user.
  RefuseDisabled,
  /// Refuse time-restricted user.
  RefuseTimeRestricted,
  /// Finger scanner not paired.
  FingerScannerNotPaired,
  /// Digital input.
  DigitalInput,
  /// One minute code pad lock.
  OneMinuteCodePadLock,
  /// Fifteen minute code pad lock.
  FifteenMinuteCodePadLock,
}

impl fmt::Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Open => "open",
      Self::RefuseUnrecognizedFinger => "refuse unrecognized finger",
      Self::RefuseTimeSlotA => "refuse time slot A user",
      Self::RefuseTimeSlotB => "refuse time slot B user",
      Self::RefuseDisabled => "refuse disabled user",
      Self::RefuseTimeRestricted => "refuse time-restricted user",
      Self::FingerScannerNotPaired => "finger scanner not paired",
      Self::DigitalInput => "digital input",
      Self::OneMinuteCodePadLock => "one minute code pad lock",
      Self::FifteenMinuteCodePadLock => "fifteen minute code pad lock",
    }
    .fmt(f)
  }
}

impl Action {
  pub(crate) fn nom(input: &str) -> nom::IResult<&str, Self> {
    alt((
      value(Self::Open, char('1')),
      value(Self::RefuseUnrecognizedFinger, char('2')),
      value(Self::RefuseTimeSlotA, char('3')),
      value(Self::RefuseTimeSlotB, char('4')),
      value(Self::RefuseDisabled, char('5')),
      value(Self::RefuseTimeRestricted, char('6')),
      value(Self::FingerScannerNotPaired, char('7')),
      value(Self::DigitalInput, char('8')),
      value(Self::OneMinuteCodePadLock, char('A')),
      value(Self::FifteenMinuteCodePadLock, char('B')),
    ))(input)
  }
}
