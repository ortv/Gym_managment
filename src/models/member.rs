use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Member {
    pub id: i32,
    pub fname: String,
    pub lname: String,
    pub phone: String,
    pub email: String,
    pub joinDate: String,
    pub membershipId: i32,
}


#[derive(Debug, Deserialize)]
pub struct CreateMember{
    pub fname: String,
    pub lname: String,
    pub phone: String,
    pub email: String,
    pub joinDate: String,
    pub membershipId: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateMember{
    pub fname: String,
    pub lname: String,
    pub phone: String,
    pub email: String,
    pub joinDate: String,
    pub membershipId: i32,
}