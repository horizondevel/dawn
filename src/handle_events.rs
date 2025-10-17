use axum::Form;
pub struct PostEventForm {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}

pub struct PostEventResponse {
    pub name: String,
    pub start_time: String,
    pub end_time: String,
}
pub async fn handle_events(
    Form(event_form): Form<PostEventForm>,
) -> Result<PostEventResponse, String> {
    let response = PostEventResponse {
        name: event_form.name,
        start_time: event_form.start_time,
        end_time: event_form.end_time,
    };
    Ok(response)
}
