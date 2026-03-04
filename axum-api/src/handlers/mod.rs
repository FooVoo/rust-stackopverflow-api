use crate::models::answer::AnswerBuilder;
use crate::models::question::QuestionBuilder;
use crate::models::*;
use axum::{Json, http::StatusCode, response::IntoResponse, response::Response};

// ---- CRUD for Questions ----
pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    if let Ok(question_detail) = QuestionDetail::new(question.title, question.description) {
        return Response::builder()
            .status(StatusCode::CREATED)
            .body(Json(question_detail).into_response().into_body())
            .unwrap();
    }

    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body("Title cannot be empty".into())
        .unwrap()
}

pub async fn read_questions() -> impl IntoResponse {
    todo!()
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    // ToDo: Add check for valid UUID
    Response::builder().status(StatusCode::OK);
}

// ---- CRUD for Answers ----
pub async fn create_answer(Json(answer): Json<Answer>) -> impl IntoResponse {
    let answer_detail = AnswerDetail::new(answer.question_uuid, answer.content);
    if let Ok(answer_detail) = answer_detail {
        return Response::builder()
            .status(StatusCode::CREATED)
            .body(Json(answer_detail).into_response().into_body())
            .unwrap();
    }
    Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(answer_detail.unwrap_err().into_response().into_body())
        .unwrap()
}

pub async fn read_answers(Json(question_uuid): Json<QuestionId>) -> impl IntoResponse {
    todo!()
}

pub async fn delete_answer(Json(answer_uuid): Json<AnswerId>) {
    // ToDo: Add check for valid UUID
    Response::builder().status(StatusCode::OK);
}
