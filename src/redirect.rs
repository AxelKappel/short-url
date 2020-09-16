use actix_web::{web, HttpResponse, HttpRequest, Responder};

use super::db;
use super::error;
use super::AppState;

pub async fn redirect(data: web::Data<AppState>, req: HttpRequest) -> impl Responder {
    let short_url = req.match_info().get("url").unwrap_or("");
    {
        let cache = data.cache.read().unwrap();
        
        if cache.contains_key(short_url){
            return HttpResponse::MovedPermanently().header("Location", cache[short_url].clone()).finish();
        }
    }

    let long_url = match db::get(short_url) {
        Ok(long_url) => long_url,
        Err(_) => {return error::render(req, "Could not find url");},
    };

    data.cache.write().unwrap().insert(short_url.to_string(), long_url.clone());
    HttpResponse::MovedPermanently().header("Location", long_url).finish()
}