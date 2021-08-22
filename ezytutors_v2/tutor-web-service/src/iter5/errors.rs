use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum EzyTutorsError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl std::error::Error for EzyTutorsError {}

impl EzyTutorsError {
    fn error_response(&self) -> String {
        match self {
            EzyTutorsError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            EzyTutorsError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            EzyTutorsError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
            EzyTutorsError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            EzyTutorsError::DBError(_msg) | EzyTutorsError::ActixError(_msg) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            EzyTutorsError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
            EzyTutorsError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }
}

impl error::ResponseError for EzyTutorsError {
    fn status_code(&self) -> StatusCode {
        self.status_code()
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzyTutorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
