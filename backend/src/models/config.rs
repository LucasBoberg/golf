#[derive(Debug, Clone)]
pub struct Config {
    pub jwt_secret: String,
    pub jwt_expires_in: i64,
    pub refresh_secret: String,
    pub refresh_expires_in: i64,
}

impl Config {
    pub fn init() -> Config {
        let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expires_in = std::env::var("JWT_EXPIRED_IN").expect("JWT_EXPIRED_IN must be set");
        let refresh_secret = std::env::var("REFRESH_SECRET").expect("REFRESH_SECRET must be set");
        let refresh_expires_in =
            std::env::var("REFRESH_EXPIRED_IN").expect("REFRESH_EXPIRED_IN must be set");
        Config {
            jwt_secret,
            jwt_expires_in: jwt_expires_in.parse::<i64>().unwrap(),
            refresh_secret,
            refresh_expires_in: refresh_expires_in.parse::<i64>().unwrap(),
        }
    }
}
