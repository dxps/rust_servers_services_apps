use actix_web::{
    error,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer, Result,
};
use serde::{Deserialize, Serialize};
use tera::Tera;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on 127.0.0.1:8080 ...");

    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();

        App::new() // Instantiate App.
            .app_data(Data::new(tera)) // Store Tera into app data.
            .configure(app_config) // Configure routes.
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn app_config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("")
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/tutors").route(web::post().to(handle_post_tutor))),
    );
}

async fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let s = tmpl
        .render("form.html", &tera::Context::new())
        .map_err(|err| {
            println!("Template error: {}", err);
            error::ErrorInternalServerError("Template error")
        })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[derive(Serialize, Deserialize)]
pub struct Tutor {
    name: String,
}

async fn handle_post_tutor(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<Tutor>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("name", &params.name);
    ctx.insert("text", "Welcome!");

    let s = tmpl.render("user.html", &ctx).map_err(|err| {
        println!("handle_post_tutor> Template error: {}", err);
        error::ErrorInternalServerError("Template error")
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[cfg(test)]
mod tests {
    use actix_web::{
        dev::{Service, ServiceResponse},
        http::{header::CONTENT_TYPE, HeaderValue, StatusCode},
        test,
        web::Form,
    };

    use super::*;

    /// A Unit Test for `handle_post_tutor`.
    #[actix_rt::test]
    async fn handle_post_1_unit_test() {
        let params = Form(Tutor {
            name: "John Doe".to_string(),
        });
        let tera =
            tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        let webdata_tera = web::Data::new(tera);

        let resp = handle_post_tutor(webdata_tera, params).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }

    /// An Integration Test for `handle_post_tutor`.
    #[actix_rt::test]
    async fn handle_post_1_integration_test() {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter2/**/*")).unwrap();
        let app =
            test::init_service(App::new().app_data(Data::new(tera)).configure(app_config)).await;

        let req = test::TestRequest::post()
            .uri("/tutors")
            .set_form(&Tutor {
                name: "John Doe".to_string(),
            })
            .to_request();
        let resp: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(
            resp.headers().get(CONTENT_TYPE).unwrap(),
            HeaderValue::from_static("text/html")
        );
    }
}
