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
    _app_state: web::Data<AppState>,
    _params: web::Path<(usize,)>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

pub async fn get_course_details(
    _app_state: web::Data<AppState>,
    _params: web::Path<(usize, usize)>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
}

pub async fn post_new_course(
    _app_state: web::Data<AppState>,
    _new_course: web::Json<Course>,
) -> HttpResponse {
    HttpResponse::Ok().json("ok")
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

        let resp = get_courses_for_tutor(app_state, tutor_id).await;
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
            course_id: 1,
            tutor_id: 1,
            course_name: "New Course".to_string(),
            posted_time: Some(NaiveDate::from_ymd(2021, 08, 06).and_hms(22, 35, 00)),
        };
        let course_param = web::Json(new_course);

        let resp = post_new_course(app_state, course_param).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
