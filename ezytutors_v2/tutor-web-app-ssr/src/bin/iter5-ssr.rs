use std::env;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::PgPool;
use tera::Tera;

use crate::iter5::{routes::app_config, state::AppState};

// Module definition to import this iteration specific code.
#[path = "../iter5/mod.rs"]
mod iter5;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //
    dotenv().ok();

    // HTTP Server elements.
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Listening on {}", &host_port);

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&db_url).await.unwrap();

    let shared_data = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter5/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .app_data(shared_data.clone())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
