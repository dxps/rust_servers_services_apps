use std::usize;

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<usize>,
    pub name: String,
    pub tutor_id: usize,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(cj: web::Json<Course>) -> Self {
        Course {
            id: cj.id,
            name: cj.name.clone(),
            tutor_id: cj.tutor_id,
            posted_time: cj.posted_time,
        }
    }
}
