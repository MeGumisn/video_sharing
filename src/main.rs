use crate::video_play::{get_video_root, read_conf};
use axum::routing::get;
use axum::Router;
use std::path::Path;
use tokio::fs;
use tower_http::cors::CorsLayer;
use tower_http::services::{ServeDir, ServeFile};

mod video_play;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::permissive();

    let config = read_conf();
    let video_root = config.dir_path.as_str();
    let port = config.port;
    // Service for serving the main frontend page
    let home_service = ServeFile::new("public/index.html");
    let video_service = ServeDir::new(video_root);

    if !Path::new(video_root).exists() {
        fs::create_dir_all(video_root).await.unwrap();
    }

    // 构建路由
    let app = Router::new()
        // 1. 访问 "/" 时，直接映射并返回 static/index.html 文件
        .route_service("/", home_service)
        // 2. 传统 API 接口
        .route("/api/list", get(video_play::list_directory))
        .nest_service("/stream", video_service)
        .layer(cors);

    let addr = format!("0.0.0.0:{}", port);
    println!("Web player interface is live at http://localhost:{}", port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
