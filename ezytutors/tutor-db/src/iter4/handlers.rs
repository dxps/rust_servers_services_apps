use actix_web::{web, HttpResponse};

use crate::{db_access, errors::EzyTutorsError, models::Course, state::AppState};
use std::convert::TryFrom;

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
) -> Result<HttpResponse, EzyTutorsError> {
    let tutor_id = params.0;
    let tutor_id = i32::try_from(tutor_id).unwrap();
    db_access::get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let tutor_id = params.0;
    let tutor_id = i32::try_from(tutor_id).unwrap();
    let course_id = params.1;
    let course_id = i32::try_from(course_id).unwrap();
    let course = db_access::get_course_details_db(&app_state.db, tutor_id, course_id).await;

    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<Course>,
) -> Result<HttpResponse, EzyTutorsError> {
    db_access::post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{env, sync::Mutex};

    use actix_web::{http::StatusCode, web};
    use chrono::NaiveDate;
    use dotenv::dotenv;
    use sqlx::PgPool;

    use crate::{handlers::get_course_details, models::Course, state::AppState};

    use super::{get_courses_for_tutor, post_new_course};

    #[actix_rt::test]
    async fn get_courses_for_tutor_successful() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
        let db_pool = PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let tutor_id: web::Path<(usize,)> = web::Path::from((1,));

        let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_details_successful() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
        let db_pool = PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(usize, usize)> = web::Path::from((1, 1));

        let resp = get_course_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn post_course_successful() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
        let db_pool = PgPool::connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let new_course = Course {
            course_id: 3,
            tutor_id: 1,
            course_name: "New Course".to_string(),
            posted_time: Some(NaiveDate::from_ymd(2021, 08, 06).and_hms(22, 35, 00)),
        };
        let course_param = web::Json(new_course);

        let resp = post_new_course(app_state, course_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
