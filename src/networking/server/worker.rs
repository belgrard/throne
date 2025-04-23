use crate::packets::{packet_manager::PacketManager, client_packet::ClientPacket};
use log::{debug, info};

#[derive(Debug)]
pub struct WorkItem {
    pub packet: ClientPacket,
}

pub async fn worker_loop(
  mut receiver: tokio::sync::mpsc::UnboundedReceiver<WorkItem>,
  shard_id: usize,
) {
  let mut manager = PacketManager::new();
  while let Some(work) = receiver.recv().await {
      manager.handle_packet(work.packet);
      debug!("Shard {} processed a packet", shard_id);
  }
  info!("Shard {} exiting", shard_id);
}
