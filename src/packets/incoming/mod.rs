pub mod handshake;

pub trait PackageHandler: Send + Sync {
  fn handle(&self);
}
