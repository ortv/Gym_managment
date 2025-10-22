use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use crate::controllers::member_controller;
use crate::controllers::membership_controller;
use crate::controllers::trainer_controller;
use crate::controllers::class_controller;
use crate::controllers::enrollment_controller;
use crate::routes::members::members_routes;
use crate::routes::membership::membership_routes;

mod controllers;
mod models; 
mod routes;




async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    membership_controller::init_db(pool).await?;
    member_controller::init_db(pool).await?;
    trainer_controller::init_db(pool).await?;
    class_controller::init_db(pool).await?;
    enrollment_controller::init_db(pool).await?;
    Ok(())
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database
    let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await.expect("Failed to connect to the database");
    init_db(&pool).await.expect("Failed to initialize database");
    println!("🚀 Server running at http://127.0.0.1:3004");
    println!("📊 SQLite database initialized at src/mydb.db");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(routes::membership::membership_routes())
            .service(routes::members::members_routes())
            .service(routes::trainers::trainers_routes())
            .service(routes::classes::classes_routes())
            .service(routes::enrollments::enrollments_routes())
    })
    .bind(("127.0.0.1", 3004))?
    .run()
    .await
}



  