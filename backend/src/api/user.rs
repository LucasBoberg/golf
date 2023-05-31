use actix_web::{
    get, post,
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};

use crate::{
    middlewares::auth,
    models::user::{SignInDTO, UserDTO},
    AppState,
};

#[post("/auth/sign-in")]
pub async fn sign_in(app_state: Data<AppState>, sign_in_dto: Json<SignInDTO>) -> HttpResponse {
    let db = &app_state.db;
    let secret = &app_state.env.jwt_secret;
    let auth_response = db.sign_in(secret.to_string(), sign_in_dto.into_inner());
    match auth_response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/auth/sign-up")]
pub async fn sign_up(app_state: Data<AppState>, user_dto: Json<UserDTO>) -> HttpResponse {
    let db = &app_state.db;
    let user = db.register_user(user_dto.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/me")]
pub async fn get_me(
    req: HttpRequest,
    app_state: Data<AppState>,
    _: auth::JwtMiddleware,
) -> HttpResponse {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();
    let db = &app_state.db;
    let user = db.get_user(user_id);
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
