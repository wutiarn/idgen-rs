use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("bad request: {0}")]
    BadRequest(String)
}

impl HttpError {
    pub fn get_status(&self) -> StatusCode {
        return match self {
            HttpError::BadRequest(_) => {
                StatusCode::BAD_REQUEST
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
