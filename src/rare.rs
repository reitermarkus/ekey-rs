#[derive(Debug, Clone)]
pub struct Rare {
  pub version: i32,
  pub cmd: i32,
  pub terminal_id: i32,
  pub terminal_serial: [char; 14],
  pub relay_id: u8,
  pub reserved: u8,
  pub user_id: i32,
  pub finger: i32,
  pub event: [char; 16],
  pub time: [char; 16],
  pub name: u16,
  pub personal_id: u16,
}

impl Rare {
  #[allow(unused)]
  fn nom(b: &[u8]) -> nom::IResult<&[u8], Self> {
    todo!()
  }
}
