use crate::calendar::{make_calendar, Day, Month, WeekDay};
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use std::str::FromStr;
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
    if path_frag.contains("calendar") {
        return handle_calendar(state, path_frag, params).await;
    }
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

async fn handle_calendar(
    state: AppState,
    path_frag: String,
    params: HashMap<String, String>,
) -> Html<String> {
    let mut context = Context::new();
    params.iter().for_each(|(k, v)| {
        context.insert(k, &uppercase_first_letter(v));
    });

    let month = Month::from_str(params.get("month").unwrap()).unwrap();
    let year = params.get("year").unwrap().parse::<i32>().unwrap();

    let may = make_calendar(month, year);
    let day_clone = may.days.clone();
    let day_cols: Vec<Vec<&Day>> = vec![
        get_days(WeekDay::Sun, &day_clone),
        get_days(WeekDay::Mon, &day_clone),
        get_days(WeekDay::Tue, &day_clone),
        get_days(WeekDay::Wed, &day_clone),
        get_days(WeekDay::Thu, &day_clone),
        get_days(WeekDay::Fri, &day_clone),
        get_days(WeekDay::Sat, &day_clone),
    ];

    context.insert("month", &may.month);
    context.insert("year", &may.year);
    context.insert("day_cols", &day_cols);
    context.insert("prev_month", &may.prev_month);
    context.insert("next_month", &may.next_month);

    Html(
        state
            .templates
            .render(&*(path_frag + ".html"), &context)
            .unwrap(),
    )
}

fn get_days(week_day: WeekDay, days: &Vec<Day>) -> Vec<&Day> {
    days.iter().filter(|day| day.week_day == week_day).collect()
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
