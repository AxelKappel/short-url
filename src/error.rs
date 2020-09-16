use actix_web::{HttpResponse, HttpRequest};
use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    css: &'a str,
    favicon: &'a str,
    message: &'a str,
}

pub fn render(req: HttpRequest, message: &str) -> HttpResponse {
    let css = req.url_for("static", &["style", "error_style.css"]).unwrap();
    let favicon = req.url_for("static", &["images", "favicon.ico"]).unwrap();

    let template = ErrorTemplate {
        css: css.as_str(),
        favicon: favicon.as_str(),
        message,
    };
    HttpResponse::Ok().body(template.render().unwrap())
}