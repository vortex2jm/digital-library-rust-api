use serde::{Serialize, Deserialize};
use uuid::Uuid;

// DTO para representar um livro
#[derive(Debug, Serialize, Deserialize)]
pub struct BookDTO {
    pub id: Option<Uuid>,       // Autogenerate na criação
    pub title: String,
    pub author: String,
    pub loc_for: Option<Uuid>,  // Representa o ID do usuário que alugou
}

impl BookDTO {
    // Converte a entidade Book para o DTO
    pub fn from_entity(book: &crate::domain::book::Book) -> Self {
        Self {
            id: Some(book.id),
            title: book.title.clone(),
            author: book.author.clone(),
            loc_for: book.loc_for,
        }
    }

    // Converte o DTO para a entidade Book
    pub fn into_entity(self) -> crate::domain::book::Book {
        crate::domain::book::Book {
            id: self.id.unwrap_or_else(Uuid::new_v4),  // Gera ID se não fornecido
            title: self.title,
            author: self.author,
            loc_for: self.loc_for,
        }
    }
}

// DTO para representar um usuário
#[derive(Debug, Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Option<Uuid>,        // Autogenerate na criação
    pub name: String,
    pub cpf: String,
    pub loc_books: Vec<Uuid>,    // Representa os IDs dos livros alugados
}

impl UserDTO {
    // Converte a entidade User para o DTO
    pub fn from_entity(user: &crate::domain::user::User) -> Self {
        Self {
            id: Some(user.id),
            name: user.name.clone(),
            cpf: user.cpf.clone(),
            loc_books: user.loc_books.clone(),
        }
    }

    // Converte o DTO para a entidade User
    pub fn into_entity(self) -> crate::domain::user::User {
        crate::domain::user::User {
            id: self.id.unwrap_or_else(Uuid::new_v4),  // Gera ID se não fornecido
            name: self.name,
            cpf: self.cpf,
            loc_books: self.loc_books,
        }
    }
}