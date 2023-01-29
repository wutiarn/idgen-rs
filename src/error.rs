use std::io::Cursor;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;

use log::info;
use rocket::{Request, Response};
use rocket::http::{Header, Status};
use rocket::response::Responder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("bad request: {0}")]
    BadRequest(String),
    #[error("internal server error: {0}")]
    InternalServerError(String),
}

impl HttpError {
    pub fn get_status(&self) -> StatusCode {
        return match self {
            HttpError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
            }
            _other => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        self.get_status()
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }
}
