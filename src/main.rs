use axum::{routing, Router, Server};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(|| async { "Hello, World!" }));
    Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
