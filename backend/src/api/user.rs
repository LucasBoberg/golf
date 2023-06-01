use actix_web::{
    get, post,
    web::{Data, Json},
    HttpMessage, HttpRequest, HttpResponse,
};
use jsonwebtoken::{decode, DecodingKey, Validation};

use crate::{
    api::ErrorResponse,
    middlewares::auth::{self, generate_token},
    models::user::{AuthResponse, RefreshDTO, SignInDTO, SignUpDTO, TokenClaims, UserDTO},
    AppState,
};

#[post("/auth/sign-in")]
pub async fn sign_in(app_state: Data<AppState>, sign_in_dto: Json<SignInDTO>) -> HttpResponse {
    let db = &app_state.db;
    let auth_response = db.sign_in(
        app_state.env.jwt_secret.to_owned(),
        app_state.env.jwt_expires_in,
        app_state.env.refresh_secret.to_owned(),
        app_state.env.refresh_expires_in,
        sign_in_dto.into_inner(),
    );
    match auth_response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

#[post("/auth/refresh")]
pub async fn refresh(app_state: Data<AppState>, refresh_dto: Json<RefreshDTO>) -> HttpResponse {
    let user_id = match decode::<TokenClaims>(
        &refresh_dto.refresh_token,
        &DecodingKey::from_secret(app_state.env.refresh_secret.as_ref()),
        &Validation::default(),
    ) {
        Ok(c) => uuid::Uuid::parse_str(&c.claims.sub).unwrap(),
        Err(_) => {
            return HttpResponse::Unauthorized().json(ErrorResponse {
                status: "error".to_string(),
                message: "Invalid refresh token".to_string(),
            });
        }
    };

    let token = generate_token(
        user_id,
        app_state.env.jwt_secret.to_owned(),
        app_state.env.jwt_expires_in,
    );

    let refresh_token = generate_token(
        user_id,
        app_state.env.refresh_secret.to_owned(),
        app_state.env.refresh_expires_in,
    );

    HttpResponse::Ok().json(AuthResponse {
        token,
        refresh_token,
    })
}

#[post("/auth/sign-up")]
pub async fn sign_up(app_state: Data<AppState>, sign_up_dto: Json<SignUpDTO>) -> HttpResponse {
    let db = &app_state.db;
    let user = db.register_user(sign_up_dto.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(UserDTO::from(user)),
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
        Ok(user) => HttpResponse::Ok().json(UserDTO::from(user)),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
