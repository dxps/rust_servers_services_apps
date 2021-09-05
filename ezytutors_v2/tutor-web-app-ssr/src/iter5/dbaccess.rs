use sqlx::PgPool;

use super::{errors::EzyTutorError, models::User};

pub async fn get_user_record(pool: &PgPool, username: String) -> Result<User, EzyTutorError> {
    // Prepare SQL statement.
    let user_row = sqlx::query_as!(
        User,
        "SELECT * FROM ezyweb_users WHERE username = $1",
        username
    )
    .fetch_optional(pool)
    .await?;

    if let Some(user) = user_row {
        Ok(user)
    } else {
        Err(EzyTutorError::NotFound("Username not found".into()))
    }
}

pub async fn post_new_user(pool: &PgPool, new_user: User) -> Result<User, EzyTutorError> {
    let user_row= sqlx::query_as!(User,"insert into ezyweb_users (username, password, tutor_id) values ($1,$2,$3) returning username, password,  tutor_id",
    new_user.username, new_user.password, new_user.tutor_id)
    .fetch_one(pool)
    .await?;

    Ok(user_row)
}
