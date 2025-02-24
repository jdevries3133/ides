use super::nav::{nav_helper, AdminNav};
use crate::prelude::*;

pub async fn home(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> Result<Response> {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => Ok(Page {
            title: "Admin Home",
            children: &PageContainer { children: &Home {} },
        }
        .render()
        .into_response()),
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

struct Home;
impl Component for Home {
    fn render(&self) -> String {
        let import = Route::AdminImportBook;
        let change_rev = Route::AdminChangeRevision;
        format!(
            r#"
            <div class="flex flex-col">
                <a class="link" href="{import}">Import Book</a>
                <a class="link" href="{change_rev}">Change Current Revision</a>
            </div>
            "#
        )
    }
}
