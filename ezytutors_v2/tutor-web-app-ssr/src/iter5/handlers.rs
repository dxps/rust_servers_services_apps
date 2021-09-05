use actix_web::{web, Error, HttpResponse, Result};

use crate::iter5::errors::EzyTutorError;

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    let s = tmpl.render("register.html", &ctx).map_err(|err| {
        println!("show_register_form> Render err: {}", err);
        EzyTutorError::TeraError("Template error".into())
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body(""))
}
