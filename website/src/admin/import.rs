use super::nav::{nav_helper, AdminNav};
use crate::prelude::*;

pub async fn import_book_ui(
    State(AppState { db }): State<AppState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    match nav_helper(Auth::from_headers(&db, &headers).await) {
        AdminNav::IsAdmin => Ok(Page {
            title: "Import Book",
            children: &PageContainer {
                children: &ImportBook {},
            },
        }
        .render()
        .into_response()),
        AdminNav::GetOuttaHere(response) => Ok(response),
        AdminNav::Err(e) => Err(e),
    }
}

struct ImportBook;
impl Component for ImportBook {
    fn render(&self) -> String {
        let handler = Route::AdminImportBook;
        format!(
            r#"
            <form class="flex flex-col gap-2" hx-post="{handler}">
                <label for="content">Content</label>
                <textarea id="content" name="content"></textarea>
                <button>save</button>
            </form>
            "#
        )
    }
}

#[derive(Deserialize)]
pub struct Payload {
    content: String,
}

pub async fn handle_import_book(
    Form(Payload { content }): Form<Payload>,
) -> Result<impl IntoResponse> {
    println!("{content}");
    Ok("OK")
}
