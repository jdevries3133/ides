use axum::{http::StatusCode, response::IntoResponse};
use std::fmt::Display;

#[derive(Debug)]
pub enum Err {}

struct ErrFrame {
    variant: Err,
    ctx: Option<String>,
}

#[derive(Default)]
pub struct ErrStack {
    stack: Vec<ErrFrame>,
}

impl ErrStack {
    pub fn wrap(mut self, err: Err) -> Self {
        self.stack.push(ErrFrame {
            variant: err,
            ctx: None,
        });
        self
    }
    pub fn because(mut self, ctx: String) -> Self {
        if let Some(last) = self.stack.last_mut() {
            last.ctx = Some(ctx);
        }
        self
    }
}

impl Display for ErrStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Oops! One or more errors occurred;")?;
        let alt = "details not available";
        for (indent, item) in self.stack.iter().enumerate() {
            let indent = "  ".repeat(indent + 1);
            let er_code = &item.variant;
            let ctx = item.ctx.as_ref();
            if let Some(ctx) = ctx {
                writeln!(f, "{indent}{er_code:?} :: {ctx}")?;
            } else {
                writeln!(f, "{indent}{er_code:?} :: {alt}")?;
            }
        }
        Ok(())
    }
}

impl IntoResponse for ErrStack {
    fn into_response(self) -> axum::response::Response {
        eprintln!("{self}");
        (StatusCode::INTERNAL_SERVER_ERROR, "An error occurred").into_response()
    }
}
