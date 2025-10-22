use actix_web::{get, post,put,delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::trainer::{Trainer, CreateTrainer, UpdateTrainer};
use crate::controllers::class_controller;




pub async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS trainers (
            trainer_id INTEGER PRIMARY KEY AUTOINCREMENT,
            fname TEXT NOT NULL,
            lname TEXT NOT NULL,
            specialty TEXT NOT NULL,
            phone TEXT NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;
    println!("🏋️ Trainers table ensured.");
    Ok(())
}


#[post("/trainers")]
pub async fn create_trainer(pool: web::Data<SqlitePool>, new_trainer: web::Json<CreateTrainer>) -> impl Responder {
    match sqlx::query(
        "INSERT INTO trainers (fname, lname, specialty, phone) VALUES (?, ?, ?, ?)"
    )
    .bind(&new_trainer.fname)
    .bind(&new_trainer.lname)
    .bind(&new_trainer.specialty)
    .bind(&new_trainer.phone)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let inserted_id = result.last_insert_rowid();
            let created_trainer = Trainer {
                trainer_id: inserted_id as i32,
                fname: new_trainer.fname.clone(),
                lname: new_trainer.lname.clone(),
                specialty: new_trainer.specialty.clone(),
                phone: new_trainer.phone.clone(),
            };
            HttpResponse::Ok().json(created_trainer)
        },
        Err(e) => {
            eprintln!("Failed to create trainer: {}", e);
            HttpResponse::InternalServerError().body("Failed to create trainer")
        }
    }
}

#[get("/trainers")]
pub async fn get_trainers(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT * FROM trainers")
    .fetch_all(pool.get_ref())
    .await 
    {
        Ok(trainers) => {
        let trainers:Vec<Trainer> = trainers.into_iter().map(|row| Trainer {
            trainer_id: row.get("trainer_id"),
            fname: row.get("fname"),
            lname: row.get("lname"),
            specialty: row.get("specialty"),
            phone: row.get("phone"),
        }).collect();
        HttpResponse::Ok().json(trainers)
        },
        Err(e) => {
            eprintln!("Failed to fetch trainers: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch trainers")
        }
    }
}
#[get("/trainers/{id}")]
pub async fn get_trainer_by_id(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let trainer_id = path.into_inner();
    match sqlx::query("SELECT * FROM trainers WHERE trainer_id = ?")
        .bind(trainer_id)
        .fetch_one(pool.get_ref())
        .await 
    {
        Ok(row) => {
            let trainer = Trainer {
                trainer_id: row.get("trainer_id"),
                fname: row.get("fname"),
                lname: row.get("lname"),
                specialty: row.get("specialty"),
                phone: row.get("phone"),
            };
            HttpResponse::Ok().json(trainer)
        },
        Err(e) => {
            eprintln!("Failed to fetch trainer: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch trainer")
        }
    }
}

#[put("/trainers/{id}")]
pub async fn update_trainer(pool: web::Data<SqlitePool>, path: web::Path<i32>, updated_trainer: web::Json<UpdateTrainer>) -> impl Responder {
    let trainer_id = path.into_inner();
    match sqlx::query(
        "UPDATE trainers SET fname = ?, lname = ?, specialty = ?, phone = ? WHERE trainer_id = ?"
    )
    .bind(&updated_trainer.fname)
    .bind(&updated_trainer.lname)
    .bind(&updated_trainer.specialty)
    .bind(&updated_trainer.phone)
    .bind(trainer_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            let trainer = Trainer {
                trainer_id,
                fname: updated_trainer.fname.clone(),
                lname: updated_trainer.lname.clone(),
                specialty: updated_trainer.specialty.clone(),
                phone: updated_trainer.phone.clone(),
            };
            HttpResponse::Ok().json(trainer)
        },
        Err(e) => {
            eprintln!("Failed to update trainer: {}", e);
            HttpResponse::InternalServerError().body("Failed to update trainer")
        }
    }
}


#[delete("/trainers/{id}")]
pub async fn delete_trainer(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let trainer_id = path.into_inner();

    match sqlx::query(
        // Before deleting the trainer, delete associated classes
        "DELETE FROM classes WHERE trainer_id = ?"
    )
    .bind(trainer_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to delete classes for trainer: {}", e);
            return HttpResponse::InternalServerError().body("Failed to delete trainer classes");
        }
    }




    match sqlx::query(
        "DELETE FROM trainers WHERE trainer_id = ?"
    )
    .bind(trainer_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => HttpResponse::Ok().body("Trainer deleted successfully"),
        Err(e) => {
            eprintln!("Failed to delete trainer: {}", e);
            HttpResponse::InternalServerError().body("Failed to delete trainer")
        }
    }
}


//return the trainers by specialty
#[get("/trainers/specialty/{specialty}")]
pub async fn get_trainers_specialty(pool: web::Data<SqlitePool>, path: web::Path<String>) -> impl Responder {
    let spec = path.into_inner();
    match sqlx::query("SELECT * FROM trainers WHERE specialty = ?")
        .bind(spec)
        .fetch_all(pool.get_ref())
        .await 
    {
        Ok(rows) => {
            let trainers:Vec<Trainer> = rows.into_iter().map(|row| Trainer {
                trainer_id: row.get("trainer_id"),
                fname: row.get("fname"),
                lname: row.get("lname"),
                specialty: row.get("specialty"),
                phone: row.get("phone"),
            }).collect();
            HttpResponse::Ok().json(trainers)
        },
        Err(e) => {
            eprintln!("Failed to fetch trainers by specialty: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch trainers by specialty")
        }
    }
}