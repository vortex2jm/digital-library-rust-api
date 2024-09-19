use crate::{application::{dto::UserDTO, user_service::UserService}, domain::errors::DomainError};
use actix_web::{get, post, delete, web, HttpResponse, Responder};
use uuid::Uuid;
use super::http_config::Response;

#[post("/user")]
pub async fn add_user(
  user_service: web::Data<UserService>,
  user: web::Json<UserDTO>
) -> impl Responder {
  match user_service.add_user(user.into_inner()) {
    Ok(id) => HttpResponse::Created().json(Response { message: format!("{}", id) }),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[get("/user")]
pub async fn get_users(
  user_service: web::Data<UserService>
) -> impl Responder {
  match user_service.get_all_users() {
    Ok(users) => HttpResponse::Ok().json(users),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[get("/user/{cpf}")]
pub async fn get_user_by_cpf(
  user_service: web::Data<UserService>,
  path: web::Path<String>
) -> impl Responder {
  let cpf = path.into_inner();
  match user_service.get_user_by_cpf(&cpf) {
    Ok(Some(user)) => HttpResponse::Ok().json(user),
    Ok(None) => HttpResponse::NotFound().json(Response{ message: format!("User {} not found", &cpf)}),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

#[delete("/user/{id}")]
pub async fn delete_user_by_id(
  user_service: web::Data<UserService>,
  path: web::Path<Uuid>
) -> impl Responder{
  
  let id = path.into_inner();
  
  match user_service.remove_user_by_id(id) {
    Ok(()) => HttpResponse::Accepted().finish(),
    Err(error) => match error {
      
      DomainError::UserNotFound => HttpResponse::NotFound().json(Response{ message: format!("User {} not found", id)}),
      _ => HttpResponse::InternalServerError().finish()
    }
  }
}

#[post("/rent/{user_id}/{book_id}")]
pub async fn rent_book(
  user_service: web::Data<UserService>,
  path: web::Path<(Uuid, Uuid)>
) -> impl Responder{
  let (user_id, book_id) = path.into_inner();
  match user_service.rent_book(user_id, book_id) {
    Ok(()) => HttpResponse::Ok().finish(),
    Err(error) => match error {
      DomainError::BookNotFound => HttpResponse::NotFound().json(Response{ message: format!("Book {} not found", book_id)}),
      DomainError::BookNotAvailable => HttpResponse::NotFound().json(Response{ message: format!("Book {} is not available", book_id)}),
      DomainError::UserNotFound => HttpResponse::NotFound().json(Response{ message: format!("User {} not found", user_id)}),
      _ => HttpResponse::InternalServerError().finish()
    }
  } 
}

#[post("/return/{user_id}/{book_id}")]
pub async fn return_book(
  user_service: web::Data<UserService>,
  path: web::Path<(Uuid, Uuid)>
) -> impl Responder{
  let (user_id, book_id) = path.into_inner();
  match user_service.return_book(user_id, book_id) {
    Ok(()) => HttpResponse::Ok().finish(),
    Err(error) => match error {
      DomainError::BookNotFound => HttpResponse::NotFound().json(Response{ message: format!("Book {} not found", book_id)}),
      DomainError::BookNotRentedByUser => HttpResponse::NotFound().json(Response{ message: format!("Book {} was not rented by the user {}", book_id, user_id)}),
      DomainError::UserNotFound => HttpResponse::NotFound().json(Response{ message: format!("User {} not found", user_id)}),
      _ => HttpResponse::InternalServerError().finish()
    }
  } 
}
