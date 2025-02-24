use super::nav::{nav_helper, AdminNav};
use crate::{components::Saved, prelude::*};
use chrono::{DateTime, Utc};
use chrono_tz::Tz;

pub async fn change_revision(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => {
            let ui = revision_change_ui(&db)
                .await
                .map_err(|e| e.wrap(ErrT::AdminBook).ctx("GET form".into()))?;
            Ok(Page {
                title: "Change Revision",
                children: &PageContainer { children: &ui },
            }
            .render()
            .into_response())
        }
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

async fn revision_change_ui(
    db: impl PgExecutor<'_> + Copy,
) -> Result<impl Component> {
    let current_revision = query_as!(
        Revision,
        "select revision_id id, created_at from current_revision
                join book_revision on id = revision_id
                where current_revision.book_id = 1",
    )
    .fetch_optional(db)
    .await
    .map_err(|e| {
        ErrStack::sqlx(e, "change_revision: fetch current revision")
    })?;
    let revisions =
        query_as!(Revision, "select id, created_at from book_revision")
            .fetch_all(db)
            .await
            .map_err(|e| {
                ErrStack::sqlx(e, "change_revision: fetch all revisions")
            })?;
    Ok(RevisionChangeUI {
        current_revision,
        revisions,
    })
}

struct Revision {
    id: i32,
    created_at: DateTime<Utc>,
}

impl Component for &[Revision] {
    fn render(&self) -> String {
        let revs = self.iter().fold(String::new(), |mut acc, rev| {
            let id = rev.id;
            let time = rev.created_at.with_timezone(&Tz::America__New_York);
            acc.push_str(&format!(
                r#"
                <p class="bold">{id}</p>
                <p>{time}</p>
                "#
            ));
            acc
        });
        format!(
            r#"
            <div class="grid grid-cols-2">
            <p>Revision ID</p>
            <p>Created At</p>
            {revs}
            "#
        )
    }
}

struct RevisionChangeUI {
    revisions: Vec<Revision>,
    current_revision: Option<Revision>,
}
impl Component for RevisionChangeUI {
    fn render(&self) -> String {
        let admin = Route::AdminHome;
        let action = Route::AdminChangeRevision;
        let current_rev = if let Some(ref rev) = self.current_revision {
            let id = rev.id;
            let time = rev.created_at.with_timezone(&Tz::America__New_York);
            &format!(
                r#"
                <p>Current revision ID is <strong>{id}</strong></p>
                <p>Current revision was created <strong>{time}</strong>
                "#
            )
        } else {
            ""
        };
        let rev_field_value = if let Some(ref rev) = self.current_revision {
            &rev.id.to_string()
        } else {
            ""
        };
        let other_revisions = self.revisions.as_slice().render();
        format!(
            r#"
            <div>
                <a class="link" href="{admin}">admin home</a>
                <h1 class="text-xl">Configure Current Revision</h1>
                {current_rev}
                <form class="flex flex-col max-w-md p-4 rounded bg-slate-200 dark:bg-slate-700" hx-post="{action}" hx-target="closest div">
                    <label for="revision">Current Revision</label>
                    <input type="number" id="revision" name="revision" value="{rev_field_value}" />
                    <button>save</button>
                </form>
                <h2 class="text-lg">Other Revisions</h2>
                {other_revisions}
            </div>
            "#
        )
    }
}

#[derive(Deserialize)]
pub struct Payload {
    revision: i32,
}

pub async fn handle_revision_change(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
    Form(Payload {
        revision: revision_id,
    }): Form<Payload>,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => {
            query!(
                "insert into current_revision
                (
                    revision_id,
                    book_id
                ) values ($1, $2)
                on conflict (book_id)
                do update set
                    revision_id = $1",
                revision_id,
                1
            )
            .execute(&db)
            .await
            .map_err(|e| {
                ErrStack::sqlx(e, "handle_revision_change: save new rev")
            })?;

            let ui = revision_change_ui(&db).await?;

            Ok([
                ui.render(),
                Saved {
                    message: "current revision updated",
                }
                .render(),
            ]
            .join("")
            .into_response())
        }
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}
