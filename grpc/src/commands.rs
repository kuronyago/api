use serde_derive::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewProject {
    pub creator: Uuid,
    pub budget: i32,
}