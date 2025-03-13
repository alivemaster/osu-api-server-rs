use super::*;
use axum::{
    body::Body,
    http::{StatusCode, header},
    response::IntoResponse,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn handler(Path(path): Path<String>) -> impl IntoResponse {
    let exe_path = std::env::current_exe().unwrap();
    let self_dir = exe_path.parent().unwrap();
    let cache_dir = self_dir.join("cache");
    let file_path = cache_dir.join(&path);

    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("404 not found: {}", err))),
    };
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let content_type = match mime_guess::from_path(&file_path).first_raw() {
        Some(content_type) => content_type,
        None => "application/octet-stream",
    };
    let headers = [(header::CONTENT_TYPE, content_type)];

    Ok((headers, body))
}
