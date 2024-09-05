use super::{ book::Book, errors::DomainError, user::User };
use uuid::Uuid;

pub trait BookRepository {
  fn del_book(&self, id: Uuid) -> Result<(), DomainError>;  
  fn save_book(&self, book: &Book) -> Result<(), DomainError>;
  fn find_book_by_id(&self, id: Uuid) -> Result<Option<Book>, DomainError>;
}

pub trait UserRepository {
  fn save_user(&self, user: User) -> Result<(), DomainError>;
  fn del_user(&self, cpf: String) -> Result<(), DomainError>;
  fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
  fn find_user_by_cpf(&self, cpf: String) -> Result<Option<User>, DomainError>;
}
