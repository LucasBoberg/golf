use actix_web::{
    delete, get, post,
    web::{self, Data, Json},
    HttpResponse,
};

use crate::{models::round::RoundDTO, AppState};

#[get("/rounds")]
pub async fn get_rounds(app_state: Data<AppState>) -> HttpResponse {
    let db = &app_state.db;
    let rounds = db.get_rounds();
    HttpResponse::Ok().json(rounds)
}

#[get("/round/{id}")]
pub async fn get_round(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let round = db.get_round(&id);
    match round {
        Some(round) => HttpResponse::Ok().json(round),
        None => HttpResponse::NotFound().body("Round not found"),
    }
}

#[post("/round")]
pub async fn create_round(app_state: Data<AppState>, round_dto: Json<RoundDTO>) -> HttpResponse {
    let db = &app_state.db;
    let round = db.create_round(round_dto.into_inner());
    match round {
        Ok(round) => HttpResponse::Ok().json(round),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/round/{id}")]
pub async fn delete_round(app_state: Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let db = &app_state.db;
    let id = uuid::Uuid::parse_str(&id).unwrap();
    let count = db.delete_round(&id);
    match count {
        Ok(count) => HttpResponse::Ok().body(format!("Deleted {} rounds", count)),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
