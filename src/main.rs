use clap::Parser;
use config::ThroneConfig;
use emulator::Throne;

mod packets;
mod emulator;
mod networking;
mod config;

#[derive(Parser, Debug)]
#[command(author, version, about = "Throne Emulator")]
pub struct Args {
    #[arg(long, short)]
    debug_logging: bool,
}

#[tokio::main]
async fn main() {
    let config = ThroneConfig::from_env();

    let mut throne = Throne::new(config);
    throne.run().await;
}
