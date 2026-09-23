use askama::Template;

/// The paste creation form.
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate;
