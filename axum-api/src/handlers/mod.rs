mod handlers_inner;

#[cfg(test)]
mod tests;

use crate::AppState;
use crate::models::*;
use axum::extract::State;
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};
use log::{error, info};

// ---- CRUD for Questions ----
pub async fn create_question(
    State(state): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    let question_detail =
        handlers_inner::create_question(question, state.questions_dao.as_ref()).await;

    match question_detail {
        Ok(question) => Json(question).into_response(),
        Err(error) => {
            error!("{:?}", error);

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to create question".into())
                .unwrap()
        }
    }
}

pub async fn read_questions(State(state): State<AppState>) -> impl IntoResponse {
    let questions = handlers_inner::read_questions(state.questions_dao.as_ref()).await;

    match questions {
        Ok(questions) => Json(questions).into_response(),
        Err(questions) => {
            error!("{:?}", questions);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to fetch questions".into())
                .unwrap()
        }
    }
}

pub async fn delete_question(
    State(state): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    let question_deletion =
        handlers_inner::delete_question(&question_uuid, state.questions_dao.as_ref()).await;

    match question_deletion {
        Ok(deletion) => {
            info!("Question deleted: {:#?}", &question_uuid.question_uuid);
            Json(StatusCode::OK).into_response()
        }
        Err(questions) => {
            error!("Failed to delete question error: {:?}", questions);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to delete question".into())
                .unwrap()
        }
    }
}

// ---- CRUD for Answers ----
pub async fn create_answer(
    State(state): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    let answer_detail = handlers_inner::create_answer(answer, state.answers_dao.as_ref()).await;

    match answer_detail {
        Ok(answer) => Json(answer).into_response(),
        Err(error) => {
            error!("{:?}", error);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to create answer".into())
                .unwrap()
        }
    }
}

pub async fn read_answers(
    State(state): State<AppState>,
    Json(question): Json<QuestionId>,
) -> impl IntoResponse {
    let answers = handlers_inner::read_answers(&question, state.answers_dao.as_ref()).await;

    match answers {
        Ok(answers) => Json(answers).into_response(),
        Err(answers) => {
            error!("Failed to fetch answers error: {:?}", answers);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to fetch answers".to_string())
                .unwrap()
                .into_response()
        }
    }
}

pub async fn delete_answer(
    State(state): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> impl IntoResponse {
    let answer_deletion =
        handlers_inner::delete_answer(&answer_uuid, state.answers_dao.as_ref()).await;

    match answer_deletion {
        Ok(deletion) => {
            info!("Answer deleted: {:#?}", &answer_uuid.answer_uuid);
            Response::builder()
                .status(StatusCode::OK)
                .body("Answer deleted successfully".to_string())
                .unwrap()
        }
        Err(answers) => {
            error!("Failed to delete answer error: {:?}", answers);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body("Failed to delete answer".to_string())
                .unwrap()
        }
    }
}
