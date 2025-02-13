use crate::{
    auth::{parse_from_headers, Auth},
    prelude::*,
};

pub async fn log_access(
    db: impl PgExecutor<'_> + Copy,
    page: i32,
    headers: &HeaderMap,
) -> Result<()> {
    let token =
        parse_from_headers(headers).map_err(|e| e.wrap(ErrT::AccessLog))?;
    let auth = Auth::get(db, &token)
        .await
        .map_err(|e| e.wrap(ErrT::AccessLog))?;

    query!(
        "insert into access_log (page, token_id)
        values ($1, $2)",
        page,
        auth.token_id
    )
    .execute(db)
    .await
    .map_err(|e| {
        ErrStack::default()
            .wrap(ErrT::SqlxError)
            .ctx(format!("query error while recording log access: {e}"))
    })?;

    Ok(())
}
