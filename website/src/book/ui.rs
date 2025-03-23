//! Reader UI

use super::access::log_access;
use crate::{htmx, prelude::*};
use ides::content::SequencedBlock;

pub async fn ui(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> Result<impl IntoResponse> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(auth) => {
            let position = get_current_position(&auth, &db).await?;
            render(&auth, &db, &position)
                .await
                .map(|c| c.into_response())
        }
        AuthResult::NotAuthenticated => {
            Ok(htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response())
        }
        AuthResult::Err(e) => Err(e),
    }
}

pub async fn render(
    auth: &Auth,
    db: impl PgExecutor<'_> + Copy,
    position: &CurrentPosition,
) -> Result<String> {
    log_access(auth, db, position.current_block_sequence)
        .await
        .map_err(|e| {
            e.wrap(ErrT::BookUi).ctx("while accessing book UI".into())
        })?;

    let blocks = ides::content::list_blocks(
        db,
        position.current_block_sequence,
        position.book_revision_id,
    )
    .await?;

    Ok(Page {
        title: "The Ides of August",
        children: &Reader {
            reader_name: &auth.name,
            blocks: &blocks,
        },
    }
    .render())
}

#[derive(Debug)]
pub struct CurrentPosition {
    pub book_revision_id: i32,
    pub current_block_sequence: i32,
    pub current_block_id: i32,
}

pub async fn get_current_position(
    auth: &Auth,
    db: impl PgExecutor<'_> + Copy,
) -> Result<CurrentPosition> {
    let result = query_as!(
        CurrentPosition,
        "select
            bl.id current_block_id,
            bl.book_revision_id,
            bl.sequence current_block_sequence
        from current_block cb
        join block bl on cb.block_id = bl.id
        where
            token_id = $1
        ",
        auth.token_id
    )
    .fetch_optional(db)
    .await
    .map_err(|e| ErrStack::sqlx(&e, "get_current_position"))?;

    match result {
        Some(r) => Ok(r),
        None => {
        query_as!(
            CurrentPosition,
            "select
                id current_block_id,
                book_revision_id,
                sequence current_block_sequence
            from block
            where
                book_revision_id = (
                    select revision_id
                    from current_revision
                    where book_id = 1
                )
            order by sequence
            limit 1"
        ).fetch_one(db).await.map_err(|e| ErrStack::sqlx(&e, "get_current_position :: current revision does not exist in the first place"))
        }
    }
}

impl Component for SequencedBlock {
    fn render(&self) -> String {
        let comment = Route::BookComment {
            block_id: Some(self.id),
        };
        let content = match self.block.r#type {
            ides::content::BlockType::SectionTitle => {
                format!(
                    r#"<h1 class="text-yellow-400">{}</h1>"#,
                    clean(&self.block.content)
                )
            }
            ides::content::BlockType::H1 => {
                format!(
                    r#"<h2 class="extra-bold text-yellow-400">{}</h2>"#,
                    clean(&self.block.content)
                )
            }
            ides::content::BlockType::Paragraph => {
                format!("<p>{}</p>", clean(&self.block.content))
            }
        };

        format!(
            r#"
            <div
                class="cursor-pointer"
                hx-push-url="true"
                hx-target="body" 
                hx-get="{comment}"
            >
                {content}
            </div>
            "#
        )
    }
}

struct Reader<'a> {
    reader_name: &'a str,
    blocks: &'a [SequencedBlock],
}
impl Component for Reader<'_> {
    fn render(&self) -> String {
        let about = Route::About;
        let reader_name = clean(self.reader_name);

        let content =
            self.blocks.iter().fold(String::new(), |mut acc, block| {
                acc.push_str(&block.render());
                acc
            });
        let toolbar = Toolbar {}.render();
        format!(
            r#"
            <div
                id="reader-container"
                class="bg-stone-50 flex flex-col dark:bg-stone-900
                dark:text-slate-200 h-dvh w-screen overflow-clip"
            >
                <div class="w-screen flex-grow p-2 overflow-y-scroll">
                    <div class="prose sm:p-4 md:p-8 dark:text-slate-200">
                        {content}
                    </div>
                </div>
                <div>
                    <div class="rounded-t flex bg-stone-300 dark:bg-stone-700 px-2">
                        <p>reading as {reader_name}</p>
                        <a class="link flex-grow text-right" href="{about}">
                            about the site
                        </a>
                    </div>
                    {toolbar}
                </div>
            </div>
            "#
        )
    }
}

struct Toolbar;
impl Component for Toolbar {
    fn render(&self) -> String {
        let next = Route::BookNextPage;
        let prev = Route::BookPrevPage;
        let forward = ForwardIcon {}.render();
        let back = BackIcon {}.render();
        format!(
            r##"
            <div class="flex">
            <button
                hx-target="#reader-container"
                hx-get="{prev}"
                class="bg-stone-300 dark:bg-stone-700 flex flex-grow p-1 justify-center items-center
                border-stone-800 border-t-2"
            >{back}</button>
            <button
                class="bg-stone-300 dark:bg-stone-700 flex flex-grow p-1 justify-center items-center
                border-stone-800 border-t-2"
                hx-target="#reader-container"
                hx-get="{next}"
            >{forward}</button>
            </div>
            "##
        )
    }
}

struct ForwardIcon;
impl Component for ForwardIcon {
    fn render(&self) -> String {
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-8">
            <path d="M2.53 3.956A1 1 0 0 0 1 4.804v6.392a1 1 0 0 0 1.53.848l5.113-3.196c.16-.1.279-.233.357-.383v2.73a1 1 0 0 0 1.53.849l5.113-3.196a1 1 0 0 0 0-1.696L9.53 3.956A1 1 0 0 0 8 4.804v2.731a.992.992 0 0 0-.357-.383L2.53 3.956Z" />
        </svg>
        "#.into()
    }
}

struct BackIcon;
impl Component for BackIcon {
    fn render(&self) -> String {
        r#"
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="currentColor" class="size-8">
            <path d="M8.5 4.75a.75.75 0 0 0-1.107-.66l-6 3.25a.75.75 0 0 0 0 1.32l6 3.25a.75.75 0 0 0 1.107-.66V8.988l5.393 2.921A.75.75 0 0 0 15 11.25v-6.5a.75.75 0 0 0-1.107-.66L8.5 7.013V4.75Z" />
        </svg>
        "#.into()
    }
}
