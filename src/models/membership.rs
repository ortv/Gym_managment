
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Membership{
    pub membershipId: i32,
    pub typeMembership: String,
    pub price: f32,
    pub durationMonths: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateMembership{
    pub typeMembership: String,
    pub price: f32,
    pub durationMonths: i32,
}
#[derive(Debug, Deserialize)]
pub struct UpdateMembership{
    pub typeMembership: String,
    pub price: f32,
    pub durationMonths: i32,
}


