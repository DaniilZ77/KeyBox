use crate::{errors::AppError, models::Secret};
use sled::Db;
use std::vec;

pub struct RepositoryHelper {
    db: Db,
}

impl RepositoryHelper {
    pub fn new() -> sled::Result<RepositoryHelper> {
        let db = sled::open("key_box_db")?;
        Ok(RepositoryHelper { db })
    }

    pub async fn save_secret(
        &self,
        old_item: Option<&Secret>,
        new_item: &Secret,
    ) -> Result<(), AppError> {
        let old = match old_item {
            Some(item) => Some(serde_json::to_vec(item)?),
            None => None,
        };
        let new = Some(serde_json::to_vec(new_item)?);
        self.db
            .compare_and_swap(Self::get_secret_db_key(&new_item.key), old, new)?
            .map_err(|_| match old_item {
                Some(_) => AppError::Changed("secret".to_string()),
                None => AppError::AlreadyExists("secret".to_string()),
            })?;
        Ok(())
    }

    pub async fn del_secret(&self, key: &str) -> Result<(), AppError> {
        self.db.remove(Self::get_secret_db_key(key))?;
        Ok(())
    }

    pub async fn list_secrets(&self) -> Result<Vec<Secret>, AppError> {
        let mut items: Vec<Secret> = vec![];
        let prefix = Self::get_secret_db_key("");
        for result in self.db.scan_prefix(prefix) {
            let (_, value) = result?;
            let item: Secret = serde_json::from_slice(&value)?;
            items.push(item);
        }
        Ok(items)
    }

    pub async fn get_secret(&self, key: &str) -> Result<Secret, AppError> {
        match self.db.get(Self::get_secret_db_key(key))? {
            Some(value) => Ok(serde_json::from_slice(&value)?),
            None => Err(AppError::NotFound("secret".to_string())),
        }
    }

    fn get_secret_db_key(key: &str) -> String {
        format!("key_box/secrets/{}", key)
    }
}
