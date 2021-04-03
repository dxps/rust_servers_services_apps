use actix_web::{web, HttpResponse};
use chrono::Utc;

use super::models::Course;

use super::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let hc_resp = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let resp = format!("{} {} times", hc_resp, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(resp)
}

pub async fn new_course(course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {
    println!("New course is posted");
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|c| c.tutor_id == course.tutor_id)
        .collect::<Vec<Course>>()
        .len();
    let new_course = Course {
        id: Some(course_count_for_user + 1),
        name: course.name.clone(),
        tutor_id: course.tutor_id,
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("Added course")
}

// ---------= TESTS =---------

#[cfg(test)]
mod tests {

    use std::sync::Mutex;

    use actix_web::http::StatusCode;

    use super::*;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            id: None,
            name: "Test Course".into(),
            tutor_id: 1,
            posted_time: None,
        });
        let app_state = web::Data::new(AppState {
            health_check_response: "ok".into(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
