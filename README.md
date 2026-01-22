# SS — Share Store

SS is a small, lightweight file sharing server inspired by dufs. It provides a simple HTTP static file server and a library you can use in your own applications.

Features
- Serve a local directory over HTTP
- Directory listing and index.html handling
- Optional permissive CORS
- `lib` and `bin` targets for embedding or running

Quick start

Run:

```
cargo run -- --path . --host 127.0.0.1 --port 3000
```

Or use as a library:

```rust
use ss::{ServerConfig, start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let c = ServerConfig { host: "127.0.0.1".into(), port: 3000, path: ".".into(), enable_cors: true };
    start_server(c).await?;
    Ok(())
}
```

License

GNU GENERAL PUBLIC LICENSE Version 3, 29 June 2007
