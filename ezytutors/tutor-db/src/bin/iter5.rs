use std::{env, io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::EzyTutorsError;
use sqlx::PgPool;
use state::AppState;

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;

#[path = "../iter5/errors.rs"]
mod errors;

#[path = "../iter5/handlers/mod.rs"]
mod handlers;

#[path = "../iter5/models/mod.rs"]
mod models;

#[path = "../iter5/routes.rs"]
mod routes;

#[path = "../iter5/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    //Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorsError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(routes::general_routes)
            .configure(routes::course_routes)
            .configure(routes::tutor_routes)
    };

    //Start HTTP server
    let host_port =
        env::var("HOST_PORT").expect("HOST_PORT (as HOST:PORT) address is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
