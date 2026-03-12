use crate::models::{DBError, Question, QuestionDetail};
use async_trait::async_trait;
use log::{error, info};
use sqlx::{PgPool, Row};
use std::str::FromStr;

#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: &String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

pub struct QuestionsDaoImpl {
    db: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        QuestionsDaoImpl { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let query = sqlx::query(
            "INSERT INTO questions ( title, description ) VALUES ( $1, $2 ) RETURNING *",
        )
        .bind(question.title)
        .bind(question.description)
        .fetch_one(&self.db)
        .await;

        match query {
            Ok(row) => {
                info!("Successfully inserted question: {:?}", row);
                Ok(QuestionDetail {
                    question_uuid: row.get::<sqlx::types::Uuid, _>("question_uuid").to_string(),
                    title: row.get("title"),
                    description: row.get("description"),
                    created_at: row
                        .get::<chrono::NaiveDateTime, _>("created_at")
                        .to_string(),
                })
            }
            Err(error) => {
                error!("Error inserting question: {:?}", error);
                Err(DBError::Other(Box::from(error)))
            }
        }
    }

    async fn delete_question(&self, question_uuid: &String) -> Result<(), DBError> {
        let uuid = sqlx::types::Uuid::from_str(question_uuid).map_err(|_error| {
            DBError::InvalidUUID(format!("Could not parse question UUID: {}", question_uuid))
        });

        if uuid.is_err() {
            return Err(uuid.unwrap_err());
        }

        let query_result = sqlx::query("DELETE FROM questions WHERE question_uuid = $1")
            .bind(uuid.as_ref().unwrap())
            .execute(&self.db)
            .await;

        if query_result.is_err() {
            return Err(DBError::Other(Box::from(query_result.unwrap_err())));
        }

        let query_result = sqlx::query("DELETE FROM answers WHERE question_uuid = $1")
            .bind(uuid.as_ref().unwrap())
            .execute(&self.db)
            .await;

        if query_result.is_err() {
            return Err(DBError::Other(Box::from(query_result.unwrap_err())));
        }

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let records = sqlx::query("SELECT * FROM questions")
            .fetch_all(&self.db)
            .await
            .map_err(|error| {
                error!("{}", error);
                DBError::Other(Box::from(error))
            });

        if let Err(query_error) = records {
            return Err(query_error);
        }

        let mut questions = vec![];

        if let Ok(results) = records {
            info!("********* Question Records *********");
            info!("{:#?}", results);

            for record in results {
                questions.push(QuestionDetail {
                    question_uuid: record
                        .get::<sqlx::types::Uuid, _>("question_uuid")
                        .to_string(),
                    title: record.get("title"),
                    description: record.get("description"),
                    created_at: record
                        .get::<chrono::NaiveDateTime, _>("created_at")
                        .to_string(),
                });
            }
        }

        Ok(questions)
    }
}
