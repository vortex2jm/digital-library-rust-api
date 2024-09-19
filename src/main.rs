mod application;
mod adapters;
mod domain;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use application::{book_service::BookService, user_service::UserService};
use adapters::{repositories::{book_repository::InMemoryBookRepository, user_repository::InMemoryUserRepository}, web::http_config::rt_config};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let book_repo = Arc::new(InMemoryBookRepository::new());
    let book_serv = web::Data::new(BookService::new(book_repo.clone()));

    let user_repo = Arc::new(InMemoryUserRepository::new());
    let user_serv = web::Data::new(UserService::new(user_repo, book_repo.clone()));

    HttpServer::new( move || {
        App::new()
            .app_data(book_serv.clone())
            .app_data(user_serv.clone())
            .configure(rt_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
