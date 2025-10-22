use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Class{
    pub class_id:i32,
    pub class_name:String,
    pub schedule:String,
    pub trainer_id:i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateClass{
    pub class_name:String,
    pub schedule:String,
    pub trainer_id:i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateClass{
    pub class_name:String,
    pub schedule:String,
    pub trainer_id:i32,
}