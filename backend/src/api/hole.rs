use actix_web::{
    delete, get, post, put,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::hole::HoleDTO, AppState};

#[get("/holes/{id}")]
pub async fn get_hole(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let hole = db.get_hole(&id);
    match hole {
        Some(hole) => HttpResponse::Ok().json(hole),
        None => HttpResponse::NotFound().body("Hole not found"),
    }
}

#[post("/holes")]
pub async fn create_hole(app_state: Data<AppState>, hole_dto: Json<HoleDTO>) -> HttpResponse {
    let db = &app_state.db;
    let hole = db.create_hole(hole_dto.into_inner());
    match hole {
        Ok(hole) => HttpResponse::Ok().json(hole),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/holes/{id}")]
pub async fn delete_hole(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let count = db.delete_hole(&id);
    match count {
        Ok(_) => HttpResponse::Ok().json("Deleted hole".to_string()),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/holes/{id}")]
pub async fn update_hole(
    app_state: Data<AppState>,
    id: web::Path<String>,
    hole_dto: Json<HoleDTO>,
) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let hole = db.update_hole(&id, hole_dto.into_inner());
    match hole {
        Ok(hole) => HttpResponse::Ok().json(hole),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
