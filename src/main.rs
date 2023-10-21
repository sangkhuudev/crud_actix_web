mod routes;
mod models;

use crate::routes::{config, health_check};
use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use sqlx::{ Pool, Postgres};
use sqlx::pool::PoolOptions;


pub struct AppState {
    pub db: Pool<Postgres>,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let pool: Pool<Postgres> = match PoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("Connection to db is successful");
            pool
        },
        Err(err) => {
            println!("Error connecting to database {:?}", err);
            std::process::exit(1);
        }
    };

    HttpServer::new(move || {
        // Client side like React, Vue running on port 3000
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::CONTENT_ENCODING,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .service(health_check)
            .configure(config)
    })
    // Server side running on port 8000
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
