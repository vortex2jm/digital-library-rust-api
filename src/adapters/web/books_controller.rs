use actix_web::{get, post, delete, web, HttpResponse, Responder};
use crate::application::book_service::BookService;
use crate::application::dto::BookDTO;
use crate::domain::errors::DomainError;
use actix_web::web::Data;
use uuid::Uuid;

#[derive(serde::Serialize)]
struct Response {
  message: String
}

#[post("/book")]
pub async fn add_book(
  book_service: Data<BookService>,
  book: web::Json<BookDTO>
) -> impl Responder {
  match book_service.add_book(book.into_inner()) {
      Ok(id) => HttpResponse::Created().json(Response { message: format!("{}", id) }),
      Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[get("/book")]
pub async fn get_books(
  book_service: Data<BookService>
) -> impl Responder {
  match book_service.get_all_books() {
    Ok(books) => HttpResponse::Ok().json(books),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[get("/book/{id}")]
pub async fn get_book_by_id(
  book_service: Data<BookService>,
  path: web::Path<Uuid>
) -> impl Responder{
  let id = path.into_inner();
  match book_service.get_book_by_id(id) {
    Ok(Some(book)) => HttpResponse::Ok().json(book),
    Ok(None) => HttpResponse::NotFound().json(Response{ message: format!("Book {} not found", id)}),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[delete("/book/{id}")]
pub async fn delete_book_by_id(
  book_service: Data<BookService>,
  path: web::Path<Uuid>
) -> impl Responder{
  
  let id = path.into_inner();
  
  match book_service.remove_book_by_id(id) {
    Ok(()) => HttpResponse::Accepted().finish(),
    Err(error) => match error {
      
      DomainError::BookNotFound => HttpResponse::NotFound().json(Response{ message: format!("Book {} not found", id)}),
      _ => HttpResponse::InternalServerError().finish()
    }
  }
}
