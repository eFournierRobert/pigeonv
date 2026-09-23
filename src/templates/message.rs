use askama::Template;

/// Page showing the paste's value.
#[derive(Template)]
#[template(path = "message.html")]
pub struct MessageTemplate {
    pub message: String,
}
