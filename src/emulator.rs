use crate::{networking::server::Server, config::ThroneConfig};
use log::{LevelFilter, debug, info};
use std::time::Instant;
use std::sync::Arc;

pub struct Throne {
    start_time: Instant,
    server: Arc<Server>,
    config: ThroneConfig,
}

impl Throne {
    pub fn new(config: ThroneConfig) -> Self {
        let server = Arc::new(Server::new(config.host.clone()));

        Self {
            server,
            start_time: Instant::now(),
            config,
        }
    }

    pub async fn run(&mut self) {

        self.start_time = Instant::now();

        self.configure_logging();
        self.print_banner();

        debug!(
            "Debugging enabled. If you see this message, it means Throne is running in debug mode."
        );

        let server = Arc::clone(&self.server);

        let server_handle = tokio::spawn(async move {
            server.start().await;
        });

        info!("System launched in {:?}.", self.start_time.elapsed());
        info!("Throne is now ready for connections.");

        server_handle.await.expect("Server task failed");
    }

    fn get_build(&self) -> &'static str {
        option_env!("CARGO_PKG_VERSION").unwrap_or("Throne-RUST-DEV")
    }

    fn configure_logging(&self) {
        env_logger::Builder::new()
            .filter_level(if self.config.debug {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            })
            .format_target(false)
            .format_timestamp(None)
            .init();
    }

    fn print_banner(&self) {
        info!(r#"
  __  .__                                
_/  |_|  |_________  ____   ____   ____  
\   __\  |  \_  __ \/  _ \ /    \_/ __ \ 
 |  | |   Y  \  | \(  <_> )   |  \  ___/ 
 |__| |___|  /__|   \____/|___|  /\___  >
           \/                  \/     \/ 
        "#);
        info!("Build: v{}", self.get_build());
    }
}
