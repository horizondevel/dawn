mod handle_calendar;
mod handle_timetable;
use crate::htmx_handler::handle_calendar::handle_calendar;
use crate::htmx_handler::handle_timetable::handle_timetable;
use std::collections::HashMap;

use axum::response::Html;
use tera::Context;

use crate::dawn_server::AppState;

pub enum HtmxHandlers {
    Calendar,
    Timetable,
}

pub fn get_handler(path_frag: &str) -> Option<HtmxHandlers> {
    if path_frag.contains("calendar") {
        Some(HtmxHandlers::Calendar)
    } else if path_frag.contains("timetable") {
        Some(HtmxHandlers::Timetable)
    } else {
        None
    }
}

pub async fn handle(
    handler: Option<HtmxHandlers>,
    state: &AppState,
    path_frag: String,
    params: &HashMap<String, String>,
) -> Html<String> {
    match handler {
        Some(HtmxHandlers::Calendar) => handle_calendar(state, path_frag, params).await,
        Some(HtmxHandlers::Timetable) => handle_timetable(state, path_frag, params).await,
        _ => {
            let mut context = Context::new();
            params.iter().for_each(|(k, v)| {
                context.insert(k, &uppercase_first_letter(v));
            });
            Html(
                state
                    .templates
                    .render(&(path_frag + ".html"), &context)
                    .unwrap(),
            )
        }
    }
}
pub fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
