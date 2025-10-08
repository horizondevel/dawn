use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::{Context, Tera};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

pub struct DawnServer {}

#[derive(Clone)]
struct AppState {
    templates: Tera,
}
impl DawnServer {
    pub const fn new() -> DawnServer {
        DawnServer {}
    }

    pub async fn serve(&self) -> Result<(), String> {
        let app_state = AppState {
            templates: Tera::new("templates/**/*.html").unwrap(),
        };
        let static_serve =
            ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));
        let app = Router::new()
            .route("/", get(handle_home))
            .route("/fragments/v1/{*path}", get(htmx_handler))
            .with_state(app_state)
            .layer(CompressionLayer::new())
            .nest_service("/static", static_serve);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await;
        if listener.is_err() {
            return Err(String::from("Could not bind to port 9999"));
        }
        let serve_res = axum::serve(listener.unwrap(), app).await;
        if serve_res.is_err() {
            return Err(String::from("Error occurred starting server"));
        }
        Ok(())
    }
}

async fn handle_home(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "Dawn");
    context.insert("message", "Dawn forever");
    Html(state.templates.render("index.html", &context).unwrap())
}

async fn htmx_handler(
    State(state): State<AppState>,
    Path(path_frag): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let mut context = Context::new();
    params.iter().for_each(|(k, v)| {
        context.insert(k, &uppercase_first_letter(v));
    });
    println!("{:?}", context);
    Html(
        state
            .templates
            .render(&*(path_frag + ".html"), &context)
            .unwrap(),
    )
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
