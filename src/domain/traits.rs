use super::{ book::Book, errors::DomainError, user::User };
use uuid::Uuid;

pub trait BookRepository {
  fn save_book(&self, book: &Book) -> Result<Uuid, DomainError>;
  fn find_book_by_id(&self, id: Uuid) -> Result<Option<Book>, DomainError>;
  fn get_all_books(&self) -> Result<Vec<Book>, DomainError>;
  fn del_book_by_id(&self, id: Uuid) -> Result<(), DomainError>;  
}

pub trait UserRepository {
  fn save_user(&self, user: &User) -> Result<Uuid, DomainError>;
  fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
  fn find_user_by_cpf(&self, cpf: String) -> Result<Option<User>, DomainError>;
  fn get_all_users(&self) -> Result<Vec<User>, DomainError>;
  fn del_user_by_id(&self, id: Uuid) -> Result<(), DomainError>;
}
