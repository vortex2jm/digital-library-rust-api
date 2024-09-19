use crate::domain::{traits::{UserRepository, BookRepository}, errors::DomainError};
use crate::domain::user::User;
use super::dto::UserDTO;
use std::sync::Arc;
use uuid::Uuid;


pub struct UserService {
    user_repository: Arc<dyn UserRepository + Sync + Send>,
    book_repository: Arc<dyn BookRepository + Sync + Send>
}

impl UserService {
    pub fn new(
        user_repository: Arc<dyn UserRepository + Sync + Send>, 
        book_repository: Arc<dyn BookRepository + Sync + Send>
    ) -> Self {
        Self { user_repository, book_repository }
    }

    pub fn add_user(&self, user: UserDTO) -> Result<Uuid, DomainError> {
        let n_usr = User::new(user.name, user.mail, user.cpf);
        self.user_repository.save_user(&n_usr)
    }

    pub fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        self.user_repository.find_user_by_id(id)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
        self.user_repository.get_all_users()
    }

    pub fn get_user_by_cpf(&self, cpf: &String) -> Result<Option<User>, DomainError> {
        self.user_repository.find_user_by_cpf(cpf.clone())
    }

    pub fn remove_user_by_id(&self, id: Uuid) -> Result<(), DomainError> {
        match self.get_user_by_id(id) {
            Ok(Some(_)) => self.user_repository.del_user_by_id(id),
            Ok(None) => Err(DomainError::UserNotFound),
            Err(error) => Err(error)
        }
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
        user.return_book(book_id);
        // user.loc_books.retain(|&id| id != book_id);
        self.user_repository.save_user(&user)?;

        Ok(())
    }

}
