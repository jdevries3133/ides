use crate::prelude::*;

pub const PAGE_SIZE: i32 = 3;

#[derive(Debug, Eq, PartialEq)]
pub struct Block {
    pub r#type: BlockType,
    pub content: String,
}

pub struct SequencedBlock {
    pub block: Block,
    pub sequence: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlockType {
    Paragraph,
    H1,
    SectionTitle,
}

impl From<BlockType> for i32 {
    fn from(val: BlockType) -> Self {
        match val {
            BlockType::Paragraph => 1,
            BlockType::H1 => 2,
            BlockType::SectionTitle => 4,
        }
    }
}

impl TryInto<BlockType> for i32 {
    type Error = ErrStack;
    fn try_into(self) -> std::result::Result<BlockType, ErrStack> {
        match self {
            1 => Ok(BlockType::Paragraph),
            2 => Ok(BlockType::H1),
            4 => Ok(BlockType::SectionTitle),
            _ => Err(ErrStack::new(ErrT::Invariant)
                .ctx(format!("{self} is not a valid i32 for BlockType"))),
        }
    }
}

pub enum Direction {
    Back,
    Forward,
}

/// Return 200 blocks adjacent to current_block_id (100 before & 100 after)
/// in sequence order.
pub async fn list_blocks(
    db: impl PgExecutor<'_> + Copy,
    current_sequence: i32,
    book_revision_id: i32,
) -> Result<Vec<SequencedBlock>> {
    struct Qres {
        content: String,
        type_id: i32,
        sequence: i32,
    }
    let mut result = query_as!(
        Qres,
        "select sequence, content, type_id
        from block
        where
            sequence > $1
            and sequence <= $2
            and book_revision_id = $3
        order by sequence
        limit $4",
        current_sequence,
        current_sequence + 3,
        book_revision_id,
        PAGE_SIZE as i64
    )
    .fetch_all(db)
    .await
    .map_err(|e| {
        ErrStack::new(ErrT::SqlxError)
            .ctx(format!("could not list_blocks: {e}"))
    })?;

    let mut x = result.drain(..);
    x.try_fold(Vec::new(), |mut acc, row| {
        acc.push(SequencedBlock {
            sequence: row.sequence,
            block: Block {
                r#type: row.type_id.try_into().map_err(|e: ErrStack| {
                    e.wrap(ErrT::BookUi)
                        .ctx("happened during list_blocks".into())
                })?,
                content: row.content,
            },
        });
        Ok(acc)
    })
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub blocks: Vec<Block>,
}

pub struct PersistedBook {
    pub revision_id: i32,
    pub book: Book,
}

impl Book {
    pub fn from_raw_plain_text(input: &str) -> Self {
        let mut title = String::new();
        let mut current_content: Vec<String> = Vec::new();
        let mut blocks = Vec::new();

        for line in input.lines() {
            let trimmed = line
                .trim()
                // Remove the form feed character, which is how word will
                // export page breaks.
                .replace("\u{000C}", "");

            // Potentially detect the end of a paragraph at an empty line.
            // Continue no matter what.
            if trimmed.is_empty() {
                if !current_content.is_empty() {
                    // End the current content and push it into the block list
                    // as a paragraph.
                    blocks.push(Block {
                        r#type: BlockType::Paragraph,
                        content: current_content.join(" "),
                    });
                    current_content.clear();
                }
                continue;
            }

            // Handle headings
            if trimmed.starts_with('#') {
                let content = trimmed.trim_start_matches('#').trim();
                let block_type =
                    match trimmed.chars().take_while(|c| *c == '#').count() {
                        1 => Some(BlockType::SectionTitle),
                        2 => Some(BlockType::H1),
                        _ => None,
                    };
                match (content.is_empty(), block_type) {
                    (false /* is not empty */, Some(block_type)) => {
                        blocks.push(Block {
                            r#type: block_type,
                            content: content.to_string(),
                        });
                    }
                    _ => { /* noop; ignore junk */ }
                };
                continue;
            }

            // Extract the title. Any title we find overwritees the ones
            // that came before.
            if trimmed.starts_with('%') {
                title = trimmed.trim_start_matches('%').trim().to_string();
                continue;
            }

            // Ignore garbage lines starting with dashes or with only
            // whitespace.
            if trimmed.contains("-----") || trimmed.is_empty() {
                continue;
            }

            current_content.push(trimmed);
        }

        if !current_content.is_empty() {
            // End the current content and push it into the block list
            // as a paragraph.
            blocks.push(Block {
                r#type: BlockType::Paragraph,
                content: current_content.join(" "),
            });
            current_content.clear();
        }

        Book { title, blocks }
    }
    pub async fn persist(
        self,
        db: impl PgExecutor<'_> + Copy,
    ) -> Result<PersistedBook> {
        // Note: for now, we treat the book as a singleton, and don't support
        // multiple books. If a row in books does not exist, we'll be in a
        // broken state.

        query!("update book set title = $1", self.title)
            .execute(db)
            .await
            .map_err(|e| {
                ErrStack::new(ErrT::AdminBook)
                    .ctx(format!("failed to update singleton book title: {e}"))
            })?;

        let Id { id: revision_id } = query_as!(
            Id,
            "insert into book_revision (book_id) values (1) returning id"
        )
        .fetch_one(db)
        .await
        .map_err(|e| {
            ErrStack::new(ErrT::AdminBook)
                .ctx(format!("failed to create a new book: {e}"))
        })?;

        for (seq, block) in self.blocks.iter().enumerate() {
            let block_type_id: i32 = block.r#type.into();
            let seq_i32 = seq as i32;
            query!(
                "insert into block
                (
                    sequence,
                    content,
                    book_revision_id,
                    type_id
                ) values ($1, $2, $3, $4)",
                seq_i32,
                block.content,
                revision_id,
                block_type_id
            )
            .execute(db)
            .await
            .map_err(|e| {
                ErrStack::new(ErrT::AdminBook)
                    .ctx(format!("cannot insert block: {e}"))
            })?;
        }

        Ok(PersistedBook {
            revision_id,
            book: self,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_book() {
        let book = Book::from_raw_plain_text(
            "% Title\n\n# Cool book!\n\n## Great\n\nBook.",
        );
        assert_eq!(book.title, "Title");
        assert_eq!(book.blocks[0].r#type, BlockType::SectionTitle);
        assert_eq!(book.blocks[1].r#type, BlockType::H1);
        assert_eq!(book.blocks[2].r#type, BlockType::Paragraph);
        assert_eq!(book.blocks.len(), 3);

        assert_eq!(book.blocks[0].content, "Cool book!");
        assert_eq!(book.blocks[1].content, "Great");
        assert_eq!(book.blocks[2].content, "Book.");
    }

    #[test]
    fn test_parse_hard_wrapping() {
        let book = Book::from_raw_plain_text(
            "this
is
a
hard-wrapped
paragraph which may have multiple
words per line.

But this is definitely
a
new
paragraph.",
        );

        let expected_blocks = vec![
            Block {
                r#type: BlockType::Paragraph,
                content: "this is a hard-wrapped paragraph which may have multiple words per line.".into()
            },
            Block {
                r#type: BlockType::Paragraph,
                content: "But this is definitely a new paragraph.".into(),
            }
        ];

        assert_eq!(book.blocks, expected_blocks);
        assert_eq!(book.blocks.len(), 2);
    }

    #[test]
    fn test_parse_remove_garbage() {
        // The lines with only dashes, or theoretically headings without
        // contnet, should be ignored.
        let book = Book::from_raw_plain_text(
            "this
is good content

----- 

-----  

#

## 


more good content
",
        );

        assert_eq!(book.blocks.len(), 2);
        assert_eq!(book.blocks[0].content, "this is good content");
        assert_eq!(book.blocks[1].content, "more good content");
        assert!(matches!(book.blocks[0].r#type, BlockType::Paragraph));
        assert!(matches!(book.blocks[1].r#type, BlockType::Paragraph));
    }

    #[test]
    fn test_parse_leading_whitespace() {
        let book = Book::from_raw_plain_text(
            "     Has leading spaces\n\n# Another Cool Book\n\n   Content with leading spaces.",
        );

        assert!(matches!(book.blocks[0].r#type, BlockType::Paragraph));
        assert!(matches!(book.blocks[1].r#type, BlockType::SectionTitle));
        assert_eq!(book.blocks.len(), 3);

        assert_eq!(book.blocks[0].content.trim(), "Has leading spaces");
        assert_eq!(book.blocks[1].content.trim(), "Another Cool Book");
        assert_eq!(
            book.blocks[2].content.trim(),
            "Content with leading spaces."
        );
    }
    #[test]
    fn test_parse_ignore_line_feed() {
        let book = Book::from_raw_plain_text(
            "This\n\nis\n\n\njust\n\nsome\n\n\ncontent.\n\n",
        );

        assert_eq!(book.blocks.len(), 5);
        assert_eq!(book.blocks[0].content, "This");
        assert_eq!(book.blocks[1].content, "is");
        assert_eq!(book.blocks[2].content, "just");
        assert_eq!(book.blocks[3].content, "some");
        assert_eq!(book.blocks[4].content, "content.");
        assert!(matches!(book.blocks[0].r#type, BlockType::Paragraph));
    }

    #[test]
    fn test_ignore_too_much_header() {
        let book = Book::from_raw_plain_text("##### woah");
        assert!(book.blocks.is_empty());
    }
}
