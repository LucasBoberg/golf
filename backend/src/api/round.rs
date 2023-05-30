use actix_web::{
    get, post,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::round::RoundDTO, repository::database::Database};

#[get("/rounds")]
pub async fn get_rounds(db: Data<Database>) -> HttpResponse {
    let rounds = db.get_rounds();
    HttpResponse::Ok().json(rounds)
}

#[get("/round/{id}")]
pub async fn get_round(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let round = db.get_round(&id);
    match round {
        Some(round) => HttpResponse::Ok().json(round),
        None => HttpResponse::NotFound().body("Round not found"),
    }
}

#[post("/round")]
pub async fn create_round(db: Data<Database>, round_dto: Json<RoundDTO>) -> HttpResponse {
    let round = db.create_round(round_dto.into_inner());
    match round {
        Ok(round) => HttpResponse::Ok().json(round),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
