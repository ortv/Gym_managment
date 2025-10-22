use actix_web::{web,Scope};
use crate::controllers::enrollment_controller;


pub fn enrollments_routes() -> Scope {
    web::scope("en")
        .service(enrollment_controller::create_enrollment)
        .service(enrollment_controller::get_enrollments)
        .service(enrollment_controller::get_enrollment_by_id)
        .service(enrollment_controller::update_enrollment)
        .service(enrollment_controller::delete_enrollment)
        .service(enrollment_controller::amount_enrollments_for_class)
}