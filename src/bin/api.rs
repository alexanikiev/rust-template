use axum::Server;
use rusttemplate::api::routes::create_router;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = create_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Starting API server on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
