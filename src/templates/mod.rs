//! Askama templates and shared error rendering.
//!
//! The HTML files live in `templates/` at the repository root (outside `src/`);
//! each struct here is bound to one via `#[template(path = ...)]`.

pub mod error;
pub mod index;
pub mod link;
pub mod message;
