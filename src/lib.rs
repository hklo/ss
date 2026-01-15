//! # SS (Share Store)
//!
//! A simple and lightweight file sharing library and server.
//!
//! ## Features
//!
//! - Serve static files from a directory
//! - Simple HTTP file server
//! - CORS support
//! - Directory listing
//!
//! ## Usage as a library
//!
//! ```no_run
//! use ss::{ServerConfig, start_server};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = ServerConfig {
//!         host: "127.0.0.1".to_string(),
//!         port: 3000,
//!         path: ".".to_string(),
//!         enable_cors: true,
//!     };
//!     
//!     start_server(config).await?;
//!     Ok(())
//! }
//! ```

pub mod config;
pub mod handler;
pub mod server;

pub use config::ServerConfig;
pub use server::start_server;
