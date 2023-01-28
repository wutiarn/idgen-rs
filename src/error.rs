use std::io::Cursor;

use log::info;
use rocket::{Request, Response};
use rocket::http::{Header, Status};
use rocket::http::hyper::server::conn::Http;
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
    pub fn get_status(&self) -> Status {
        return match self {
            HttpError::BadRequest(_) => {
                Status::BadRequest
            }
            _other => {
                Status::InternalServerError
            }
        };
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for HttpError {
    fn respond_to(self, _: &'r Request) -> rocket::response::Result<'o> {
        let response_body = self.to_string();
        info!("Responding with error: {}", response_body);
        Response::build()
            .status(self.get_status())
            .header(Header::new("Content-Type", "text/plain"))
            .sized_body(response_body.len(), Cursor::new(response_body))
            .ok()
    }
}
