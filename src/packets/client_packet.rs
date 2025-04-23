use super::buffer::Buffer;

#[derive(Debug)]
pub struct ClientPacket {
  pub header: u32,
  pub buffer: Buffer,
}

impl ClientPacket {
  pub fn new(header: u32, buffer: Buffer) -> Self {
    Self {
      header,
      buffer,
    }
  }

  pub fn read_short(&mut self) -> u16 {
    self.buffer.read_short()
  }

  pub fn read_int(&mut self) -> u32 {
    self.buffer.read_int()
  }

  pub fn read_boolean(&mut self) -> bool {
    self.buffer.read_byte() == 1
  }

  pub fn read_string(&mut self) -> String {
    self.buffer.read_string()
  }
}
