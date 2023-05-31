use actix_web::web;

use self::{
    course::{
        create_course, delete_course, get_course, get_course_holes, get_courses, update_course,
    },
    hole::{create_hole, delete_hole, get_hole, update_hole},
    round::{create_round, delete_round, get_round, get_rounds},
};

pub mod course;
pub mod hole;
pub mod round;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
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
