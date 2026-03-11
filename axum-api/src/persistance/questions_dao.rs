use async_trait::async_trait;
use axum::Json;
use log::{error, info};
use serde::de::Unexpected::Str;
use sqlx::{PgPool, Row};

use crate::models::{DBError, Question, QuestionDetail};

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
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = todo!();

        // TODO: Make a database query to delete a question given the question uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM questions WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.

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

        let questions = vec![];

        if let Ok(results) = records {
            info!("********* Question Records *********");
            info!("{:?}", results);
        }

        Ok(questions)
    }
}
