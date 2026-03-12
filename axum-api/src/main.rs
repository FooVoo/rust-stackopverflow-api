use axum::{
    Router,
    routing::{delete, get, post},
};
use log::info;

use dotenv::dotenv;
use sqlx::Executor;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

mod handlers;
mod models;
mod persistance;

use crate::persistance::answers_dao::{AnswersDao, AnswersDaoImpl};
use crate::persistance::questions_dao::{QuestionsDao, QuestionsDaoImpl};
use handlers::*;

#[derive(Clone)]
struct AppState {
    questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").expect("DATABASE_URL not set"))
        .await
        .expect("DATABASE_URL not set");

    let recs = pool
        .fetch_all(sqlx::query("SELECT * from questions"))
        .await
        .expect("Query failed");
    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    let app_state = AppState {
        questions_dao: Arc::new(questions_dao),
        answers_dao: Arc::new(answers_dao),
    };

    info!("********* Question Records *********");
    info!("{:#?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
