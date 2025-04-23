pub mod worker;

use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message};
use futures_util::StreamExt;
use log::{debug, error, info, trace};

use crate::packets::{buffer::Buffer, client_packet::ClientPacket};
use worker::{WorkItem, worker_loop};

const WORKER_COUNT: usize = 4;

pub struct Server {
    host: String,
}

impl Server {
    pub fn new(host: String) -> Self {
        Self { host }
    }

    pub async fn start(self: Arc<Self>) {
        let listener = TcpListener::bind(&self.host)
            .await
            .expect("Failed to bind to address");
        info!("WebSocket server listening on {}", self.host);

        let mut senders = Vec::with_capacity(WORKER_COUNT);
        for shard_id in 0..WORKER_COUNT {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel::<WorkItem>();
            senders.push(tx);
            tokio::spawn(worker_loop(rx, shard_id));
            trace!("Shard {} spawned.", shard_id);
        }

        let mut rr_counter: usize = 0;
        loop {
            match listener.accept().await {
                Ok((stream, addr)) => {
                    debug!("New TCP connection from {}", addr);
                    let idx = rr_counter % senders.len();
                    rr_counter = rr_counter.wrapping_add(1);
                    let sender = senders[idx].clone();
                    tokio::spawn(handle_connection(stream, addr, sender));
                }
                Err(e) => error!("Error accepting connection: {}", e),
            }
        }
    }
}

async fn handle_connection(
    stream: tokio::net::TcpStream,
    addr: SocketAddr,
    work_sender: tokio::sync::mpsc::UnboundedSender<WorkItem>,
) {
    let mut ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            error!("WebSocket handshake failed for {}: {}", addr, e);
            return;
        }
    };

    while let Some(msg) = ws_stream.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                let mut buffer = Buffer::new(data.to_vec());
                buffer.read_int();
                let header = buffer.read_short();
                let packet = ClientPacket::new(header.into(), buffer);

                if let Err(e) = work_sender.send(WorkItem { packet }) {
                    error!("Failed to enqueue packet: {}", e);
                }
            }
            Ok(_) => debug!("Non-binary frame from {}", addr),
            Err(e) => {
                error!("Error reading from {}: {}", addr, e);
                break;
            }
        }
    }
}