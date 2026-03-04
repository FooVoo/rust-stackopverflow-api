use crate::models::QuestionDetail;
use chrono::Utc;
use uuid::Uuid;

pub(crate) trait QuestionBuilder {
    fn new(title: String, description: String) -> Result<QuestionDetail, String>;
}

impl QuestionBuilder for QuestionDetail {
    fn new(title: String, description: String) -> Result<QuestionDetail, String> {
        if title.is_empty() {
            return Err(String::from("title is empty"));
        }
        if description.is_empty() {
            return Ok(QuestionDetail {
                question_uuid: Uuid::new_v4().to_string(),
                title,
                description: "No description provided".to_string(),
                created_at: Utc::now().to_rfc3339(),
            });
        }
        Ok(QuestionDetail {
            question_uuid: Uuid::new_v4().to_string(),
            title,
            description,
            created_at: Utc::now().to_rfc3339(),
        })
    }
}
