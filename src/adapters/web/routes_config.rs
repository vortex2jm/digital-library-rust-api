use actix_web::web;
use super::books_controller;

pub fn rt_config (cfg: &mut web::ServiceConfig) {
  cfg
    .service(books_controller::add_book)
    .service(books_controller::delete_book_by_id)
    .service(books_controller::get_book_by_id)
    .service(books_controller::get_books);
}
