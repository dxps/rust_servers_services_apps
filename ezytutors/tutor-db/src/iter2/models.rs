use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(jc: web::Json<Course>) -> Self {
        Course {
            course_id: jc.course_id,
            tutor_id: jc.tutor_id,
            course_name: jc.course_name.clone(),
            posted_time: jc.posted_time,
        }
    }
}
