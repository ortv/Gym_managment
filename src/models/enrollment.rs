use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Enrollment{
    pub enrollment_id:i32,
    pub member_id:i32,
    pub class_id:i32,
    pub enrollment_date:String,
}

#[derive(Debug, Deserialize)]
pub struct CreateEnrollment{
    pub member_id:i32,
    pub class_id:i32,
    pub enrollment_date:String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateEnrollment{
    pub member_id:i32,
    pub class_id:i32,
    pub enrollment_date:String,
}