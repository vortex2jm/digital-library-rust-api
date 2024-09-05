use crate::domain::user::User;
use std::collections::HashMap;

pub struct InMemoryUserRepository {
  users: HashMap<u32, User>
}

impl InMemoryUserRepository {
  fn new() -> Self {
    InMemoryUserRepository { users: HashMap::new() }
  }
}
