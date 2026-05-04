use uuid::Uuid;
use axum::{body::Body, http::{Request, header}, middleware::Next, response::Response, extract::State};
use tracing::Instrument;
use jsonwebtoken::{Validation, decode};
use crate::{errors::AppError, handlers::AppState, models::Claims};
use metrics::counter;

pub async fn request_id(req: Request<Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    let span = tracing::info_span!(
        "request",
        request_id = %request_id,
        method = %method,
        path = %path
    );

    let response = next
        .run(req)
        .instrument(span)
        .await;

    response
}

pub async fn jwt_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|hv| hv.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or_else(|| {
            counter!(
                "auth_errors_total",
                "reason" => "token_not_provided"
            ).increment(1);
            AppError::Unauthorized("token not provided".to_string())
        })?;

    let token_data = decode::<Claims>(token, &state.secret, &Validation::default())
        .map_err(|e| {
            counter!(
                "auth_errors_total",
                "reason" => "token_not_valid"
            ).increment(1);
            AppError::Unauthorized(e.to_string())
        })?;

    counter!("auth_success_total").increment(1);

    req.extensions_mut().insert(token_data.claims);
    
    Ok(next.run(req).await)
}