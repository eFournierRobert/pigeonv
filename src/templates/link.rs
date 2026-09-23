use askama::Template;

/// Page shown after creating a paste, carrying the share link.
#[derive(Template)]
#[template(path = "link.html")]
pub struct LinkTemplate {
    pub uuid: String,
}
