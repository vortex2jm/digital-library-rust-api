use crate::domain::traits::BookRepository;
use crate::domain::errors::DomainError;
use crate::domain::book::Book;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryBookRepository {
  books: RwLock<HashMap<Uuid, Book>>
}

impl InMemoryBookRepository {
  fn new() -> Self {
    InMemoryBookRepository { books: RwLock::new(HashMap::new()) }
  }
}

impl BookRepository for InMemoryBookRepository {
  fn find_book_by_id(&self, id: Uuid) -> Result<Option<Book>, DomainError> {
      let books = self
        .books
        .read()
        .map_err(|_| {
        DomainError::LockError
      })?;
      Ok(books.get(&id).cloned()) // Returns a copy, but could returns a reference
  }

  fn save_book(&self, book: &Book) -> Result<(), DomainError> {
      let mut books = self
        .books
        .write()
        .map_err(|_| {
        DomainError::LockError
      })?;
        books.insert(book.id.clone(), book.clone());   // Saves a copy
        Ok(())
  }

  fn del_book_by_id(&self, id: Uuid) -> Result<(), DomainError> {
      let mut books = self
        .books
        .write()
        .map_err(|_| {
        DomainError::LockError
      })?;
      books.remove(&id);
      Ok(())
  }

  fn get_all_books(&self) -> Result<Vec<Book>, DomainError> {
    let books = self.books.read().map_err(|_| DomainError::LockError)?;
    Ok(books.values().cloned().collect())  // Converte os valores do HashMap em um Vec<Book>
}
}
