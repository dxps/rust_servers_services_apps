use sqlx::PgPool;

use crate::{errors::EzyTutorsError, models::Course};

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzyTutorsError> {
    // sql prep.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1", tutor_id
    ).fetch_all(pool)
    .await?;

    let courses: Vec<Course> = course_rows
        .iter()
        .map(|row| Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name.clone(),
            posted_time: row.posted_time,
        })
        .collect();
    match courses.len() {
        0 => Err(EzyTutorsError::NotFound(format!(
            "No courses found for tutor id {}",
            tutor_id
        ))),
        _ => Ok(courses),
    }
}
pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzyTutorsError> {
    // sql prep.
    let row = sqlx::query!(
        "SELECT course_name, posted_time FROM ezy_course_c5 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;

    if let Ok(course_row) = row {
        Ok(Course {
            course_id,
            tutor_id,
            course_name: course_row.course_name,
            posted_time: course_row.posted_time,
        })
    } else {
        Err(EzyTutorsError::NotFound(format!(
            "No course found for course_id={} and tutor_id={}",
            course_id, tutor_id
        )))
    }
}

pub async fn post_new_course_db(
    pool: &PgPool,
    new_course: Course,
) -> Result<Course, EzyTutorsError> {
    let row = sqlx::query!("INSERT INTO ezy_course_c5 (course_id, tutor_id, course_name) VALUES ($1, $2, $3) RETURNING posted_time",
        new_course.course_id, new_course.tutor_id, new_course.course_name).fetch_one(pool).await?;

    Ok(Course {
        course_id: new_course.course_id,
        tutor_id: new_course.tutor_id,
        course_name: new_course.course_name.clone(),
        posted_time: row.posted_time,
    })
}
