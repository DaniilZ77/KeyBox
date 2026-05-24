use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Secret {
    pub key: String,
    pub value: String,
    pub groups: Vec<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

impl Secret {
    fn now() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn validate_key(key: &str) -> Result<(), AppError> {
        if key.len() < 3 {
            return Err(AppError::ValidationError("key too short".to_string()));
        }
        if !key.chars().all(|c| c.is_alphanumeric()) {
            return Err(AppError::ValidationError(
                "key contains invalid characters".to_string(),
            ));
        }
        Ok(())
    }

    pub fn from(req: CreateSecretReq) -> Result<Secret, AppError> {
        Self::validate_key(&req.key)?;
        let now = Self::now();
        Ok(Secret {
            key: req.key,
            value: req.value,
            groups: req.groups,
            created_at: now,
            updated_at: now,
        })
    }

    pub fn update(&mut self, upd: UpdateSecretReq) {
        if let Some(new_value) = upd.value {
            self.value = new_value;
        }
        if let Some(new_groups) = upd.groups {
            self.groups = new_groups;
        }
        self.updated_at = Self::now();
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateSecretReq {
    pub key: String,
    pub value: String,
    pub groups: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct UpdateSecretReq {
    pub value: Option<String>,
    pub groups: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Claims {
    pub exp: usize,
    pub group: String,
    pub admin: bool,
}
