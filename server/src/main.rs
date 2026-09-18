use axum::{Router, extract::{Path, Query}, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/hello", get(hello))
    .route("/echo", get(echo));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> String {
    format!("Hello world!")
}

#[derive(serde::Deserialize)]
struct Echo {
    string: Option<String>
}

async fn echo(string: Query<Echo>) -> String {
    if let Some(string) = string.0.string {
        string
    } else {
        "no string to echo".to_string()
    }
}