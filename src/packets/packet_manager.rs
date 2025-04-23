use std::collections::HashMap;
use super::incoming::PackageHandler;
use super::client_packet::ClientPacket;
use log::debug;

use super::incoming::handshake::release_version::ReleaseVersionEvent;

pub struct PacketManager {
  incoming: HashMap<u32, Box<dyn PackageHandler>>,
}

impl PacketManager {
  pub fn new() -> Self {
    let mut manager = PacketManager { incoming: HashMap::new() };
    
    manager.register_handshake();

    manager
  }

  pub fn register_handler(&mut self, header: u32, handler: Box<dyn PackageHandler>) {
    self.incoming.insert(header, handler);
  }

  fn register_handshake(&mut self) {
    self.register_handler(4000, Box::new(ReleaseVersionEvent::new()));
  }

  pub fn handle_packet(&mut self, mut packet: ClientPacket) {
    debug!("Packet header: {}", packet.header);

    if self.is_registered(packet.header) {
      let handler = self.incoming.get(&packet.header).unwrap();

      handler.handle();
    }
  }

  fn is_registered(&mut self, header: u32) -> bool {
    self.incoming.contains_key(&header)
  }
}
