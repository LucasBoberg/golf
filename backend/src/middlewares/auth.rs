use core::fmt;
use std::future::{ready, Ready};

use actix_web::{
    dev::Payload, error::ErrorUnauthorized, web, Error as ActixWebError, FromRequest, HttpMessage,
    HttpRequest,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{models::user::TokenClaims, AppState};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    status: String,
    message: String,
}

impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
pub struct JwtMiddleware {
    pub user_id: Uuid,
}

impl FromRequest for JwtMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<JwtMiddleware, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let data = req.app_data::<web::Data<AppState>>().unwrap();
        let auth_header = req.headers().get("Authorization");
        let token = match auth_header {
            Some(header) => header.to_str().unwrap().split_whitespace().last().unwrap(),
            None => return ready(Err(ErrorUnauthorized("Missing authorization header"))),
        };
        let claims = match decode::<TokenClaims>(
            &token,
            &DecodingKey::from_secret(data.env.jwt_secret.as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(ErrorResponse {
                    status: "error".to_string(),
                    message: "Invalid token".to_string(),
                })));
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        ready(Ok(JwtMiddleware { user_id }))
    }
}

pub fn generate_token(user_id: Uuid, secret: String, expires_in: i64) -> String {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(expires_in)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id.to_string(),
        exp,
        iat,
    };

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
