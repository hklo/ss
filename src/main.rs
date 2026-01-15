use clap::Parser;
use ss::{start_server, ServerConfig};

/// SS - A simple and lightweight file sharing server
#[derive(Parser, Debug)]
#[command(name = "ss")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to serve
    #[arg(default_value = ".")]
    path: String,

    /// Host address to bind to
    #[arg(short = 'H', long, default_value = "127.0.0.1")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Enable CORS support
    #[arg(long)]
    cors: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = ServerConfig {
        host: args.host,
        port: args.port,
        path: args.path,
        enable_cors: args.cors,
    };

    start_server(config).await?;

    Ok(())
}
