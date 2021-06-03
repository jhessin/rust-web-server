//! Holds states that can be stored by the server.

#[derive(StateData)]
pub struct User {
  pub email: String,
  pub name: String,
  pub id: String,
}
