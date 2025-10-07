use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new().route("/", get(hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn hello() -> String {
    "Hello, world!".to_string()
}