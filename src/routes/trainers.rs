use actix_web::{web,Scope};
use crate::controllers::trainer_controller;


pub fn trainers_routes() -> Scope {
    web::scope("tr")
        .service(trainer_controller::create_trainer)
        .service(trainer_controller::get_trainers)
        .service(trainer_controller::get_trainer_by_id)
        .service(trainer_controller::update_trainer)
        .service(trainer_controller::delete_trainer)
        .service(trainer_controller::get_trainers_specialty)
        
}