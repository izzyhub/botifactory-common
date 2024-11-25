use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectJson {
    pub id: i64,
    pub name: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize, Deserialize)]
pub struct CreateProject {
    pub name: String,
}

impl CreateProject {
    pub fn new(name: &str) -> Self {
        CreateProject {
            name: name.to_string(),
        }
    }
}

impl ProjectJson {
    pub fn new(id: i64, name: &str, created_at: i64, updated_at: i64) -> Self {
        ProjectJson {
            id,
            name: name.to_string(),
            created_at,
            updated_at,
        }
    }
}
