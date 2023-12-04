use nom::multi::fold_many_m_n;

pub fn digit(s: &str) -> nom::IResult<&str, char> {
  match s.chars().next() {
    Some(c) if c.is_ascii_digit() => Ok((&s[1..], c)),
    _ => Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA))),
  }
}

pub fn alphanumeric(s: &str) -> nom::IResult<&str, char> {
  match s.chars().next() {
    Some(c) if c.is_alphanumeric() || c == ' ' || c == '*' => Ok((&s[1..], c)),
    _ => Err(nom::Err::Error(nom::error::Error::new(s, nom::error::ErrorKind::IsA))),
  }
}

pub fn digit_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, usize> {
  move |s: &str| fold_many_m_n(n, n, digit, || 0, |acc: usize, c| acc * 10 + c.to_digit(10).unwrap_or(0) as usize)(s)
}

pub fn alphanumeric_n(n: usize) -> impl FnMut(&str) -> nom::IResult<&str, String> {
  move |s: &str| {
    fold_many_m_n(n, n, alphanumeric, String::new, |mut s: String, c| {
      s.push(c);
      s
    })(s)
  }
}
