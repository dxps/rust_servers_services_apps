use actix_web::{web, HttpResponse};

use crate::{models::Course, state::AppState};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let hc_resp = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let resp = format!("{} {} times", hc_resp, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(resp)
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<(usize,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}
