use serde::{Deserialize, Serialize};

/// Configuration for the file server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Host address to bind to
    pub host: String,
    /// Port to listen on
    pub port: u16,
    /// Root directory path to serve
    pub path: String,
    /// Enable CORS support
    pub enable_cors: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            path: ".".to_string(),
            enable_cors: false,
        }
    }
}
