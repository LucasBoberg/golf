use std::fmt::{Display, Formatter, Result as FmtResult};

use actix_web::{
    http::{header, StatusCode},
    web, Error, HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string_pretty};

use self::{
    course::{
        create_course, delete_course, get_course, get_course_holes, get_courses, update_course,
    },
    hole::{create_hole, delete_hole, get_hole, update_hole},
    round::{create_round, delete_round, get_round, get_rounds},
    user::{get_me, refresh, sign_in, sign_up},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u16,
}

impl Display for ErrorResponse {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl From<Error> for ErrorResponse {
    fn from(err: Error) -> Self {
        ErrorResponse {
            message: err.to_string(),
            status: 500,
        }
    }
}

impl From<diesel::result::Error> for ErrorResponse {
    fn from(err: diesel::result::Error) -> ErrorResponse {
        ErrorResponse {
            message: err.to_string(),
            status: 500,
        }
    }
}

impl ResponseError for ErrorResponse {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.message });
        HttpResponse::build(StatusCode::from_u16(self.status).unwrap())
            .append_header(header::ContentType::json())
            .json(err_json)
    }

    fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }
}

pub mod course;
pub mod hole;
pub mod round;
pub mod user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(sign_in)
            .service(sign_up)
            .service(refresh)
            .service(get_me)
            .service(get_rounds)
            .service(get_round)
            .service(create_round)
            .service(delete_round)
            .service(get_courses)
            .service(get_course)
            .service(get_course_holes)
            .service(create_course)
            .service(delete_course)
            .service(update_course)
            .service(get_hole)
            .service(create_hole)
            .service(delete_hole)
            .service(update_hole),
    );
}
