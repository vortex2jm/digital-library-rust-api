use crate::domain::{book::Book, errors::DomainError, traits::BookRepository};
use uuid::Uuid;

pub struct BookService<B: BookRepository> {
  book_repository: B,
}

impl <B: BookRepository> BookService<B> {
  pub fn get_book(&self, id: &Uuid) -> Result<Option<Book>, DomainError> {
    self.book_repository.find_book_by_id(*id)
  }
} 
