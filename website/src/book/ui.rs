//! Reader UI

use super::access::log_access;
use crate::{htmx, prelude::*};
use ides::content::SequencedBlock;

#[derive(Deserialize)]
pub struct Params {
    #[allow(dead_code)]
    /// On the client, this is set to the product of `window.innerHeight` &
    /// `window.innerWidth`.
    screen_area: i32,
}

pub async fn ui(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
    _params: Query<Params>,
) -> Result<impl IntoResponse> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(auth) => {
            let position = get_current_position(&auth, &db).await?;
            render(&auth, &db, &position).await
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
) -> Result<Response> {
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
        title: "Ides of August",
        children: &PageContainer {
            children: &Reader {
                reader_name: &auth.name,
                blocks: &blocks,
                position,
            },
        },
    }
    .render()
    .into_response())
}

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
    .map_err(|e| ErrStack::sqlx(e, "get_current_position"))?;

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
        ).fetch_one(db).await.map_err(|e| ErrStack::sqlx(e, "get_current_position :: current revision does not exist in the first place"))
        }
    }
}

struct Reader<'a> {
    reader_name: &'a str,
    blocks: &'a [SequencedBlock],
    position: &'a CurrentPosition,
}
impl Component for Reader<'_> {
    fn render(&self) -> String {
        let next = Route::BookNextPage;
        let prev = Route::BookPrevPage;
        let reader_name = clean(self.reader_name);
        let content = self
            .blocks
            .iter()
            .filter(|b| b.sequence > self.position.current_block_sequence)
            .take(5)
            .fold(String::new(), |mut acc, block| {
                acc.push_str(&format!(
                    "<p>{}</p>",
                    clean(&block.block.content)
                ));
                acc
            });
        format!(
            r#"
            <div class="w-full h-full">
            <div class="prose dark:text-slate-100">{content}</div>
            <button hx-get="{prev}" hx-target="body">previous page</button>
            <button hx-get="{next}" hx-target="body">next page</button>
            <p>reading as {reader_name}</p>
            </div>
            "#
        )
    }
}
