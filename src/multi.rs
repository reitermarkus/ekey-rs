use std::str::FromStr;

use nom::multi::fold_many_m_n;
use serde::Serialize;

const UNDEFINED_CHAR: char = '-';

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum UserStatus {
  Active,
  Inactive,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Finger {
  LeftPinky = 1,
  LeftRing = 2,
  LeftMiddle = 3,
  LeftPointer = 4,
  LeftThumb = 5,
  RightThumb = 6,
  RightPointer = 7,
  RightMiddle = 8,
  RightRing = 9,
  RightPinky = 0,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Key {
  Key1,
  Key2,
  Key3,
  Key4,
  PassKey,
}

#[derive(Debug, Clone, Serialize)]
pub struct Multi {
  user_id: u16,
  user_name: Option<String>,
  user_status: Option<UserStatus>,
  finger: Option<Finger>,
  key: Key,
  finger_scanner_serial: String,
  finger_scanner_name: String,
  action: usize,
  input_id: Option<usize>,
}

fn digit(s: &str) -> nom::IResult<&str, char> {
  match s.chars().nth(0) {
    Some(c) if c.is_digit(10) => {
      Ok((&s[1..], c))
    },
    _ => Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA))),
  }
}

fn alphanumeric(s: &str) -> nom::IResult<&str, char> {
  match s.chars().nth(0) {
    Some(c) if c.is_alphanumeric() || c == ' ' || c == '*' => {
      Ok((&s[1..], c))
    },
    _ => Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA))),
  }
}

fn digit_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, usize> {
  move |s: &str| {
    fold_many_m_n(
      n, n,
      digit,
      || 0,
      |acc: usize, c| {
        acc * 10 + c.to_digit(10).unwrap_or(0) as usize
      }
    )(s)
  }
}

fn alphanumeric_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, String> {
  move |s: &str| {
    fold_many_m_n(
      n, n,
      alphanumeric,
      String::new,
      |mut s: String, c| {
        s.push(c);
        s
      }
    )(s)
  }
}

fn digit_min_max(min: usize, max: usize) -> impl FnMut(&str) -> nom::IResult<&str, usize> {
  move |s: &str| {
    match digit_n(1)(s)? {
      (_, n) if n < min || n > max => {
        Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA)))
      },
      (s, n) => Ok((s, n))
    }
  }
}

fn optional_digit_min_max(min: usize, max: usize) -> impl FnMut(&str) -> nom::IResult<&str, Option<usize>> {
  use nom::character::complete::char;

  move |s: &str| {
    if let Ok((s, _)) = char::<&str, nom::error::Error<&str>>(UNDEFINED_CHAR)(s) {
      return Ok((s, None))
    }

    let (s, n) = digit_min_max(min, max)(s)?;
    Ok((s, Some(n)))
  }
}

impl Multi {
  fn nom(s: &str) -> nom::IResult<&str, Self> {
    use nom::character::{complete::{anychar, char}};

    let (s, _) = char('1')(s)?;
    let (s, separator) = anychar(s)?;
    let (s, user_id) = digit_n(4)(s)?;
    let (s, _) = char(separator)(s)?;
    let (s, user_name) = alphanumeric_n(9)(s)?;

    let user_name = if user_name.starts_with(UNDEFINED_CHAR) {
      None
    } else {
      Some(user_name.trim_end().to_owned())
    };

    let (s, _) = char(separator)(s)?;
    let (s, user_status) = optional_digit_min_max(0, 1)(s)?;
    let user_status = user_status
      .map(|n| if n == 1 { UserStatus::Active } else { UserStatus::Inactive });
    let (s, _) = char(separator)(s)?;
    let (s, finger_id) = optional_digit_min_max(0, 9)(s)?;
    let finger = finger_id.map(|f| match f {
        1 => Finger::LeftPinky,
        2 => Finger::LeftRing,
        3 => Finger::LeftMiddle,
        4 => Finger::LeftPointer,
        5 => Finger::LeftThumb,
        6 => Finger::RightThumb,
        7 => Finger::RightPointer,
        8 => Finger::RightMiddle,
        9 => Finger::RightRing,
        0 => Finger::RightPinky,
        _ => unreachable!(),
      });
    let (s, _) = char(separator)(s)?;
    let (s, key) = optional_digit_min_max(1, 4)(s)?;
    let key = match key {
      Some(1) => Key::Key1,
      Some(2) => Key::Key2,
      Some(3) => Key::Key3,
      Some(4) => Key::Key4,
      None => Key::PassKey,
      _ => unreachable!(),
    };
    let (s, _) = char(separator)(s)?;
    let (s, finger_scanner_serial) = alphanumeric_n(14)(s)?;
    let (s, _) = char(separator)(s)?;
    let (s, finger_scanner_name) = alphanumeric_n(4)(s)?;
    let finger_scanner_name = finger_scanner_name.trim_end().to_owned();
    let (s, _) = char(separator)(s)?;
    let (s, action) = digit_min_max(1, 8)(s)?;
    let (s, _) = char(separator)(s)?;
    let (s, input_id) = optional_digit_min_max(1, 4)(s)?;

    Ok((s, Multi {
      user_id: user_id as u16,
      user_name,
      user_status,
      finger,
      key,
      finger_scanner_serial,
      finger_scanner_name,
      action,
      input_id,
    }))
  }
}

impl FromStr for Multi {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use nom::combinator::all_consuming;
    use nom::Finish;

    match all_consuming(Self::nom)(s).finish() {
      Ok((_, multi)) => Ok(multi),
      Err(_) => return Err(()),
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
    assert_eq!(packet.key, Key::Key2);
    assert_eq!(packet.finger_scanner_serial, "80156809150025");
    assert_eq!(packet.finger_scanner_name, "GAR");
    assert_eq!(packet.action, 1);
    assert_eq!(packet.input_id, None);
  }
}
