use std::{env, io, sync::Mutex};

#[path = "../iter2/handlers.rs"]
mod handlers;
#[path = "../iter2/models.rs"]
mod models;
#[path = "../iter2/routes.rs"]
mod routes;
#[path = "../iter2/state.rs"]
mod state;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use state::AppState;

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
