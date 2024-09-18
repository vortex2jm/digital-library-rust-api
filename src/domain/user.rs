use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
  pub id: Uuid, // Autogenerate
  pub name: String,
  pub cpf: String,
  pub loc_books: Vec<Uuid> // Book id's
}

impl User {
  fn new(name: String, cpf: String) -> Self {
    let id = Uuid::new_v4();
    User { id, name, cpf, loc_books: vec![]}
  }

  pub fn rent_book(&mut self, book_id: Uuid) {
    self.loc_books.push(book_id);
  }

  pub fn return_book(&mut self, book_id: Uuid) {
    self.loc_books.retain(|&id| id != book_id);
  }
}
