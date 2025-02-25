//! The book!

mod access;
mod comment;
mod page;
mod ui;

pub use comment::{comment, handle_comment};
pub use page::{next_page, prev_page};
pub use ui::ui;
