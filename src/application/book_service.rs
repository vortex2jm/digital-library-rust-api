use std::sync::Arc;

use crate::domain::{traits::BookRepository, errors::DomainError};
use crate::domain::book::Book;
use uuid::Uuid;

use super::dto::BookDTO;

pub struct BookService {
    book_repository: Arc<dyn BookRepository + Sync + Send>
}

impl BookService {
    pub fn new(book_repository: Arc<dyn BookRepository + Sync + Send>) -> Self {
        Self { book_repository }
    }

    pub fn add_book(&self, book: BookDTO) -> Result<Uuid, DomainError> {
        let nb = Book::new(book.title, book.author);
        self.book_repository.save_book(&nb)
    }

    pub fn get_book_by_id(&self, id: Uuid) -> Result<Option<Book>, DomainError> {
        self.book_repository.find_book_by_id(id)
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>, DomainError> {
        self.book_repository.get_all_books()
    }

    pub fn remove_book_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        match self.get_book_by_id(id) {
            Ok(Some(_)) => self.book_repository.del_book_by_id(id),
            Ok(None) => Err(DomainError::BookNotFound),
            Err(error) => Err(error)
        }
    }
}
