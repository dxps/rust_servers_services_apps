use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(nt: web::Json<NewTutor>) -> Self {
        NewTutor {
            tutor_name: nt.tutor_name.clone(),
            tutor_pic_url: nt.tutor_pic_url.clone(),
            tutor_profile: nt.tutor_profile.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(ut: web::Json<UpdateTutor>) -> Self {
        UpdateTutor {
            tutor_name: ut.tutor_name.clone(),
            tutor_pic_url: ut.tutor_pic_url.clone(),
            tutor_profile: ut.tutor_profile.clone(),
        }
    }
}
