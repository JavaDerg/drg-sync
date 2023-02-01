mod client;
mod room;
mod room_mng;

use crate::room_mng::{RoomManager, RoomManagerHandle};
use axum::extract::{Path, State, WebSocketUpgrade};
use axum::handler::HandlerWithoutStateExt;
use axum::response::Response;
use axum::routing::get;
use axum::{Router, Server};
use eyre::eyre;
use std::env;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{fmt, EnvFilter};
use uuid::Uuid;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let _ = dotenv::dotenv();
    // setup logging
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let manager = RoomManager::new();

    let router = Router::new()
        .route("/join/:room_id", get(join_room))
        .layer(TraceLayer::new_for_http())
        .with_state(manager);

    let bind = env::var("LISTEN_ON").map_err(|_| eyre!("LISTEN_ON must be set"))?;

    Server::bind(&bind.parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

pub async fn join_room(
    State(rm): State<RoomManagerHandle>,
    ws: WebSocketUpgrade,
    Path(room_id): Path<String>,
) -> Response {
    ws.on_upgrade(
        move |socket| async move { rm.join_or_make(room_id, Uuid::new_v4(), socket).await },
    )
}
