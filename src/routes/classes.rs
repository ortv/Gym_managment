use actix_web::{web,Scope};
use crate::controllers::class_controller;


pub fn classes_routes() -> Scope {
    web::scope("cl")
        .service(class_controller::create_class)
        .service(class_controller::get_classes)
        .service(class_controller::get_class_by_id)
        .service(class_controller::update_class)
        .service(class_controller::delete_class)
        .service(class_controller::amount_classes_for_trainer)
        
}