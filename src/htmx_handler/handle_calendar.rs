use std::{collections::HashMap, str::FromStr};

use axum::response::Html;
use tera::Context;

use crate::{
    calendar::{Day, Month, WeekDay, make_calendar},
    dawn_server::AppState,
    htmx_handler::uppercase_first_letter,
};

pub async fn handle_calendar(
    state: &AppState,
    path_frag: String,
    params: &HashMap<String, String>,
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
            .render(&(path_frag + ".html"), &context)
            .unwrap(),
    )
}
fn get_days(week_day: WeekDay, days: &[Day]) -> Vec<&Day> {
    days.iter().filter(|day| day.week_day == week_day).collect()
}
