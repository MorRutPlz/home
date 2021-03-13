use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct AuthResponse {
    pub id: Uuid,
    pub name: String,
}
