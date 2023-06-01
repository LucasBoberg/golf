use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder, Result};
use models::config::Config;
use repository::database::Database;
use serde::Serialize;

mod api;
mod middlewares;
mod models;
mod repository;

pub struct AppState {
    db: Database,
    env: Config,
}

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Healthy".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let db = repository::database::Database::new();
    let config = Config::init();
    let app_data = web::Data::new(AppState { db, env: config });

    let addr = std::env::var("ADDRESS").expect("ADDRESS must be set");
    let port = std::env::var("PORT").expect("PORT must be set");
    let address = format!("{}:{}", addr, port);

    log::info!("starting HTTP server at http://{}", address);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::config)
            .wrap(middleware::Logger::default())
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(address)?
    .run()
    .await
}
