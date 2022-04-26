mod asset;
mod template;

use std::{net::SocketAddr, sync::Arc};

use askama::Template;
use axum::{
    extract::Extension,
    handler::Handler,
    http::{StatusCode, Uri},
    response::IntoResponse,
    routing::get,
    Router,
};
use blossom_config::Config;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

use crate::{asset::StaticFile, template::HtmlTemplate};

pub async fn listen(addr: SocketAddr, pg: PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let config = Arc::new(Config::load().await?);

    let router = Router::new()
        .route("/", get(index))
        .route("/dist/*file", static_handler.into_service())
        .layer(TraceLayer::new_for_http())
        .fallback(not_found.into_service())
        .layer(Extension(pg))
        .layer(Extension(config));

    tracing::info!("Web server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Failed to bind to address");

    Ok(())
}

async fn static_handler(uri: Uri) -> impl IntoResponse {
    let mut path = uri.path().trim_start_matches('/').to_string();

    if path.starts_with("dist/") {
        path = path.replace("dist/", "");
    }

    StaticFile(path)
}

async fn index(Extension(config): Extension<Arc<Config>>) -> impl IntoResponse {
    let template = IndexTemplate {
        title: "Home",
        game_name: config.game.name.clone(),
    };
    HtmlTemplate(template)
}

async fn not_found(Extension(config): Extension<Arc<Config>>) -> impl IntoResponse {
    let template = NotFoundTemplate {
        title: "Not Found",
        game_name: config.game.name.clone(),
    };
    (StatusCode::NOT_FOUND, HtmlTemplate(template))
}

#[derive(Template)]
#[template(path = "404.html")]
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
