use std::{env, io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
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

    let db_url = env::var("DATABASE_URL").expect("DB_URL is not set in .env file");
    let db_pool = PgPool::connect(&db_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm ok".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::general_routes)
            .configure(routes::course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
