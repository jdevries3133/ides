use super::nav::{nav_helper, AdminNav};
use crate::prelude::*;
use axum::http::StatusCode;
use ides::auth::{Role, Token};

struct OverviewPage<'a> {
    existing_tokens: &'a [DisplayToken],
}
impl Component for OverviewPage<'_> {
    fn render(&self) -> String {
        let action_route = Route::AdminToken;

        let rendered_tokens =
            self.existing_tokens
                .iter()
                .fold(String::new(), |mut acc, tok| {
                    let id = tok.id;
                    let name = clean(&tok.name);
                    let revoke = Route::AdminRevokeToken { token_id: Some(id) };
                    acc.push_str(&format!(
                        r#"
                <p>{id}</p>
                <p>{name}</p>
                <button
                    hx-delete="{revoke}"
                    hx-target="body"
                    class="bg-red-500 hover:bg-red-600 rounded p-2"
                >
                    Revoke
                </button>
                "#
                    ));
                    acc
                });

        format!(
            r#"
            <form hx-post="{action_route}" hx-target="body">

                <h2 class="text-xl">Create Token</h2>

                <label for="name">Name</label>
                <input
                    type="text"
                    id="name"
                    name="name"
                    placeholder="Name"
                />
                <button class="bg-green-200 dark:bg-green-700 dark:hover:bg-green-800 hover:bg-green-300 rounded p-2">
                    Submit
                </button>

            </form>
            <h2 class="text-xl">Existing Tokens</h2>
            <div class="grid gap-2 grid-cols-3 max-w-prose">
                {rendered_tokens}
            </div>
            "#
        )
    }
}

pub async fn manage_tokens(
    State(AppState { db }): State<AppState>,
    Query(params): Query<PaginationParams>,
    headers: HeaderMap,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => {
            let tokens = db_load_tokens(&db, &params.into()).await?;
            Ok(Page {
                title: "Manage Tokens",
                children: &PageContainer {
                    children: &OverviewPage {
                        existing_tokens: &tokens,
                    },
                },
            }
            .render()
            .into_response())
        }
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

struct AfterCreatePage<'a> {
    name: &'a str,
    token: &'a Token,
}
impl Component for AfterCreatePage<'_> {
    fn render(&self) -> String {
        let name = clean(self.name);
        let token = clean(self.token.display_secret_value());
        let back = Route::AdminToken;
        format!(
            r#"
            <div class="prose dark:prose-invert">
                <h1>Token Created</h1>
                <p>Here is a token for {name}:</p>
                <p class="italic text-sm bg-yellow-100 dark:bg-yellow-700 inline-block rounded p-1">
                    You won't be able to view this token again after you leave
                    this page.
                </p>
                <pre>{token}</pre>
                <a href="{back}">Go Back</a>
            </div>
            "#
        )
    }
}
#[derive(Deserialize)]
pub struct Payload {
    name: String,
}

pub async fn handle_create_token(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
    Form(Payload { name }): Form<Payload>,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => {
            let token = ides::auth::Token::create()?;
            db_persist_reader_token(&db, &name, &token).await?;
            Ok(Page {
                title: "Manage Tokens",
                children: &PageContainer {
                    children: &AfterCreatePage {
                        name: &name,
                        token: &token,
                    },
                },
            }
            .render()
            .into_response())
        }
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

pub async fn handle_revoke_token(
    State(AppState { db }): State<AppState>,
    Query(params): Query<PaginationParams>,
    Path(token_id): Path<i32>,
    headers: HeaderMap,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => {
            let delete_result = db_delete_token(&db, token_id).await;
            match delete_result {
                Ok(_) => {
                    let tokens = db_load_tokens(&db, &params.into()).await?;
                    Ok(Page {
                        title: "Manage Tokens",
                        children: &PageContainer {
                            children: &OverviewPage {
                                existing_tokens: &tokens,
                            },
                        },
                    }
                    .render()
                    .into_response())
                }
                Err(e) => {
                    if *e.peek() == ErrT::TokenInUse {
                        Ok((
                            StatusCode::NOT_IMPLEMENTED,
                            [
                                "Error: revocation of a token that has ",
                                "been used supported, because the token is ",
                                "referenced by the access log. The ",
                                "quickest hack to revoke access is to ",
                                "change the token value, which Jack can do ",
                                "on the backend. To fully support this we ",
                                "need to think about how to modify the ",
                                "access log after token revocation. ",
                                "Probably, we need to implement ",
                                "soft-deletion of tokens (basically flag a ",
                                "token as revoked but don't delete it), ",
                                "which is a bit of a pain. Refresh the ",
                                "page to dismiss this error.",
                            ]
                            .join(""),
                        )
                            .into_response())
                    } else {
                        Ok(e.into_response())
                    }
                }
            }
        }
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

struct DisplayToken {
    id: i32,
    name: String,
}

async fn db_load_tokens(
    db: impl PgExecutor<'_>,
    pagination: &SqlPagination,
) -> Result<Vec<DisplayToken>> {
    query_as!(
        DisplayToken,
        "select id, name from token order by name desc limit $1 offset $2",
        pagination.limit,
        pagination.offset
    )
    .fetch_all(db)
    .await
    .map_err(|e| ErrStack::sqlx(&e, "db_load_tokens"))
}

async fn db_persist_reader_token(
    db: impl PgExecutor<'_>,
    name: &str,
    token: &Token,
) -> Result<()> {
    let role_id: i32 = Role::Reader.into();
    query!(
        "insert into token
        (
                name,
                token_digest,
                role_id
        ) values ($1, $2, $3)",
        name,
        token.sha256_hex(),
        role_id
    )
    .execute(db)
    .await
    .map_err(|e| ErrStack::sqlx(&e, "db_persist_reader_token"))?;
    Ok(())
}

async fn db_delete_token(db: impl PgExecutor<'_>, token_id: i32) -> Result<()> {
    query!("delete from token where id = $1", token_id)
        .execute(db)
        .await
        .map_err(|e| {
            let stack = ErrStack::sqlx(&e, "db_delete_token");
            if let Some(db_err) = e.as_database_error() {
                if let Some(constraint) = db_err.constraint() {
                    if constraint == "access_log_token_id_fkey" {
                        stack.wrap(ErrT::TokenInUse)
                    } else {
                        stack
                    }
                } else {
                    stack
                }
            } else {
                stack
            }
        })?;
    Ok(())
}
