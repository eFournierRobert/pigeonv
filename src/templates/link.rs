use askama::Template;

#[derive(Template)]
#[template(path = "link.html")]
pub struct LinkTemplate {
    pub uuid: String,
}
