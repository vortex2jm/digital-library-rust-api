use serde::{ Serialize, Deserialize };
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Book {
  pub id: Uuid, // Autogenerate
  pub title: String,
  pub author: String,
  pub loc_for: Option<Uuid>  // User id 
}

impl Book {
  pub fn new(title: String, author: String) -> Self {
    let id = Uuid::new_v4();
    Book { id, title, author, loc_for: None }
  }

  pub fn is_available(&self) -> bool {
    self.loc_for.is_none()
  }

  pub fn rm_renter(&mut self) {
    self.loc_for = None;
  }

  pub fn add_renter(&mut self, usr_id: Uuid) {
    self.loc_for = Some(usr_id);
  }
}
