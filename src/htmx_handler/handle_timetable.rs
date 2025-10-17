use std::collections::HashMap;

use axum::response::Html;
use serde::{Deserialize, Serialize};
use tera::Context;

use crate::{dawn_server::AppState, htmx_handler::uppercase_first_letter};

#[derive(Debug, Serialize, Deserialize)]
struct Time {
    start_time: String,
    end_time: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Day {
    day: String,
    date: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Timetable {
    times: Vec<Time>,
    days: Vec<Day>,
}

pub async fn handle_timetable(
    state: &AppState,
    path_frag: String,
    params: &HashMap<String, String>,
) -> Html<String> {
    let mut context = Context::new();
    params.iter().for_each(|(k, v)| {
        context.insert(k, &uppercase_first_letter(v));
    });
    let mut times: Vec<Time> = Vec::new();
    for i in 7..24 {
        times.push(Time {
            start_time: i.to_string() + ":00",
            end_time: (i + 1).to_string() + ":00",
        });
    }
    let days: Vec<Day> = vec![
        Day {
            day: "30".to_string(),
            date: "2017-04-30".to_string(),
        },
        Day {
            day: "1".to_string(),
            date: "2017-05-01".to_string(),
        },
        Day {
            day: "2".to_string(),
            date: "2017-05-02".to_string(),
        },
        Day {
            day: "3".to_string(),
            date: "2017-05-03".to_string(),
        },
        Day {
            day: "4".to_string(),
            date: "2017-05-04".to_string(),
        },
        Day {
            day: "5".to_string(),
            date: "2017-05-05".to_string(),
        },
        Day {
            day: "6".to_string(),
            date: "2017-05-06".to_string(),
        },
    ];
    let timetable = Timetable { times, days };
    context.insert("times", &timetable.times);
    context.insert("days", &timetable.days);
    Html(
        state
            .templates
            .render(&(path_frag + ".html"), &context)
            .unwrap(),
    )
}
