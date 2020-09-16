use actix_web::{HttpResponse, HttpRequest, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    css: &'a str,
    favicon: &'a str,
    new: &'a str,
    url: &'a str,
}

pub async fn index(req: HttpRequest) -> impl Responder {
    let css = req.url_for("static", &["style", "index_style.css"]).unwrap();
    let favicon = req.url_for("static", &["images", "favicon.ico"]).unwrap();
    let new = req.url_for_static("new").unwrap();
    let url = req.url_for_static("index").unwrap();

    let template = IndexTemplate {
        css: css.as_str(),
        favicon: favicon.as_str(),
        new: new.as_str(),
        url: url.as_str(),
    };
    
    HttpResponse::Ok().body(template.render().unwrap())
}