use log::info;

use crate::packets::incoming::PackageHandler;

pub struct ReleaseVersionEvent {}

impl ReleaseVersionEvent {
  pub fn new() -> Self {
    Self {}
  }
}

impl PackageHandler for ReleaseVersionEvent {
  fn handle(&self) {
    info!("NABER");
  }
}
