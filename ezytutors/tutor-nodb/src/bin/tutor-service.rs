use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use state::AppState;

use routes::*;

#[path = "../state.rs"]
mod state;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "iam_ok".to_string(),
        visit_count: Mutex::new(0),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
