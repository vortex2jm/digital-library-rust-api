use super::{ user::User, book::Book };

pub trait BookRepository {
  fn find_book_by_id(&self, id: u32) -> Option<Book>;
  fn save_book(&self, book: &Book);
  fn del_book(&self, id: u32);  
}

pub trait UserRepository {
  fn save_user(&self, user: User);
  fn find_user_by_cpf(&self, cpf: String) -> Option<User>;
  fn find_user_by_id(&self, id: u32) -> Option<User>;
  fn del_user(&self, cpf: String);
}
