use axum::{
    body::Body,
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs::{self, File};
use tokio_util::io::ReaderStream;
// 引入静态文件服务组件

pub const VIDEO_ROOT: &str = "F:\\迅雷下载\\Animate";
const CONFIG_FILE: &str = "config.json";

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dir_path: String,
    pub port:u32
}

impl Default for Config {
    fn default() -> Self {
        Self{
            dir_path:"./".to_owned(),
            port:3000
        }

    }
}
pub fn read_conf()->Config {
    match std::fs::File::open(CONFIG_FILE) {
        Err(e) => {
            println!("Error opening config file: {:?}", e);
            Config::default()
        }
        Ok(file) => {
            let mut de = serde_json::Deserializer::from_reader(file);
            Config::deserialize(&mut de).unwrap_or_else(|e| {
                println!("Error deserializing config file: {:?}", e);
                Config::default()
            })
        }
    }
}

pub fn get_video_root() -> String {
    read_conf().dir_path
}

#[derive(Deserialize)]
pub struct PathParam {
    path: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FileItem {
    name: String,
    rel_path: String,
    is_dir: bool,
}

// 路径防跨越安全检查
fn safe_resolve_path(rel_path: &str) -> Option<PathBuf> {
    let mut full_path = PathBuf::from(get_video_root());
    for component in Path::new(rel_path).components() {
        match component {
            std::path::Component::Normal(p) => full_path.push(p),
            _ => return None,
        }
    }
    Some(full_path)
}

// 获取目录列表的 API
pub async fn list_directory(
    Query(params): Query<PathParam>,
) -> Result<Json<Vec<FileItem>>, (StatusCode, String)> {
    let rel_dir = params.path.unwrap_or_default();
    let full_path =
        safe_resolve_path(&rel_dir).ok_or((StatusCode::BAD_REQUEST, "非法路径".to_string()))?;

    if !full_path.exists() {
        return Err((StatusCode::NOT_FOUND, "未找到该目录".to_string()));
    }

    let mut items = Vec::new();
    let mut entries = fs::read_dir(&full_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    while let Ok(Some(entry)) = entries.next_entry().await {
        let file_type = entry
            .file_type()
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        let name = entry.file_name().to_string_lossy().into_owned();

        let is_dir = file_type.is_dir();
        let is_video = !is_dir
            && (name.ends_with(".mp4") || name.ends_with(".mkv") || name.ends_with(".webm"));

        if is_dir || is_video {
            let new_rel_path = if rel_dir.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", rel_dir, name)
            };

            items.push(FileItem {
                name,
                rel_path: new_rel_path,
                is_dir,
            });
        }
    }

    // 排序：文件夹排在最上方，视频文件按名称字母排序
    items.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));

    Ok(Json(items))
}

#[cfg(test)]
mod tests {
    use crate::video_play::{list_directory, PathParam};
    use axum::extract::Query;

    #[tokio::test]
    async fn test_list_videos() -> Result<(), (axum::http::StatusCode, String)> {
        let result = list_directory(Query::<PathParam>(PathParam {
            path: Some("".to_string()),
        }))
        .await?;
        println!("{:?}", result);
        Ok(())
    }
}
