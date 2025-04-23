use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Buffer {
  cursor: Cursor<Vec<u8>>,
}

impl Buffer {
  pub fn new(data: Vec<u8>) -> Self {
    Buffer {
      cursor: Cursor::new(data),
    }
  }

  pub fn read_byte(&mut self) -> u8 {
    self.cursor.read_u8().unwrap()
  }

  pub fn read_short(&mut self) -> u16 {
    self.cursor.read_u16::<BigEndian>().unwrap()
  }

  pub fn read_int(&mut self) -> u32 {
    self.cursor.read_u32::<BigEndian>().unwrap()
  }

  pub fn read_string(&mut self) -> String {
    let len = self.read_short() as usize;
    let mut buffer = vec![0u8; len];
    self.cursor.read_exact(&mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
  }

  pub fn remaining(&self) -> usize {
    (self.cursor.get_ref().len() as u64 - self.cursor.position()) as usize
  }
}
