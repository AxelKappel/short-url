use actix_web::{HttpRequest, Responder};

use super::error;

pub async fn default(req: HttpRequest) -> impl Responder {
    error::render(req, "Page not found")
}