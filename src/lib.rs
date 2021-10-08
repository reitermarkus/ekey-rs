struct Rare {
  version: i32,
  cmd: i32,
  terminal_id: i32,
  terminal_serial: [char; 14],
  relay_id: u8,
  reserved: u8,
  user_id: i32,
  finger: i32,
  event: [char; 16],
  time: [char; 16],
  name: u16,
  personal_id: u16,
}

enum UserStatus {
  Active,
  Inactive,
}

enum Finger {
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

struct Multi {
  user_id: u16,
  user_name: Option<String>,
  user_status: Option<UserStatus>,
  finger: Option<Finger>,
}

impl Multi {
  fn nom(s: &str) -> nom::IResult<&str, Self> {
    use nom::character::complete::{anychar, char};

    let (s, _) = char('1')(s)?;
    let (s, separator) = anychar(s)?;
    let (s, user_id): (&str, _) = todo!();
    let (s, user_name): (&str, _) = todo!();
    let (s, user_status): (&str, _) = todo!();
    let (s, finger):  (&str, _) = todo!();

    Ok((s, Multi {
      user_id,
      user_name,
      user_status,
      finger,
    }))
  }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
