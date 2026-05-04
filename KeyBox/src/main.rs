mod handlers;
mod models;
mod repository;
mod errors;
mod encryption;
mod middlewares;
use axum::{Router, middleware, routing::{delete, get, post, put}};
use handlers::*;
use middlewares::*;
use std::env;
use tracing_subscriber;
use tracing::info;
use base64::{engine::general_purpose::STANDARD, Engine as _};
use axum_prometheus::PrometheusMetricLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let (prometheus_layer, metric_handle) = PrometheusMetricLayer::pair();

    let encoded_secret_key = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let secret_key = STANDARD
        .decode(encoded_secret_key)
        .expect("cannot decode base64 SECRET_KEY");

    let state = AppState::new(String::from_utf8(secret_key).unwrap());

    let public_routes = Router::new()
        .route("/ping", get(pong));

    let protected_routes = Router::new()
        .route("/secrets", post(create_secret))
        .route("/secrets", get(list_secrets))
        .route("/secrets/{key}", delete(delete_secret))
        .route("/secrets/{key}", get(get_secret))
        .route("/secrets/{key}", put(update_secret))
        .layer(middleware::from_fn_with_state(state.clone(), jwt_middleware));

    let app = protected_routes
        .merge(public_routes)
        .route("/metrics", get(move || async move { metric_handle.render() }))
        .layer(prometheus_layer)
        .layer(middleware::from_fn(request_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .unwrap();

    info!("Server running on 0.0.0.0:8080");
    axum::serve(listener, app)
        .await
        .unwrap();
}