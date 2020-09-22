use actix_web::{web, App, HttpServer};

use std::sync::RwLock;
use std::collections::HashMap;

mod db;
mod url_validator;
mod error;
mod index;
mod new;
mod redirect;
mod default;

pub struct AppState {
    cache: RwLock<HashMap<std::string::String, std::string::String>>,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        cache: RwLock::new(HashMap::new()),
    });

    HttpServer::new(move ||{
        App::new()
            .app_data(data.clone())
            .service(actix_files::Files::new("/static", "./static"))
            .service(web::resource("/")
                .name("index")
                .route(web::get().to(index::index))
            )
            .service(web::resource("/new")
                .name("new")
                .route(web::post().to(new::new)),
            )
            .service(web::resource("/{url}")
                .name("redirect")
                .route(web::get().to(redirect::redirect))
            )
            .service(web::resource("/static/{directory}/{filename}")
                .name("static")
            )
            .default_service(
                web::get().to(default::default)
            )
    })
    .bind("127.0.0.1:80")?
    .run()
    .await
}