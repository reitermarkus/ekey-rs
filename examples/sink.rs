use std::{net::UdpSocket, str};

use ekey::multi::Multi;

fn main() {
  let socket = UdpSocket::bind("0.0.0.0:56000").unwrap();

  let mut buf = [0; 512];
  loop {
    let (size, _) = socket.recv_from(&mut buf).unwrap();
    let s = str::from_utf8(&buf[..size]).unwrap();

    dbg!(s.parse::<Multi>().unwrap());
  }
}
