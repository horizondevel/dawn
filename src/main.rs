use axum::extract::State;
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
        .route("/frag", get(handle_frag))
        .route("/icon", get(handle_icon))
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
async fn handle_icon(State(state): State<AppState>) -> Html<String> {
    Html(
        state
            .templates
            .render("icon.html", &Context::default())
            .unwrap(),
    )
}

async fn handle_frag(State(state): State<AppState>) -> Html<String> {
    Html(
        state
            .templates
            .render("frag.html", &Context::default())
            .unwrap(),
    )
}
