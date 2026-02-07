use async_trait::async_trait;
use chrono::{DateTime, Utc};
use models::DangerCategory;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::{DangerScore, MessageId, ModerationResult, ResultStore, StoreError};

pub struct PostgresModerationRepository {
    pool: PgPool,
}

impl PostgresModerationRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ResultStore for PostgresModerationRepository {
    async fn save(&self, result: &ModerationResult) -> Result<(), StoreError> {
        let categories: Vec<String> = result
            .categories
            .iter()
            .map(|c| match c {
                DangerCategory::RadicalPositions => "radical_positions",
                DangerCategory::AdvertisingSpam => "advertising_spam",
                DangerCategory::Doxxing => "doxxing",
            })
            .map(String::from)
            .collect();

        sqlx::query(
            r#"
            INSERT INTO moderations (id, message, danger_score, categories, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
        )
        .bind(result.id.as_uuid())
        .bind(&result.message)
        .bind(result.danger_score.value())
        .bind(&categories)
        .bind(result.created_at)
        .execute(&self.pool)
        .await
        .map_err(|e| StoreError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Option<ModerationResult>, StoreError> {
        let row: Option<ModerationRow> = sqlx::query_as(
            r#"
            SELECT id, message, danger_score, categories, created_at
            FROM moderations
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StoreError::QueryError(e.to_string()))?;

        Ok(row.map(|r| r.into()))
    }
}

#[derive(sqlx::FromRow)]
struct ModerationRow {
    id: Uuid,
    message: String,
    danger_score: f32,
    categories: Vec<String>,
    created_at: DateTime<Utc>,
}

impl From<ModerationRow> for ModerationResult {
    fn from(row: ModerationRow) -> Self {
        let categories = row
            .categories
            .iter()
            .filter_map(|s| match s.as_str() {
                "radical_positions" => Some(DangerCategory::RadicalPositions),
                "advertising_spam" => Some(DangerCategory::AdvertisingSpam),
                "doxxing" => Some(DangerCategory::Doxxing),
                _ => None,
            })
            .collect();

        Self {
            id: MessageId::from_uuid(row.id),
            message: row.message,
            danger_score: DangerScore::new(row.danger_score),
            categories,
            created_at: row.created_at,
        }
    }
}
