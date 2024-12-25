use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let listner = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    serve(listner, app).await.unwrap();
}
