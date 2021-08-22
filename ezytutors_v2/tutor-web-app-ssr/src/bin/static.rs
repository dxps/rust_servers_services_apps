use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080 ...");

    HttpServer::new(|| {
        App::new().service(actix_files::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
