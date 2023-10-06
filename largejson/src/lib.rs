pub mod webservice;
pub mod schema;
pub mod error;
pub mod tokio_tools;

/// Name of the Crate
pub const NAME: &str = env!("CARGO_PKG_NAME");
/// Version of the Crate
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
