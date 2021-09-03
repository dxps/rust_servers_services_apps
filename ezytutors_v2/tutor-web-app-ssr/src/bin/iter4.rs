use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use awc::Client;
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize, Debug)]
pub struct Tutor {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080 ...");

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter4/**/*")).unwrap();

        App::new() // Instantiate App.
            .app_data(web::Data::new(tera)) // Store Tera into app data.
            .configure(app_config) // Configure routes.
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(actix_files::Files::new("/static", "./static").show_files_listing())
        .service(
            web::scope("")
                .service(web::resource("/tutors").route(web::get().to(handle_get_tutors))),
        );
}

async fn handle_get_tutors(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let client = Client::default();
    let resp = client
        .get("http://localhost:3000/tutors")
        .send()
        .await
        .unwrap()
        .body()
        .await
        .unwrap();

    let str_list = std::str::from_utf8(resp.as_ref()).unwrap();
    let tutor_list: Vec<Tutor> = serde_json::from_str(str_list).unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutor_list);

    let rendered_html = tmpl.render("list.html", &ctx).map_err(|err| {
        println!("handle_get_tutors render error: {}", err);
        error::ErrorInternalServerError("Template error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}
