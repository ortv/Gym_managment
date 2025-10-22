use actix_web::{web,Scope};
use crate::controllers::member_controller;



pub fn members_routes() -> Scope {
    web::scope("m")
        .service(member_controller::create_member)
        .service(member_controller::get_members)
        .service(member_controller::get_member)
        .service(member_controller::update_member)
        .service(member_controller::delete_member)
        .service(member_controller::join_cur_year)
}