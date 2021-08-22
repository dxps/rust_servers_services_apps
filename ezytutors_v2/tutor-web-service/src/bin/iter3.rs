use std::{env, io, sync::Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use state::AppState;

#[path = "../iter3/state.rs"]
mod state;

#[path = "../iter3/models.rs"]
mod models;

#[path = "../iter3/db_access.rs"]
mod db_access;

#[path = "../iter3/handlers.rs"]
mod handlers;

#[path = "../iter3/routes.rs"]
mod routes;

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
