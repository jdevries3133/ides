//! Admin UI for importing and updating the book, etc.

mod change_revision;
mod home;
mod import;
mod manage_token;
mod nav;

pub use change_revision::{change_revision, handle_revision_change};
pub use home::home;
pub use import::{handle_import_book, import_book_ui};
pub use manage_token::{
    handle_create_token, handle_revoke_token, manage_tokens,
};
