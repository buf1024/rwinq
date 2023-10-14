use std::{collections::HashMap, time::Duration};

use axum::{
    error_handling::HandleErrorLayer, http::StatusCode, response::IntoResponse, routing::get,
    BoxError, Json, Router,
};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rwinq=debug,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    tracing::info!("started");

    let app = Router::new()
        .route("/", get(index))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );
    // .with_state(db);

    tracing::debug!("listening on 0.0.0.0:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index(// pagination: Option<Query<Pagination>>,
    // State(db): State<Db>,
) -> impl IntoResponse {
    // let todos = db.read().unwrap();

    // let Query(pagination) = pagination.unwrap_or_default();

    // let todos = todos
    //     .values()
    //     .skip(pagination.offset.unwrap_or(0))
    //     .take(pagination.limit.unwrap_or(usize::MAX))
    //     .cloned()
    //     .collect::<Vec<_>>();
    let mut m = HashMap::new();
    m.insert("hello", "world");
    Json(m)
}
