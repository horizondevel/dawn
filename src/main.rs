use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let static_serve = ServeDir::new("static")
        .not_found_service(ServeFile::new("static/index.html"));
    let app = Router::new()
        .route_service("/", ServeFile::new("static/index.html"))
        .nest_service("/static", static_serve)
    ;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, app).await?;
    Ok(())
}