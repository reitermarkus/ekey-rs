use nom::{bytes::complete::take_while_m_n, multi::fold_many_m_n};

fn digit(s: &str) -> nom::IResult<&str, char> {
  match s.chars().next() {
    Some(c) if c.is_ascii_digit() => Ok((&s[1..], c)),
    _ => Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA))),
  }
}

pub fn u16_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, u16> {
  move |s: &str| fold_many_m_n(n, n, digit, || 0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as u16)(s)
}

pub fn u64_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, u64> {
  move |s: &str| fold_many_m_n(n, n, digit, || 0, |acc, c| acc * 10 + c.to_digit(10).unwrap() as u64)(s)
}

fn is_alphanumeric(c: char) -> bool {
  c.is_alphanumeric() || c == ' ' || c == '*'
}

pub fn alphanumeric_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, &str> {
  move |s: &str| take_while_m_n(n, n, is_alphanumeric)(s)
}
