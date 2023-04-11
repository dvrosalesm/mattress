use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mattress {
    pub owner: String,
}

#[derive(Serialize, Deserialize)]
pub enum Claims {
    Name = 0,
    DOB = 1,
    Email = 2,
}

#[derive(Serialize, Deserialize)]
pub struct Credential {
    pub url: String,
    pub expiration: usize,
    pub claims: Vec<Claims>,
}

