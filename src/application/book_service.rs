use uuid::Uuid;

use crate::domain::{traits::BookRepository, errors::DomainError};
use crate::domain::book::Book;

pub struct BookService<R: BookRepository> {
    book_repository: R,
}

impl<R: BookRepository> BookService<R> {
    pub fn new(book_repository: R) -> Self {
        Self { book_repository }
    }

    pub fn add_book(&self, book: Book) -> Result<(), DomainError> {
        self.book_repository.save_book(&book)
    }

    pub fn get_book_by_id(&self, id: Uuid) -> Result<Option<Book>, DomainError> {
        self.book_repository.find_book_by_id(id)
    }

    pub fn get_all_books(&self) -> Result<Vec<Book>, DomainError> {
        self.book_repository.find_all_books()
    }

    pub fn remove_book_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        self.book_repository.del_book_by_id(id)
    }
}
