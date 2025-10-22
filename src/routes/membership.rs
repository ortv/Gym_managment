
use actix_web::{web,Scope};
use crate::controllers::membership_controller;

pub fn membership_routes() -> Scope {
    web::scope("ms")
        .service(membership_controller::create_membership)
        .service(membership_controller::get_memberships)
        .service(membership_controller::get_membership)
        .service(membership_controller::update_membership)
        .service(membership_controller::delete_membership)
        .service(membership_controller::actual_payment)
}