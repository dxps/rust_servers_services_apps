use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080 ...");

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter3/**/*")).unwrap();

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
    // Some mock data, for convenience.
    let tutors: Vec<Tutor> = vec![
        Tutor {
            name: String::from("Tutor 1"),
        },
        Tutor {
            name: String::from("Tutor 2"),
        },
        Tutor {
            name: String::from("Tutor 3"),
        },
        Tutor {
            name: String::from("Tutor 4"),
        },
        Tutor {
            name: String::from("Tutor 5"),
        },
    ];
    let mut ctx = tera::Context::new();
    ctx.insert("tutors", &tutors);

    let rendered_html = tmpl.render("list.html", &ctx).map_err(|err| {
        println!("handle_get_tutors render error: {}", err);
        error::ErrorInternalServerError("Template error")
    })?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered_html))
}
