use crate::errors::AppError;
use crate::models::{CreateSecretReq, Secret, UpdateSecretReq};
use crate::repository::*;
use crate::{encryption::Encryption, models::Claims};
use axum::{
    extract::{Extension, Json, Path, State},
    http::StatusCode,
};
use jsonwebtoken::DecodingKey;
use metrics::counter;
use std::sync::Arc;
use tracing::*;

#[derive(Clone)]
pub struct AppState {
    pub repository_helper: Arc<RepositoryHelper>,
    pub encryption: Arc<Encryption>,
    pub secret: Arc<DecodingKey>,
}

impl AppState {
    pub fn new(secret_key: String) -> AppState {
        let rh = RepositoryHelper::new().expect("cannot create RepositoryHelper");
        let encryption = Encryption::new(&secret_key);
        let secret = DecodingKey::from_secret(secret_key.as_ref());

        AppState {
            repository_helper: Arc::new(rh),
            encryption: Arc::new(encryption),
            secret: Arc::new(secret),
        }
    }

    fn mask_secret(&self, item: &mut Secret) -> Result<(), AppError> {
        item.value = self.encryption.encrypt(&item.value)?;
        Ok(())
    }

    fn unmask_secret(&self, item: &mut Secret) -> Result<(), AppError> {
        item.value = self.encryption.decrypt(&item.value)?;
        Ok(())
    }
}

pub async fn pong() -> &'static str {
    "pong"
}

pub async fn create_secret(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Json(req): Json<CreateSecretReq>,
) -> Result<(StatusCode, Json<Secret>), AppError> {
    if !claims.admin {
        return Err(AppError::NotEnoughRights());
    }

    let item = Secret::from(req)?;

    let mut to_save = item.clone();
    state.mask_secret(&mut to_save)?;

    state.repository_helper.save_secret(None, &to_save).await?;

    info!("Successfully created secret! {}", item.key);

    counter!("secrets_created_total").increment(1);

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update_secret(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(req): Json<UpdateSecretReq>,
) -> Result<(StatusCode, Json<Secret>), AppError> {
    if !claims.admin {
        return Err(AppError::NotEnoughRights());
    }

    if req.value.is_none() && req.groups.is_none() {
        return Err(AppError::ValidationError("nothing to update".to_string()));
    }

    let old_item = state.repository_helper.get_secret(&key).await?;

    let mut new_item = old_item.clone();
    new_item.update(req);

    let mut to_save = new_item.clone();
    state.mask_secret(&mut to_save)?;

    state
        .repository_helper
        .save_secret(Some(&old_item), &to_save)
        .await?;

    info!("Successfully updated secret! {}", new_item.key);

    Ok((StatusCode::OK, Json(new_item)))
}

pub async fn list_secrets(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Secret>>), AppError> {
    let items = state.repository_helper.list_secrets().await?;

    let mut result_items: Vec<Secret> = vec![];
    for mut item in items {
        if claims.admin || item.groups.contains(&claims.group) {
            state.unmask_secret(&mut item)?;
            result_items.push(item);
        }
    }

    Ok((StatusCode::OK, Json(result_items)))
}

pub async fn delete_secret(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<StatusCode, AppError> {
    if !claims.admin {
        return Err(AppError::NotEnoughRights());
    }

    state.repository_helper.del_secret(&key).await?;

    info!("Successfully deleted secret! {}", key);

    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_secret(
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<(StatusCode, Json<Secret>), AppError> {
    let mut item = state.repository_helper.get_secret(&key).await?;

    if !claims.admin && !item.groups.contains(&claims.group) {
        return Err(AppError::NotEnoughRights());
    }
    state.unmask_secret(&mut item)?;

    Ok((StatusCode::OK, Json(item)))
}
