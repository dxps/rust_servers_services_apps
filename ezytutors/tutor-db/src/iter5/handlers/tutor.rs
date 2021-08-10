use actix_web::{web, HttpResponse};

use crate::{dbaccess::tutor::get_all_tutors_db, errors::EzyTutorsError, state::AppState};

pub async fn get_all_tutors(
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorsError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}
