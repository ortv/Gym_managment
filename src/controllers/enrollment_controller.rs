use actix_web::{get, post, put, delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::enrollment::{Enrollment, CreateEnrollment, UpdateEnrollment};
use crate::models::member::Member;
use crate::models::class::Class;


pub async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS enrollments (
            enrollment_id INTEGER PRIMARY KEY AUTOINCREMENT,
            member_id INTEGER NOT NULL,
            class_id INTEGER NOT NULL,
            enrollment_date TEXT NOT NULL,
            FOREIGN KEY (member_id) REFERENCES members(id),
            FOREIGN KEY (class_id) REFERENCES classes(class_id)
        );
        "#
    )
    .execute(pool)
    .await?;
    println!("📝 Enrollments table ensured.");
    Ok(())
}


#[post("/enrollments")]
pub async fn create_enrollment(pool:web::Data<SqlitePool>,new_enrollment:web::Json<CreateEnrollment>) -> impl Responder {
    match sqlx::query(
        "INSERT INTO enrollments (member_id, class_id, enrollment_date) VALUES (?, ?, ?)"
    )
    .bind(new_enrollment.member_id)
    .bind(new_enrollment.class_id)
    .bind(&new_enrollment.enrollment_date)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let inserted_id = result.last_insert_rowid();
            let created_enrollment = Enrollment {
                enrollment_id: inserted_id as i32,
                member_id: new_enrollment.member_id,
                class_id: new_enrollment.class_id,
                enrollment_date: new_enrollment.enrollment_date.clone(),
            };
            HttpResponse::Ok().json(created_enrollment)
        },
        Err(e) => {
            eprintln!("Failed to create enrollment: {}", e);
            HttpResponse::InternalServerError().body("Failed to create enrollment")
        }
    }
}

#[get("/enrollments")]
pub async fn get_enrollments(pool: web::Data<SqlitePool>) -> impl Responder {
    match sqlx::query("SELECT * FROM enrollments")
    .fetch_all(pool.get_ref())
    .await {
        Ok(enrollments) => {
            let enrollments:Vec<Enrollment> = enrollments.into_iter().map(|row| Enrollment{
                enrollment_id: row.get("enrollment_id"),
                member_id: row.get("member_id"),
                class_id: row.get("class_id"),
                enrollment_date: row.get("enrollment_date"),
            }).collect();
            HttpResponse::Ok().json(enrollments)
        },
        Err(e) => {
            eprintln!("Failed to fetch enrollments: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch enrollments")
        }
    }
}

#[get("/enrollments/{id}")]
pub async fn get_enrollment_by_id(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let enrollment_id = path.into_inner();
    match sqlx::query("SELECT * FROM enrollments WHERE enrollment_id = ?")
        .bind(enrollment_id)
        .fetch_one(pool.get_ref())
        .await {
            Ok(row) => {
                let enrollment = Enrollment {
                    enrollment_id: row.get("enrollment_id"),
                    member_id: row.get("member_id"),
                    class_id: row.get("class_id"),
                    enrollment_date: row.get("enrollment_date"),
                };
                HttpResponse::Ok().json(enrollment)
            },
            Err(e) => {
                eprintln!("Failed to fetch enrollment: {}", e);
                HttpResponse::InternalServerError().body("Failed to fetch enrollment")
            }
        }
}
#[put("/enrollments/{id}")]
pub async fn update_enrollment(pool: web::Data<SqlitePool>, path: web::Path<i32>, updated_enrollment: web::Json<UpdateEnrollment>) -> impl Responder {
    let enrollment_id = path.into_inner();
    match sqlx::query(
        "UPDATE enrollments SET member_id = ?, class_id = ?, enrollment_date = ? WHERE enrollment_id = ?"
    )
    .bind(updated_enrollment.member_id)
    .bind(updated_enrollment.class_id)
    .bind(&updated_enrollment.enrollment_date)
    .bind(enrollment_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            let enrollment = Enrollment {
                enrollment_id,
                member_id: updated_enrollment.member_id,
                class_id: updated_enrollment.class_id,
                enrollment_date: updated_enrollment.enrollment_date.clone(),

        };
        HttpResponse::Ok().body("Enrollment updated successfully")
    },
        Err(e) => {
            eprintln!("Failed to update enrollment: {}", e);
            HttpResponse::InternalServerError().body("Failed to update enrollment")
        }
    }
}

#[delete("/enrollments/{id}")]
pub async fn delete_enrollment(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let enrollment_id = path.into_inner();
    match sqlx::query("DELETE FROM enrollments WHERE enrollment_id = ?")
        .bind(enrollment_id)
        .execute(pool.get_ref())
        .await {
            Ok(_) => HttpResponse::Ok().body("Enrollment deleted successfully"),
            Err(e) => {
                eprintln!("Failed to delete enrollment: {}", e);
                HttpResponse::InternalServerError().body("Failed to delete enrollment")
            }
        }
}



//return the amount of  enrollments in a specific class(by class_id)
#[get("/enrollments/count/{class_id}")]
pub async fn amount_enrollments_for_class(pool: web::Data<SqlitePool>,path:web::Path<i32>) -> impl Responder {
    let class_id = path.into_inner();
    match sqlx::query("SELECT COUNT(*) as count FROM enrollments WHERE class_id = ?")
        .bind(class_id)
        .fetch_one(pool.get_ref())
        .await {
            Ok(row) => {
                let count: i64 = row.get("count");
                HttpResponse::Ok().json(count)
            },
            Err(e) => {
                eprintln!("Failed to count enrollments: {}", e);
                HttpResponse::InternalServerError().body("Failed to count enrollments")
            }
        }
}