use actix_web::{get, post,put,delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::member::Member;
use crate::models::member::{CreateMember,UpdateMember};
use crate::controllers::membership_controller;


pub async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    /*let database_url = "sqlite:src/mydb.db";
    let pool = SqlitePool::connect(database_url).await?;*/
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            fname TEXT NOT NULL,
            lname TEXT NOT NULL,
            phone TEXT NOT NULL,
            email TEXT NOT NULL,
            joinDate TEXT NOT NULL,
            membershipId INTEGER NOT NULL,
            FOREIGN KEY (membershipId) REFERENCES memberships(membershipId)
        );
        "#
    ).execute(pool).await?;
    println!("members initialized successfully.");
    Ok(())
}

#[post("/members")]
pub async fn create_member(pool: web::Data<SqlitePool>, new_member: web::Json<CreateMember>) -> impl Responder {
    match  sqlx::query(
        "INSERT INTO members (fname, lname, phone, email, joinDate, membershipId) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(&new_member.fname)
    .bind(&new_member.lname)    
    .bind(&new_member.phone)
    .bind(&new_member.email)
    .bind(&new_member.joinDate)
    .bind(new_member.membershipId)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let inserted_id = result.last_insert_rowid();
            let created_member = Member {
                id: inserted_id as i32,
                fname: new_member.fname.clone(),
                lname: new_member.lname.clone(),
                phone: new_member.phone.clone(),
                email: new_member.email.clone(),
                joinDate: new_member.joinDate.clone(),
                membershipId: new_member.membershipId,
            };
            HttpResponse::Ok().json(created_member)
        },
        Err(e) => {
            eprintln!("Failed to create member: {}", e);
            HttpResponse::InternalServerError().body("Failed to create member")
        }
    }
}

#[get("/members")]
pub async fn get_members(pool: web::Data<SqlitePool>) -> impl Responder{
    match sqlx::query("SELECT * FROM members")
        .fetch_all(pool.get_ref())
        .await
         {
            Ok(members) => {
            let members: Vec<Member> = members.into_iter().map(|row| Member {
                id: row.get("id"),
                fname: row.get("fname"),
                lname: row.get("lname"),
                phone: row.get("phone"),
                email: row.get("email"),
                joinDate: row.get("joinDate"),
                membershipId: row.get("membershipId"),
            }).collect();
            HttpResponse::Ok().json(members)
            },
            Err(e) => {
                eprintln!("Failed to fetch members: {}", e);
                HttpResponse::InternalServerError().body("Failed to fetch members")
            }
        }
}
#[get("/members/{id}")]
pub async fn get_member(pool: web::Data<SqlitePool>, path: web::Path<i64>) -> impl Responder {
    let member_id = path.into_inner();

    match sqlx::query("SELECT * FROM members WHERE id = ?")
        .bind(&member_id)
        .fetch_one(pool.get_ref())
        .await 
    {
        Ok(row) => {
            let member = Member {
                id: row.get("id"),
                fname: row.get("fname"),
                lname: row.get("lname"),
                phone: row.get("phone"),
                email: row.get("email"),
                joinDate: row.get("joinDate"),
                membershipId: row.get("membershipId"),
            };
            HttpResponse::Ok().json(member)
        }
        Err(e) => {
            eprintln!("Failed to fetch member: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch member")
        }
    }
}


#[put("/members/{id}")]
pub async fn update_member(pool:web::Data<SqlitePool>,path:web::Path<i64>,updated_member:web::Json<UpdateMember>)->impl Responder{
    let member_id=path.into_inner();
    match sqlx::query(
        "UPDATE members SET fname = ?, lname = ?, phone = ?, email = ?, joinDate = ?, membershipId = ? WHERE id = ?"
    )
    .bind(&updated_member.fname)
    .bind(&updated_member.lname)    
    .bind(&updated_member.phone)
    .bind(&updated_member.email)
    .bind(&updated_member.joinDate)
    .bind(updated_member.membershipId)
    .bind(member_id)
    .execute(pool.get_ref())
    .await {
        Ok(_) => {
            let member = Member {
                id: member_id as i32,
                fname: updated_member.fname.clone(),
                lname: updated_member.lname.clone(),
                phone: updated_member.phone.clone(),
                email: updated_member.email.clone(),
                joinDate: updated_member.joinDate.clone(),
                membershipId: updated_member.membershipId,
            };
            HttpResponse::Ok().json(member)
        },
        Err(e) => {
            eprintln!("Failed to update member: {}", e);
            HttpResponse::InternalServerError().body("Failed to update member")
        }
    }
}
#[delete("/members/{id}")]
pub async fn delete_member(pool:web::Data<SqlitePool>,path:web::Path<i64>)->impl Responder{
    let member_id=path.into_inner();
    match sqlx::query("DELETE FROM members WHERE id = ?")
        .bind(member_id)
        .execute(pool.get_ref())
        .await {
            Ok(_) => HttpResponse::Ok().body("Member deleted successfully"),
            Err(e) => {
                eprintln!("Failed to delete member: {}", e);
                HttpResponse::InternalServerError().body("Failed to delete member")
            }
        }
}


//return the amount of members who joined in the given year
#[get("/members/joined/{year}")]
pub async fn join_cur_year(pool:web::Data<SqlitePool>,path:web::Path<i64>)->impl Responder{
    let cur_year=path.into_inner();
    match sqlx::query("SELECT COUNT(*) as count FROM members WHERE strftime('%Y', joinDate) = ?")
        .bind(cur_year.to_string())
        .fetch_one(pool.get_ref())
        .await {
            Ok(row) => {
                let count: i64 = row.get("count");
                HttpResponse::Ok().json(count)
            },
            Err(e) => {
                eprintln!("Failed to count members: {}", e);
                HttpResponse::InternalServerError().body("Failed to count members")
            }
        }
}