  use axum::{
    extract::Multipart,
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
    http::StatusCode,
};
use std::path::Path;
use tokio::fs;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() {
   
    fs::create_dir_all("uploads").await.unwrap();
    fs::create_dir_all("public").await.unwrap();

    let app = Router::new().nest_service("/", ServeDir::new("public")).nest_service("/files", ServeDir::new("uploads"))
        .route("/api/files", get(all_files)).route("/upload", post(uploader));

    println!(" http://localhost:3000");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
   
    axum::serve(listener, app).await.unwrap();
}

async fn uploader(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        
        if name == "file" {
            let filename = field.file_name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            let filepath = Path::new("uploads").join(&filename);
            fs::write(&filepath, data).await.unwrap();
            println!(" Uploaded: {}", filename);
        }
    }
    StatusCode::OK
}

async fn all_files() -> Json<Vec<String>> {
    let mut file_names = Vec::new();
   
    if let Ok(mut entries) = fs::read_dir("uploads").await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let filename = entry.file_name().into_string().unwrap();
            file_names.push(filename);
        }
    }
    Json(file_names)
}