use uuid::Uuid;
use crate::domain::{traits::{UserRepository, BookRepository}, errors::DomainError};
use crate::domain::user::User;

pub struct UserService<U: UserRepository, B: BookRepository> {
    user_repository: U,
    book_repository: B,
}

impl<U: UserRepository, B: BookRepository> UserService<U, B> {
    pub fn new(user_repository: U, book_repository: B) -> Self {
        Self { user_repository, book_repository }
    }

    pub fn add_user(&self, user: User) -> Result<(), DomainError> {
        self.user_repository.save_user(&user)
    }

    pub fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.user_repository.find_user_by_id(id)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        self.user_repository.find_all_users()
    }

    pub fn remove_user_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        self.user_repository.del_user_by_id(id)
    }

    pub fn rent_book(&self, user_id: Uuid, book_id: Uuid) -> Result<(), DomainError> {
        
        let mut user = self.user_repository.find_user_by_id(user_id)?
            .ok_or(DomainError::UserNotFound)?;

        let mut book = self.book_repository.find_book_by_id(book_id)?
            .ok_or(DomainError::BookNotFound)?;

        if !book.is_available() {
            return Err(DomainError::BookNotAvailable);
        }

        // Associar o livro ao usuário
        book.add_renter(user.id);
        self.book_repository.save_book(&book)?;

        // Adicionar o ID do livro à lista de livros alugados pelo usuário
        user.rent_book(book.id);
        self.user_repository.save_user(&user)?;

        Ok(())
    }

    pub fn return_book(&self, user_id: Uuid, book_id: Uuid) -> Result<(), DomainError> {

        let mut user = self.user_repository.find_user_by_id(user_id)?
            .ok_or(DomainError::UserNotFound)?;

        let mut book = self.book_repository.find_book_by_id(book_id)?
            .ok_or(DomainError::BookNotFound)?;

        if !user.loc_books.contains(&book.id) {
            return Err(DomainError::BookNotRentedByUser);
        }

        // Remover o locatário do livro (torná-lo disponível novamente)
        book.rm_renter();
        self.book_repository.save_book(&book)?;

        // Remover o livro da lista de livros alugados pelo usuário
        user.loc_books.retain(|&id| id != book_id);
        self.user_repository.save_user(&user)?;

        Ok(())
    }

}
