use sqlx::PgPool;

use crate::models::Course;

pub async fn get_courses_for_tutor_db(pool: &PgPool, tutor_id: i32) -> Vec<Course> {
    // sql prep.
    let course_rows = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1", tutor_id
    ).fetch_all(pool)
    .await.unwrap();

    course_rows
        .iter()
        .map(|row| Course {
            course_id: row.course_id,
            tutor_id: row.tutor_id,
            course_name: row.course_name.clone(),
            posted_time: row.posted_time,
        })
        .collect()
}

pub async fn get_course_details_db(pool: &PgPool, tutor_id: i32, course_id: i32) -> Course {
    // sql prep.
    let row = sqlx::query!(
        "SELECT course_name, posted_time FROM ezy_course_c4 WHERE tutor_id = $1 AND course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Course {
        course_id,
        tutor_id,
        course_name: row.course_name,
        posted_time: row.posted_time,
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let row = sqlx::query!("INSERT INTO ezy_course_c4 (course_id, tutor_id, course_name) VALUES ($1, $2, $3) RETURNING posted_time",
        new_course.course_id, new_course.tutor_id, new_course.course_name).fetch_one(pool).await.unwrap();

    Course {
        course_id: new_course.course_id,
        tutor_id: new_course.tutor_id,
        course_name: new_course.course_name.clone(),
        posted_time: row.posted_time,
    }
}
