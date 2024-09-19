mod application;
mod adapters;
mod domain;

use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use adapters::{repositories::book_repository::InMemoryBookRepository, web::routes_config::rt_config};
use application::book_service::BookService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let book_repo = Arc::new(InMemoryBookRepository::new());
    let book_serv = web::Data::new(BookService::new(book_repo));

    HttpServer::new( move || {
        App::new()
            .app_data(book_serv.clone())
            .configure(rt_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
