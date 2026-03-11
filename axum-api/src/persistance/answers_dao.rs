use crate::models::{Answer, AnswerDetail, DBError, postgres_error_codes};
use async_trait::async_trait;
use log::info;
use sqlx::postgres::PgDatabaseError;
use sqlx::types::Uuid;
use sqlx::{Executor, PgPool, query};

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
        // Use the `sqlx::types::Uuid::parse_str` method to parse the `question_uuid` field
        // in `Answer` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = Uuid::parse_str(answer.question_uuid.as_str()).map_err(|_error| {
            DBError::InvalidUUID(format!(
                "Could not parse question UUID: {}",
                answer.question_uuid
            ))
        });

        if let Err(uuid_error) = uuid {
            return Err(uuid_error);
        }

        // Make a database query to insert a new answer.
        // Here is the SQL query:
        // ```
        // INSERT INTO answers ( question_uuid, content )
        // VALUES ( $1, $2 )
        // RETURNING *
        // ```
        // If executing the query results in an error, check to see if
        // the error code matches `postgres_error_codes::FOREIGN_KEY_VIOLATION`.
        // If so early return the `DBError::InvalidUUID` error. Otherwise early return
        // the `DBError::Other` error.
        // let record = todo!();

        let query_result = sqlx::query(
            "
            INSERT INTO answers ( question_uuid, content )
            VALUES ( $1, $2 )
            RETURNING *",
        )
        .bind(uuid.ok())
        .bind(answer.content)
        .execute(&self.db)
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

        if let Err(query_error) = query_result {
            return Err(query_error);
        }

        if let Ok(record) = query_result {}

        // Populate the AnswerDetail fields using `record`.
        Ok(AnswerDetail {
            answer_uuid: todo!(),
            question_uuid: todo!(),
            content: todo!(),
            created_at: todo!(),
        })
    }

    async fn delete_answer(&self, answer_uuid: &String) -> Result<(), DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `answer_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = todo!();

        // TODO: Make a database query to delete an answer given the answer uuid.
        // Here is the SQL query:
        // ```
        // DELETE FROM answers WHERE answer_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.

        Ok(())
    }

    async fn get_answers(&self, question_uuid: &String) -> Result<Vec<AnswerDetail>, DBError> {
        // Use the `sqlx::types::Uuid::parse_str` method to parse `question_uuid` into a `Uuid` type.
        // parse_str docs: https://docs.rs/sqlx/latest/sqlx/types/struct.Uuid.html#method.parse_str
        //
        // If `parse_str` returns an error, map the error to a `DBError::InvalidUUID` error
        // and early return from this function.
        let uuid = todo!();

        // Make a database query to get all answers associated with a question uuid.
        // Here is the SQL query:
        // ```
        // SELECT * FROM answers WHERE question_uuid = $1
        // ```
        // If executing the query results in an error, map that error
        // to a `DBError::Other` error and early return from this function.
        let records = todo!();

        // Iterate over `records` and map each record to a `AnswerDetail` type
        let answers = todo!();

        Ok(answers)
    }
}
