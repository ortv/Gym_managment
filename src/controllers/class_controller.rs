use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::class::{Class, CreateClass, UpdateClass};




pub async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS classes (
            class_id INTEGER PRIMARY KEY AUTOINCREMENT,
            class_name TEXT NOT NULL,
            schedule TEXT NOT NULL,
            trainer_id INTEGER NOT NULL,
            FOREIGN KEY (trainer_id) REFERENCES trainers(trainer_id)
        );
        "#
    )
    .execute(pool)
    .await?;
    println!("📚 Classes table ensured.");
    Ok(())
}

#[post("/classes")]
pub async fn create_class(pool: web::Data<SqlitePool>, new_class: web::Json<CreateClass>) -> impl Responder {
    match sqlx::query(
        "INSERT INTO classes (class_name, schedule, trainer_id) VALUES (?, ?, ?)"
    )
    .bind(&new_class.class_name)
    .bind(&new_class.schedule)
    .bind(new_class.trainer_id)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let inserted_id = result.last_insert_rowid();
            let created_class = Class {
                class_id: inserted_id as i32,
                class_name: new_class.class_name.clone(),
                schedule: new_class.schedule.clone(),
                trainer_id: new_class.trainer_id,
            };
            HttpResponse::Ok().json(created_class)
        },
        Err(e) => {
            eprintln!("Failed to create class: {}", e);
            HttpResponse::InternalServerError().body("Failed to create class")
        }
    }
}

#[get("/classes")]
pub async fn get_classes(pool: web::Data<SqlitePool>) -> impl Responder {
    match  sqlx::query("SELECT * FROM classes")
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(classes) => {
            let classes:Vec<Class> = classes.into_iter().map(|row| Class{
                class_id:row.get("class_id"),
                class_name:row.get("class_name"),
                schedule:row.get("schedule"),
                trainer_id:row.get("trainer_id"),
            }).collect();
        HttpResponse::Ok().json(classes)
        },
        Err(e) => {
            eprintln!("Failed to fetch classes: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch classes")
        }
    }
}

//return the classes of a given if of trainer
#[get("/classes/{id}")]
pub async fn get_class_by_id(pool: web::Data<SqlitePool>, class_id: web::Path<i32>) -> impl Responder {
   let classId= class_id.into_inner();
   match sqlx::query("SELECT * FROM classes WHERE class_id = ?")
        .bind(classId)
        .fetch_one(pool.get_ref())
        .await {
            Ok(row) => {
                let class = Class {
                    class_id: row.get("class_id"),
                    class_name: row.get("class_name"),
                    schedule: row.get("schedule"),
                    trainer_id: row.get("trainer_id"),
                };
                HttpResponse::Ok().json(class)
            },
            Err(e) => {
                eprintln!("Failed to fetch class: {}", e);
                HttpResponse::InternalServerError().body("Failed to fetch class")
            }
        }
}
#[put("/classes/{id}")]
pub async fn update_class(pool: web::Data<SqlitePool>, class_id: web::Path<i32>, updated_class: web::Json<UpdateClass>) -> impl Responder {
    let classId = class_id.into_inner();
    match sqlx::query(
        "UPDATE classes SET class_name = ?, schedule = ?, trainer_id = ? WHERE class_id = ?"
    )
    .bind(&updated_class.class_name)
    .bind(&updated_class.schedule)
    .bind(updated_class.trainer_id)
    .bind(classId)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            let class = Class {
                class_id: classId,
                class_name: updated_class.class_name.clone(),
                schedule: updated_class.schedule.clone(),
                trainer_id: updated_class.trainer_id,
            };
            HttpResponse::Ok().json(class)
        },
        Err(e) => {
            eprintln!("Failed to update class: {}", e);
            HttpResponse::InternalServerError().body("Failed to update class")
        }
    }
}

#[delete("/classes/{id}")]
pub async fn delete_class(pool: web::Data<SqlitePool>, class_id: web::Path<i32>) -> impl Responder {
    let classId = class_id.into_inner();

    match sqlx::query("DELETE FROM enrollments WHERE class_id = ?")
        .bind(classId)
        .execute(pool.get_ref())
        .await {
            Ok(_) => (),
            Err(e) => {
                eprintln!("Failed to delete enrollments for class: {}", e);
                return HttpResponse::InternalServerError().body("Failed to delete class enrollments");
            }
        }



    match sqlx::query("DELETE FROM classes WHERE class_id = ?")
    .bind(classId)
    .execute(pool.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().body("Class deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete class: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete class")
        }
    }
}

#[get("/classes/count/{trainer_id}")]
pub async fn amount_classes_for_trainer(pool: web::Data<SqlitePool>,trainer_id: web::Path<i32>) -> impl Responder {
    let id = trainer_id.into_inner();

    match sqlx::query("SELECT COUNT(*) as class_count FROM classes WHERE trainer_id = ?")
        .bind(id)
        .fetch_one(pool.get_ref())
        .await 
    {
        Ok(row) => {
            let count: i32 = row.get("class_count");
            HttpResponse::Ok().json(count) // Wrap count in a JSON response
        },
        Err(e) => {
            eprintln!("Failed to fetch class count: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch class count")
        }
    }
}

