use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::course::CourseDTO, AppState};

#[get("/courses")]
pub async fn get_courses(app_state: Data<AppState>) -> HttpResponse {
    let db = &app_state.db;
    let courses = db.get_courses();
    HttpResponse::Ok().json(courses)
}

#[get("/courses/{id}")]
pub async fn get_course(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let course = db.get_course(&id);
    match course {
        Some(course) => HttpResponse::Ok().json(course),
        None => HttpResponse::NotFound().body("Course not found"),
    }
}

#[get("/courses/{id}/holes")]
pub async fn get_course_holes(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let holes = db.get_holes_by_course_id(&id);
    HttpResponse::Ok().json(holes)
}

#[post("/courses")]
pub async fn create_course(app_state: Data<AppState>, course_dto: Json<CourseDTO>) -> HttpResponse {
    let db = &app_state.db;
    let course = db.create_course(course_dto.into_inner());
    match course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/courses/{id}")]
pub async fn delete_course(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let count = db.delete_course(&id);
    match count {
        Ok(_) => HttpResponse::Ok().json("Deleted course".to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/courses/{id}")]
pub async fn update_course(
    app_state: Data<AppState>,
    id: web::Path<String>,
    course_dto: Json<CourseDTO>,
) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let course = db.update_course(&id, course_dto.into_inner());
    match course {
        Ok(course) => HttpResponse::Ok().json(course),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
