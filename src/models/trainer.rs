use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Trainer{
    pub trainer_id: i32,
    pub fname: String,
    pub lname: String,
    pub specialty: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTrainer{
    pub fname: String,
    pub lname: String,
    pub specialty: String,
    pub phone: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTrainer{
    pub fname: String,
    pub lname: String,
    pub specialty: String,
    pub phone: String,
}