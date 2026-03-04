use crate::models::AnswerDetail;
use chrono::Utc;
use uuid::Uuid;

pub(crate) trait AnswerBuilder {
    fn new(question_uuid: String, content: String) -> Result<AnswerDetail, String>;
}

impl AnswerBuilder for AnswerDetail {
    fn new(question_uuid: String, content: String) -> Result<AnswerDetail, String> {
        if !question_uuid.is_empty() {
            return Err("UUID is empty".to_string());
        }

        if !content.is_empty() {
            return Err("Content is empty".to_string());
        }

        Ok(AnswerDetail {
            answer_uuid: Uuid::new_v4().to_string(),
            question_uuid,
            content,
            created_at: Utc::now().to_rfc3339(),
        })
    }
}
