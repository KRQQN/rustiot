use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
}
