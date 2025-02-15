use crate::{auth::Auth, prelude::*};

pub async fn log_access(
    auth: &Auth,
    db: impl PgExecutor<'_> + Copy,
    page: i32,
) -> Result<()> {
    query!(
        "insert into access_log (page, token_id)
        values ($1, $2)",
        page,
        auth.token_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        ErrStack::new(ErrT::SqlxError)
            .ctx(format!("query error while recording log access: {e}"))
    })?;

    Ok(())
}
