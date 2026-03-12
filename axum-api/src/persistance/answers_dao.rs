use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};
use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::{PgPool, Row};

#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: &String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: &String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        AnswersDaoImpl { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::parse_str(answer.question_uuid.as_str()).map_err(|_error| {
            DBError::InvalidUUID(format!(
                "Could not parse question UUID: {}",
                answer.question_uuid
            ))
        });

        if let Err(uuid_error) = uuid {
            return Err(uuid_error);
        }

        let query_result = sqlx::query(
            "
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *",
        )
        .bind(uuid.ok())
        .bind(answer.content)
        .fetch_one(&self.db)
        .await
        .map_err(|error| {
            if let sqlx::Error::Database(db_error) = &error {
                if let Some(code) = db_error.code() {
                    if code == postgres_error_codes::FOREIGN_KEY_VIOLATION {
                        return DBError::InvalidUUID(format!(
                            "Invalid question UUID: {}",
                            answer.question_uuid
                        ));
                    }
                }
            }
            DBError::Other(Box::new(error))
        });

        if query_result.is_err() {
            return Err(query_result.unwrap_err());
        }

        let record = query_result?;

        Ok(AnswerDetail {
            answer_uuid: record.get::<Uuid, _>("answer_uuid").to_string(),
            question_uuid: record.get::<Uuid, _>("question_uuid").to_string(),
            content: record.get("content"),
            created_at: record
                .get::<chrono::NaiveDateTime, _>("created_at")
                .to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: &String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::parse_str(answer_uuid).map_err(|_error| {
            DBError::InvalidUUID(format!("Could not parse answer UUID: {}", answer_uuid))
        });

        if uuid.is_err() {
            return Err(uuid.unwrap_err());
        }

        let query = sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(uuid?)
            .execute(&self.db)
            .await;

        if query.is_err() {
            return Err(DBError::Other(Box::new(query.unwrap_err())));
        }

        Ok(())
    }

    async fn get_answers(&self, question_uuid: &String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::parse_str(question_uuid.as_str()).map_err(|_error| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        });

        if uuid.is_err() {
            return Err(uuid.unwrap_err());
        }

        let query_result = sqlx::query("SELECT * FROM answers WHERE question_uuid = $1")
            .bind(uuid?)
            .fetch_all(&self.db)
            .await;

        let mut records: Vec<AnswerDetail> = vec![];

        if let Ok(answers) = query_result {
            for record in answers {
                records.push(AnswerDetail {
                    answer_uuid: record.get::<Uuid, _>("answer_uuid").to_string(),
                    question_uuid: record.get::<Uuid, _>("question_uuid").to_string(),
                    content: record.get("content"),
                    created_at: record
                        .get::<chrono::NaiveDateTime, _>("created_at")
                        .to_string(),
                });
            }
        }

        Ok(records)
    }
}
