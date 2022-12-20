mod api;
mod asset;
mod template;

use std::sync::Arc;

use askama::Template;
use axum::{
    extract::Extension, handler::HandlerWithoutStateExt, http::Uri, response::IntoResponse,
    routing::get, Router, Server,
};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::config::Config;

use template::Html;

use asset::StaticFile;

pub async fn listen(pg: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::load().await?);
    let addr = config.web_addr();

    let api_route_handler = Router::new().route("/health", get(api::health));

    let app = Router::new()
        .route("/", get(index))
        .nest("/api/v1", api_route_handler)
        .route_service("/static/*file", static_file_handler.into_service())
        .fallback(not_found)
        .layer(Extension(pg))
        .layer(Extension(config))
        .layer(TraceLayer::new_for_http());

    tracing::info!("Web server listening on {}", addr);

    if let Err(e) = Server::bind(&addr).serve(app.into_make_service()).await {
        tracing::info!("Web server stopped\n{}", e);
    }

    Ok(())
}

async fn static_file_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("static/") {
        path = path.replace("static/", "");
    }

    StaticFile(path)
}

async fn index(config: Extension<Arc<Config>>) -> impl IntoResponse {
    let template = IndexTemplate { title: "Home", game_name: config.game.name.clone() };
    Html(template)
}

async fn not_found(config: Extension<Arc<Config>>) -> impl IntoResponse {
    let template = NotFoundTemplate { title: "Not Found", game_name: config.game.name.clone() };
    Html(template)
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate {
    title: &'static str,
    game_name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: &'static str,
    game_name: String,
}
