use actix_web::web;
use super::{books_controller, users_controller};

pub fn rt_config (cfg: &mut web::ServiceConfig) {
  cfg
    .service(books_controller::add_book)
    .service(books_controller::delete_book_by_id)
    .service(books_controller::get_book_by_id)
    .service(books_controller::get_books)
    .service(users_controller::add_user)
    .service(users_controller::delete_user_by_id)
    .service(users_controller::get_user_by_cpf)
    .service(users_controller::get_users)
    .service(users_controller::rent_book)
    .service(users_controller::return_book);
}

#[derive(serde::Serialize)]
pub struct Response {
  pub message: String
}
