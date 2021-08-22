use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(jcc: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: jcc.tutor_id,
            course_name: jcc.course_name.clone(),
            course_description: jcc.course_description.clone(),
            course_format: jcc.course_format.clone(),
            course_structure: jcc.course_structure.clone(),
            course_level: jcc.course_level.clone(),
            course_duration: jcc.course_duration.clone(),
            course_language: jcc.course_language.clone(),
            course_price: jcc.course_price,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateCourse {
    pub course_name: Option<String>,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(juc: web::Json<UpdateCourse>) -> Self {
        UpdateCourse {
            course_name: juc.course_name.clone(),
            course_description: juc.course_description.clone(),
            course_format: juc.course_format.clone(),
            course_structure: juc.course_structure.clone(),
            course_duration: juc.course_duration.clone(),
            course_price: juc.course_price,
            course_language: juc.course_language.clone(),
            course_level: juc.course_level.clone(),
        }
    }
}
