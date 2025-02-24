//! Admin UI for importing and updating the book, etc.

mod change_revision;
mod home;
mod import;
mod nav;

pub use change_revision::{change_revision, handle_revision_change};
pub use home::home;
pub use import::{handle_import_book, import_book_ui};
