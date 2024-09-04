//! Run with
//!
//! ```not_rust
//! cargo run -p rust-example
//! ```

use std::net::SocketAddr;

use axum::{
    extract::{connect_info::ConnectInfo, ws::WebSocketUpgrade}, 
    response::{Html, IntoResponse}, 
    routing::get, 
    Router
};
use axum_extra::TypedHeader;
use headers::UserAgent;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,tower_http=debug,axum::rejection=trace", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    
    tracing::info!("Starting server...");

    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/ws", get(ws_handler))
        .layer(TraceLayer::new_for_http());

    // run it
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("listening on {} - http://localhost:{}/", listener.local_addr().unwrap(), listener.local_addr().unwrap().port());
    axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "unknown".to_string()
    };

    tracing::debug!("WebSocket connection from {} with user agent: {}", addr, user_agent);

    ws.on_upgrade(move |socket| async move {
        tracing::debug!("WebSocket connection established, from: {}", addr);
        // handle the WebSocket connection

        // Wait a second.
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        // Close the socket, we don't need it!
        socket.close().await.unwrap();
    })
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}