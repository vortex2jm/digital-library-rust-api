use crate::domain::traits::UserRepository;
use crate::domain::user::{self, User};
use crate::domain::errors::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryUserRepository {
  users: RwLock<HashMap<Uuid, User>>
}

impl InMemoryUserRepository {
  fn new() -> Self {
    InMemoryUserRepository { users: RwLock::new(HashMap::new()) }
  }
}

impl UserRepository for InMemoryUserRepository {
  fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
    let users = self
      .users
      .read()
      .map_err(|_| {
        DomainError::LockError
      })?;
    Ok(users.get(&id).cloned())
  }

  fn find_user_by_cpf(&self, cpf: String) -> Result<Option<User>, DomainError> {
    let users = self
      .users
      .read()
      .map_err(|_| {
        DomainError::LockError
      })?;
    let user = users.values().find(|u| u.cpf == cpf);
    Ok(user.cloned())
  }

  fn save_user(&self, user: User) -> Result<(), DomainError> {
    let mut users = self
      .users
      .write()
      .map_err(|_| {
        DomainError::LockError
      })?;
    users.insert(user.id, user);
    Ok(())
  }

  fn del_user(&self, cpf: String) -> Result<(), DomainError> {
    let mut users = self
      .users
      .write()
      .map_err(|_| {
        DomainError::LockError
      })?;
    let user = users.values().find(|u| u.cpf == cpf);
    // If user is not found, return an error. Necessary cause unwrap() is used and it can panic.
    if user.is_none() {
      return Err(DomainError::UserNotFound);
    }
    let id = user.unwrap().id;
    users.remove(&id);
    Ok(())
  }
}
