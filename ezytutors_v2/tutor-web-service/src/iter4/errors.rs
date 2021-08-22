use std::fmt;

use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;
use sqlx::error::Error as SQLxError;

#[derive(Debug, Serialize)]
pub enum EzyTutorsError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorsError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorsError::DBError(msg) => {
                println!("DBError occured: {:?}", msg);
                "Database error".into()
            }
            EzyTutorsError::ActixError(msg) => {
                println!("ActixError occured: {:?}", msg);
                "Actix error".into()
            }
            EzyTutorsError::NotFound(msg) => {
                println!("NotError occured: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for EzyTutorsError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            EzyTutorsError::DBError(_) | EzyTutorsError::ActixError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorsError::NotFound(_) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzyTutorsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for EzyTutorsError {
    fn from(err: actix_web::error::Error) -> Self {
        EzyTutorsError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for EzyTutorsError {
    fn from(err: SQLxError) -> Self {
        EzyTutorsError::DBError(err.to_string())
    }
}
