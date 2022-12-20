use askama::Template;
use axum::{http::StatusCode, response::IntoResponse};

pub struct Html<T>(pub T);

impl<T> IntoResponse for Html<T>
where
    T: Template,
{
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => axum::response::Html(html).into_response(),
            Err(e) => {
                tracing::error!("Failed to render template: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
