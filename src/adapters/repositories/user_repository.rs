use crate::domain::traits::UserRepository;
use crate::domain::user::User;
use crate::domain::errors::DomainError;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryUserRepository {
  users: RwLock<HashMap<Uuid, User>>
}

impl InMemoryUserRepository {
  pub fn new() -> Self {
    InMemoryUserRepository { users: RwLock::new(HashMap::new()) }
  }
}

impl UserRepository for InMemoryUserRepository {
  
  fn save_user(&self, user: &User) -> Result<Uuid, DomainError> {
    let mut users = self
      .users
      .write()
      .map_err(|_| {
        DomainError::LockError
      })?;
    users.insert(user.id.clone(), user.clone());
    Ok(user.id.clone())
  }
  
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

  fn get_all_users(&self) -> Result<Vec<User>, DomainError> {
    let users = self.users.read().map_err(|_| DomainError::LockError)?;
    Ok(users.values().cloned().collect())
  }

  fn del_user_by_id(&self, id: Uuid) -> Result<(), DomainError> {
    let mut users = self
      .users
      .write()
      .map_err(|_| {
        DomainError::LockError
      })?;
    users.remove(&id);
    Ok(())
  }
}
