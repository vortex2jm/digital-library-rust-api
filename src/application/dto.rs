use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct BookDTO {
    #[validate(length(min = 1, message = "Title cannot be empty"))]
    pub title: String,
    
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub author: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserDTO {
    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub mail: String,
    
    #[validate(length(min = 11, max = 11, message = "Invalid cpf format"))]
    pub cpf: String,
}
