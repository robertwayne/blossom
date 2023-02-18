use axum::{
    body::{boxed, Full},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/static"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let body = boxed(Full::from(content.data));
                let mime = mime_guess::from_path(path).first_or_octet_stream();

                let Ok(response) = Response::builder()
                    .header(header::CONTENT_TYPE, mime.as_ref())
                    .body(body) else {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                    };

                response
            }
            None => StatusCode::NOT_FOUND.into_response(),
        }
    }
}
