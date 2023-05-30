use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::hole::HoleDTO, repository::database::Database};

#[get("/holes/{id}")]
pub async fn get_hole(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let hole = db.get_hole(&id);
    match hole {
        Some(hole) => HttpResponse::Ok().json(hole),
        None => HttpResponse::NotFound().body("Hole not found"),
    }
}

#[post("/holes")]
pub async fn create_hole(db: Data<Database>, hole_dto: Json<HoleDTO>) -> HttpResponse {
    let hole = db.create_hole(hole_dto.into_inner());
    match hole {
        Ok(hole) => HttpResponse::Ok().json(hole),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
