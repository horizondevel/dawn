use crate::htmx_handler::{get_handler, handle};
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::collections::HashMap;
use tera::{Context, Tera};
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

pub struct DawnServer {}
#[derive(Clone)]
pub struct AppState {
    pub templates: Tera,
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
            .route("/api/events", post(handle_new_event))
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

impl Default for DawnServer {
    fn default() -> Self {
        Self::new()
    }
}

async fn handle_home(State(state): State<AppState>) -> Html<String> {
    let mut context = Context::new();
    context.insert("title", "Dawn");
    context.insert("message", "Dawn forever");
    Html(state.templates.render("index.html", &context).unwrap())
}

#[derive(Deserialize, Debug)]
struct NewEventRequest {
    event_name: String,
    start_time: String,
    end_time: String,
}

async fn handle_new_event(
    State(state): State<AppState>,
    Form(form): Form<NewEventRequest>,
) -> Html<String> {
    println!("Handling new event {:?}", form);

    let mut context = Context::new();
    context.insert("name", &form.event_name);
    context.insert("start_time", &form.start_time);
    context.insert("end_time", &form.end_time);
    let template_name = "components/timetable/timetable_event_new.html";
    Html(state.templates.render(template_name, &context).unwrap())
}

async fn htmx_handler(
    State(state): State<AppState>,
    Path(path_frag): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> Html<String> {
    let handler = get_handler(&path_frag);
    handle(handler, &state, path_frag.clone(), &params).await
}
