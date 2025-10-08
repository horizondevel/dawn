use crate::calendar::{Calendar, Day, Month, WeekDay};
use axum::extract::{Path, Query, State};
use axum::response::Html;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
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

    let month = Month::May;
    let year = 2017;
    let days = vec![
        Day {
            month: Month::April,
            day: 30,
            week_day: WeekDay::Sun,
        },
        Day {
            month: Month::May,
            day: 1,
            week_day: WeekDay::Mon,
        },
        Day {
            month: Month::May,
            day: 2,
            week_day: WeekDay::Tues,
        },
        Day {
            month: Month::May,
            day: 3,
            week_day: WeekDay::Wed,
        },
        Day {
            month: Month::May,
            day: 4,
            week_day: WeekDay::Thurs,
        },
        Day {
            month: Month::May,
            day: 5,
            week_day: WeekDay::Fri,
        },
        Day {
            month: Month::May,
            day: 6,
            week_day: WeekDay::Sat,
        },
        Day {
            month: Month::May,
            day: 7,
            week_day: WeekDay::Sun,
        },
        Day {
            month: Month::May,
            day: 8,
            week_day: WeekDay::Mon,
        },
        Day {
            month: Month::May,
            day: 9,
            week_day: WeekDay::Tues,
        },
        Day {
            month: Month::May,
            day: 10,
            week_day: WeekDay::Wed,
        },
        Day {
            month: Month::May,
            day: 11,
            week_day: WeekDay::Thurs,
        },
        Day {
            month: Month::May,
            day: 12,
            week_day: WeekDay::Fri,
        },
        Day {
            month: Month::May,
            day: 13,
            week_day: WeekDay::Sat,
        },
        Day {
            month: Month::May,
            day: 14,
            week_day: WeekDay::Sun,
        },
        Day {
            month: Month::May,
            day: 15,
            week_day: WeekDay::Mon,
        },
        Day {
            month: Month::May,
            day: 16,
            week_day: WeekDay::Tues,
        },
        Day {
            month: Month::May,
            day: 17,
            week_day: WeekDay::Wed,
        },
        Day {
            month: Month::May,
            day: 18,
            week_day: WeekDay::Thurs,
        },
        Day {
            month: Month::May,
            day: 19,
            week_day: WeekDay::Fri,
        },
        Day {
            month: Month::May,
            day: 20,
            week_day: WeekDay::Sat,
        },
        Day {
            month: Month::May,
            day: 21,
            week_day: WeekDay::Sun,
        },
        Day {
            month: Month::May,
            day: 22,
            week_day: WeekDay::Mon,
        },
        Day {
            month: Month::May,
            day: 23,
            week_day: WeekDay::Tues,
        },
        Day {
            month: Month::May,
            day: 24,
            week_day: WeekDay::Wed,
        },
        Day {
            month: Month::May,
            day: 25,
            week_day: WeekDay::Thurs,
        },
        Day {
            month: Month::May,
            day: 26,
            week_day: WeekDay::Fri,
        },
        Day {
            month: Month::May,
            day: 27,
            week_day: WeekDay::Sat,
        },
        Day {
            month: Month::May,
            day: 28,
            week_day: WeekDay::Sun,
        },
        Day {
            month: Month::May,
            day: 29,
            week_day: WeekDay::Mon,
        },
        Day {
            month: Month::May,
            day: 30,
            week_day: WeekDay::Tues,
        },
        Day {
            month: Month::May,
            day: 31,
            week_day: WeekDay::Wed,
        },
        Day {
            month: Month::June,
            day: 1,
            week_day: WeekDay::Thurs,
        },
        Day {
            month: Month::June,
            day: 2,
            week_day: WeekDay::Fri,
        },
        Day {
            month: Month::June,
            day: 3,
            week_day: WeekDay::Sat,
        },
    ];
    let day_clone = days.clone();
    let day_cols: Vec<Vec<&Day>> = vec![
        get_days(WeekDay::Sun, &day_clone),
        get_days(WeekDay::Mon, &day_clone),
        get_days(WeekDay::Tues, &day_clone),
        get_days(WeekDay::Wed, &day_clone),
        get_days(WeekDay::Thurs, &day_clone),
        get_days(WeekDay::Fri, &day_clone),
        get_days(WeekDay::Sat, &day_clone),
    ];

    let may = Calendar { month, year, days };
    context.insert("month", &may.month);
    context.insert("year", &may.year);
    context.insert("day_cols", &day_cols);

    println!("{:?}", context);
    Html(
        state
            .templates
            .render(&*(path_frag + ".html"), &context)
            .unwrap(),
    )
}

fn get_days(week_day: WeekDay, days: &Vec<Day>) -> Vec<&Day> {
    days.iter()
        .filter(|day| day.week_day == week_day)
        .collect()
}

fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
