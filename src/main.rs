use actix_web::{web, App, HttpServer, HttpRequest, Result};
use actix_files::{NamedFile};

use std::sync::RwLock;
use std::collections::HashMap;
use std::path::PathBuf;

mod db;
mod url_validator;
mod error;
mod index;
mod new;
mod redirect;

pub struct AppState {
    cache: RwLock<HashMap<std::string::String, std::string::String>>,
}

async fn static_files(req: HttpRequest) -> Result<NamedFile> {
    let path: PathBuf = ["static", req.match_info().query("directory"), req.match_info().query("filename")].iter().collect();
    Ok(NamedFile::open(path)?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        cache: RwLock::new(HashMap::new()),
    });

    HttpServer::new(move ||{
        App::new()
            .app_data(data.clone())
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
                .route(web::get().to(static_files))
            )
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}