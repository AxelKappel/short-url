use actix_web::{web, HttpResponse, HttpRequest, Responder};
use askama::Template;
use serde::Deserialize;

use super::db;
use super::url_validator;
use super::error;

#[derive(Deserialize)]
pub struct FormData {
    short_url: String,
    long_url: String,
}

#[derive(Template)]
#[template(path = "new.html")]
struct NewTemplate<'a> {
    css: &'a str,
    favicon: &'a str,
    short_url: &'a str,
}

pub async fn new(req: HttpRequest, form: web::Form<FormData>) -> impl Responder {
    if form.short_url.is_empty() && form.short_url.is_empty() {
        return error::render(req, "Alias and url must atleast be one character long");
    }

    //validates that the url is an actual url
    if !url_validator::is_valid(&form.long_url) {
        return error::render(req, "Invalid url");
    }

    //try to add to database
    let success = match db::add(&form.short_url, &form.long_url) {
        Ok(success) => success,
        Err(_) => {
            return error::render(req, "Alias taken");
        },
    };

    if !success {
        return error::render(req, "Alias taken");
    }

    let css = req.url_for("static", &["style", "new_style.css"]).unwrap();
    let favicon = req.url_for("static", &["images", "favicon.ico"]).unwrap();
    let short_url = req.url_for("redirect", &[&form.short_url]).unwrap();

    let template = NewTemplate {
        css: css.as_str(),
        favicon: favicon.as_str(),
        short_url: short_url.as_str(),
    };

    HttpResponse::Ok().body(template.render().unwrap())
}