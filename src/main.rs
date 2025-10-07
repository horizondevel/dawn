use std::collections::HashMap;
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use tera::{Context, Tera};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Clone)]
struct AppState {
    templates: Tera,
}
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app_state = AppState {
        templates: Tera::new("templates/**/*.html").unwrap(),
    };
    let static_serve =
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));
    let app = Router::new()
        .route("/", get(handle_home))
        .route("/fragments/v1/{*path}", get(htmx_handler))
        .with_state(app_state)
        .nest_service("/static", static_serve);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:9999").await?;
    axum::serve(listener, app).await?;
    Ok(())
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