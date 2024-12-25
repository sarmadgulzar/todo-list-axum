mod db;
mod error;
mod handlers;
mod models;
mod routes;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = db::initialize_db().await;
    let app = routes::create_router(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
