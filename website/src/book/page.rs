//! Pagination through the block-based book content. One important goal for
//! pagination is to keep the reader's state consistent while the book is
//! revised, allowing the author to publish updates continuously.
//!
//! We have a few pieces of information which contribute to knowing where the
//! reader is in the book;
//!
//! - sequence # of the current block in the current revision
//! - content checksum of the current paragraph
//! - % of progress through the whole book (can be computed)
//!
//! If the book was static, the sequence # of the current block would be
//! good enough for pagination; you just read all the blocks in order - duh.
//! We need to use the other pieces of information to try to keep the reader's
//! place through continuous content updates, though.
//!
//! # Normal Pagination Strategy
//!
//! Normally, we can simply iterate through blocks in sequence order; easy
//! peasy.
//!
//! # Content Update Strategy
//!
//! When the content has changed, we shouldn't use the sequence number. We
//! can assume that block insertion and deletion happens in some region of the
//! book in 100% of content updates, and that means that on average there is a
//! 50% chance that the modification has happened before the reader's current
//! position in the book, thereby causing a content update to potentially
//! launch them to a dramatically different place in the book if we simply
//! use the sequence number. So, moving across content updates, we'll consider
//! the sequence number useless.
//!
//! We also should not take content checksum for granted as a perfect
//! identifier. After all, a book can contain blocks with the same content
//! checksum, because it can contain blocks with the same content.
//!
//! ## The Import Operation
//!
//! An admin performs a book import by pasting the plain-text content of the
//! book into the import page. At this time, we parse the plain text into
//! blocks and insert the parsed content into the database. See
//! [ides::content::Book::from_raw_plain_text]. At this time, checksums of
//! content for all blocks is also computed in the database.
//!
//! ## The Update Operation
//!
//! From the admin perspective, the update operation will happen when we toggle
//! the live version of the same book from one revision to another. At update
//! time, we will find the set of checksums with a 1:1 mapping between
//! revisions. In other words, a checksum which exists exactly once in both
//! versions can be used to map positions in-between versions.
//!
//! ## Page Update Algorithm
//!
//! The reader's current page is a pointer to a block. So, the goal is to move
//! their pointer to the best block in the new revision.
//!
//! [wip]
