use actix_web::{web, HttpResponse};

use crate::{
    dbaccess,
    errors::EzyTutorsError,
    models::course::{CreateCourse, UpdateCourse},
    state::AppState,
};

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorsError> {
    let tutor_id = params.into_inner();
    dbaccess::get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorsError> {
    let (tutor_id, course_id) = params.into_inner();
    dbaccess::get_course_details_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn post_new_course(
    app_state: web::Data<AppState>,
    new_course: web::Json<CreateCourse>,
) -> Result<HttpResponse, EzyTutorsError> {
    dbaccess::post_new_course_db(&app_state.db, new_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
    update_course: web::Json<UpdateCourse>,
) -> Result<HttpResponse, EzyTutorsError> {
    let (tutor_id, course_id) = params.into_inner();
    dbaccess::update_course_details_db(&app_state.db, tutor_id, course_id, update_course.into())
        .await
        .map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(
    app_state: web::Data<AppState>,
    params: web::Path<(i32, i32)>,
) -> Result<HttpResponse, EzyTutorsError> {
    let (tutor_id, course_id) = params.into_inner();
    dbaccess::delete_course_db(&app_state.db, tutor_id, course_id)
        .await
        .map(|res| HttpResponse::Ok().json(res))
}

// --------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::{env, sync::Mutex};

    use actix_web::{http::StatusCode, web};
    use dotenv::dotenv;
    use sqlx::PgPool;

    use crate::{handlers::get_course_details, models::CreateCourse, state::AppState};

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
        let tutor_id: web::Path<i32> = web::Path::from(1);

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
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));

        let resp = get_course_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // Run with cargo test -- --nocapture
    // Get course details with invalid course id.
    #[actix_rt::test]
    async fn get_course_detail_failure_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let parameters: web::Path<(i32, i32)> = web::Path::from((1, 21));
        let resp = get_course_details(app_state, parameters).await;
        match resp {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
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

        let new_course = CreateCourse {
            tutor_id: 1,
            course_name: "Third course".into(),
            course_description: Some("This is a test course".into()),
            course_format: None,
            course_level: Some("Beginner".into()),
            course_price: None,
            course_duration: None,
            course_language: Some("English".into()),
            course_structure: None,
        };
        let course_param = web::Json(new_course);

        let resp = post_new_course(app_state, course_param).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
