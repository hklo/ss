use axum::{
    body::Body,
    extract::State,
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use std::path::{Path, PathBuf};
use tokio::fs;
use tokio_util::io::ReaderStream;

/// State shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub root_path: PathBuf,
}

/// Handle file serving and directory listing
pub async fn file_handler(State(state): State<AppState>, uri: Uri) -> Result<Response, StatusCode> {
    let path = uri.path().trim_start_matches('/');
    let mut file_path = state.root_path.join(path);

    // Security: prevent directory traversal
    if !file_path.starts_with(&state.root_path) {
        return Err(StatusCode::FORBIDDEN);
    }

    // Handle directory requests
    if file_path.is_dir() {
        let index_path = file_path.join("index.html");
        if index_path.exists() {
            file_path = index_path;
        } else {
            return Ok(render_directory_listing(&file_path, path)
                .await?
                .into_response());
        }
    }

    // Serve file
    serve_file(&file_path).await
}

/// Serve a single file
async fn serve_file(path: &Path) -> Result<Response, StatusCode> {
    let file = fs::File::open(path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let metadata = file
        .metadata()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let content_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_LENGTH, metadata.len())
        .body(body)
        .unwrap())
}

/// Render directory listing as HTML
async fn render_directory_listing(dir: &Path, uri_path: &str) -> Result<Html<String>, StatusCode> {
    let mut entries = fs::read_dir(dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut items = Vec::new();

    // Add parent directory link if not at root
    if uri_path != "" {
        items.push(r#"<li>📁 <a href="..">..</a></li>"#.to_string());
    }

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let file_name = entry.file_name().to_string_lossy().to_string();
        let metadata = entry
            .metadata()
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let icon = if metadata.is_dir() { "📁" } else { "📄" };
        let link = if uri_path.is_empty() {
            format!("/{}", file_name)
        } else {
            format!("/{}/{}", uri_path, file_name)
        };

        items.push(format!(
            r#"<li>{} <a href="{}">{}</a></li>"#,
            icon, link, file_name
        ));
    }

    let html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Directory listing for /{}</title>
    <style>
        body {{ font-family: monospace; max-width: 800px; margin: 50px auto; padding: 0 20px; }}
        h1 {{ color: #333; }}
        ul {{ list-style: none; padding: 0; }}
        li {{ padding: 8px; border-bottom: 1px solid #eee; }}
        li:hover {{ background-color: #f5f5f5; }}
        a {{ text-decoration: none; color: #0066cc; }}
        a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <h1>📂 Index of /{}</h1>
    <ul>{}</ul>
    <hr>
    <footer><small>SS File Server</small></footer>
</body>
</html>"#,
        uri_path,
        uri_path,
        items.join("\n")
    );

    Ok(Html(html))
}
