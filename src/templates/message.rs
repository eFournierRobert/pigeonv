use askama::Template;

#[derive(Template)]
#[template(path = "message.html")]
pub struct MessageTemplate {
    pub message: String,
}
