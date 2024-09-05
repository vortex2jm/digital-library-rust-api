use crate::domain::traits::{ BookRepository, UserRepository };
use crate::domain::book::Book;
use std::collections::HashMap;

pub struct InMemoryBookRepository {
  books: HashMap<u32, Book>
}
