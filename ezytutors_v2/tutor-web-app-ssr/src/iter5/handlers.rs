use actix_web::{
    web::{self, Bytes},
    Error, HttpResponse, Result,
};
use argon2::Config;
use serde_json::json;

use crate::iter5::{
    dbaccess::post_new_user,
    errors::EzyTutorError,
    models::{TutorResponse, User},
};

use super::{dbaccess::get_user_record, models::TutorRegisterForm, state::AppState};

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_username", "");
    ctx.insert("current_password", "");
    ctx.insert("current_password_confirmation", "");
    ctx.insert("current_name", "");
    ctx.insert("current_imageurl", "");
    ctx.insert("current_profile", "");

    let s = tmpl.render("register.html", &ctx).map_err(|err| {
        println!("show_register_form> Render err: {}", err);
        EzyTutorError::TeraError("Template error".into())
    })?;

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    app_state: web::Data<AppState>,
    params: web::Form<TutorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    let username = params.username.clone();
    let user = get_user_record(&app_state.db, username.to_string()).await;
    let user_not_found: bool = user.is_err();
    // If user is not found in database, proceed to verification of passwords
    if user_not_found {
        if params.password != params.password_confirmation {
            ctx.insert("error", "Passwords do not match");
            ctx.insert("current_username", &params.username);
            ctx.insert("current_password", "");
            ctx.insert("current_password_confirmation", "");
            ctx.insert("current_name", &params.name);
            ctx.insert("current_imageurl", &params.imageurl);
            ctx.insert("current_profile", &params.profile);
            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
        } else {
            let new_tutor = json!({
                "tutor_name": &params.name,
                "tutor_pic_url": &params.imageurl,
                "tutor_profile": &params.profile
            });
            let awc_client = awc::Client::default();
            let mut res = awc_client
                .post("http://localhost:3000/tutors")
                .send_json(&new_tutor)
                .await
                .unwrap();
            let res_status = &res.status();
            let res_body_bytes = &res.body().await.unwrap_or_else(|err| {
                println!("Error calling WS: '{}' (status={})", err, res_status);
                Bytes::new()
            });
            let ws_res = std::str::from_utf8(&res_body_bytes)?;
            println!(
                "Got WS response body: '{}' (length={}, status={})",
                ws_res,
                ws_res.len(),
                res_status
            );
            let tutor_response: TutorResponse =
                serde_json::from_str(&ws_res).unwrap_or_else(|err| {
                    println!(
                        "Error parsing ws response: '{}' having response: '{}' ",
                        err, &ws_res
                    );
                    TutorResponse {
                        tutor_id: 0,
                        tutor_name: "ERR".into(),
                        tutor_pic_url: "".into(),
                        tutor_profile: "".into(),
                    }
                });
            s = format!("Congratulations. You have been successfully registered with EzyTutor and your tutor id is: {}. To start using EzyTutor, please login with your credentials.",tutor_response.tutor_id);
            // Hash the password
            let salt = b"somerandomsalt";
            let config = Config::default();
            let hash =
                argon2::hash_encoded(params.password.clone().as_bytes(), salt, &config).unwrap();
            let user = User {
                username,
                password: hash,
                tutor_id: tutor_response.tutor_id,
            };
            let _tutor_created = post_new_user(&app_state.db, user).await?;
        }
    } else {
        ctx.insert("error", "User Id already exists");
        ctx.insert("current_username", &params.username);
        ctx.insert("current_password", "");
        ctx.insert("current_password_confirmation", "");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_imageurl", &params.imageurl);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|_| EzyTutorError::TeraError("Template error".to_string()))?;
    };

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
