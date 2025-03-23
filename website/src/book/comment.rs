use crate::{htmx, prelude::*};
use axum::http::HeaderValue;

pub async fn comment(
    State(AppState { db }): State<AppState>,
    Path(block_id): Path<i32>,
    headers: HeaderMap,
) -> Result<Response> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(_) => {
            struct Qres {
                content: String,
            }
            let Qres { content } = query_as!(
                Qres,
                "select content from block where id = $1",
                block_id
            )
            .fetch_one(&db)
            .await
            .map_err(|e| ErrStack::sqlx(&e, "render comment form"))?;
            Ok(Page {
                title: "Comment",
                children: &PageContainer {
                    children: &CommentForm {
                        block_id,
                        block_content: &content,
                    },
                },
            }
            .render()
            .into_response())
        }
        AuthResult::NotAuthenticated => {
            Ok(htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response())
        }
        AuthResult::Err(e) => Err(e),
    }
}

struct CommentForm<'a> {
    block_id: i32,
    block_content: &'a str,
}
impl Component for CommentForm<'_> {
    fn render(&self) -> String {
        let comment = Route::BookComment {
            block_id: Some(self.block_id),
        };
        let block_content = clean(self.block_content);
        format!(
            r#"
            <form class="flex flex-col gap-2" hx-post="{comment}">
                <h1 class="text-xl">Leave a Comment</h1>
                <blockquote
                    class="italic border-l-2 border-stone-300
                    dark:border-stone-700 pl-2"
                >
                    {block_content}
                </blockquote>
                <label for="comment">comment</label>
                <textarea
                    id="comment"
                    name="comment"
                ></textarea>
                <button
                    class="bg-orange-500 text-white font-bold self-start p-2 m-2
                    rounded"
                >
                    save
                </button>
            </form>
            "#
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct Payload {
    pub comment: String,
}

pub async fn handle_comment(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
    Path(block_id): Path<i32>,
    Form(payload): Form<Payload>,
) -> Result<Response> {
    match Auth::from_headers(&db, &headers).await {
        AuthResult::Authenticated(auth) => {
            query!(
                "insert into comment (comment, block_id, token_id) values ($1, $2, $3)",
                payload.comment,
                block_id,
                auth.token_id
            )
            .execute(&db)
            .await
            .map_err(|e| {
                ErrStack::sqlx(&e, "inserting a comment into the DB")
            })?;
            let mut headers = HeaderMap::new();
            headers.insert(
                "Hx-Push-Url",
                HeaderValue::from_str(&Route::Book.as_string())
                    .expect("book route is ASCII"),
            );
            Ok((
                headers,
                [
                    super::ui::render(
                        &auth,
                        &db,
                        &super::ui::get_current_position(&auth, &db).await?,
                    )
                    .await?,
                    Saved {
                        message: "comment saved",
                    }
                    .render(),
                ]
                .join(""),
            )
                .into_response())
        }
        AuthResult::NotAuthenticated => {
            Ok(htmx::redirect(HeaderMap::new(), &Route::Auth.as_string())
                .into_response())
        }
        AuthResult::Err(e) => Err(e),
    }
}
