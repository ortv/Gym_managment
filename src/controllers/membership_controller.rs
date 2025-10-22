use actix_web::{get, post,put,delete, web, HttpResponse, Responder};
use sqlx::{SqlitePool, Row};
use crate::models::member::Member;
use crate::models::membership::{Membership,CreateMembership,UpdateMembership};
use crate::controllers::member_controller;



pub async fn init_db (pool: &SqlitePool)-> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS memberships (
            membershipId INTEGER PRIMARY KEY AUTOINCREMENT,
            typeMembership TEXT NOT NULL,
            price REAL NOT NULL,
            durationMonths INTEGER NOT NULL
        );
        "#
    )
    .execute(pool)
    .await?;
    //member_controller::init_db(pool).await?;
    println!("📊 Memberships table ensured.");
    Ok(())
}


#[post("/memberships")]
pub async fn create_membership(pool: web::Data<SqlitePool>, new_membership: web::Json<CreateMembership>) -> impl Responder {
    match  sqlx::query(
        "INSERT INTO memberships (typeMembership, price, durationMonths) VALUES (?, ?, ?)"
    )
    .bind(&new_membership.typeMembership)
    .bind(new_membership.price)    
    .bind(new_membership.durationMonths)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            let inserted_id = result.last_insert_rowid();
            let created_membership = Membership {
                membershipId: inserted_id as i32,
                typeMembership: new_membership.typeMembership.clone(),
                price: new_membership.price,
                durationMonths: new_membership.durationMonths,
            };
            HttpResponse::Ok().json(created_membership)
        },
        Err(e) => {
            eprintln!("Failed to create membership: {}", e);
            HttpResponse::InternalServerError().body("Failed to create membership")
        }
    }
}

#[get("/memberships")]
pub async fn get_memberships(pool: web::Data<SqlitePool>)->impl Responder{
  match sqlx::query(
    "SELECT * FROM memberships"
  )
  .fetch_all(pool.get_ref())
  .await{
    Ok(rows)=>{
        let memberships:Vec<Membership> = rows.iter().map(|row| Membership{
            membershipId: row.get("membershipId"),
            typeMembership: row.get("typeMembership"),
            price: row.get("price"),
            durationMonths: row.get("durationMonths"),
        }).collect();
        HttpResponse::Ok().json(memberships)
    },
    Err(e)=>{
        eprintln!("Failed to fetch memberships: {}", e);
        HttpResponse::InternalServerError().body("Failed to fetch memberships")
    }
}
}

#[get("/memberships/{id}")]
pub async fn get_membership(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let membership_id = path.into_inner();
    match sqlx::query("SELECT * FROM memberships WHERE membershipId = ?")
        .bind(membership_id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(row) => {
            let membership = Membership {
                membershipId: row.get("membershipId"),
                typeMembership: row.get("typeMembership"),
                price: row.get("price"),
                durationMonths: row.get("durationMonths"),
            };
            HttpResponse::Ok().json(membership)
        },
        Err(e) => {
            eprintln!("Failed to fetch membership: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch membership")
        }
    }
}
   

#[put("/memberships/{id}")]
pub async fn update_membership(pool: web::Data<SqlitePool>, path: web::Path<i32>, updated_membership: web::Json<UpdateMembership>) -> impl Responder {
    let membership_id = path.into_inner();
    match sqlx::query(
        "UPDATE memberships SET typeMembership = ?, price = ?, durationMonths = ? WHERE membershipId = ?"
    )
    .bind(&updated_membership.typeMembership)
    .bind(updated_membership.price)
    .bind(updated_membership.durationMonths)
    .bind(membership_id)
    .execute(pool.get_ref())
    .await {
        Ok(result) => {
            if result.rows_affected() == 0 {
                return HttpResponse::NotFound().body("Membership not found");
            }
            let membership = Membership {
                membershipId: membership_id,
                typeMembership: updated_membership.typeMembership.clone(),
                price: updated_membership.price,
                durationMonths: updated_membership.durationMonths,
            };
            HttpResponse::Ok().json(membership)
        },
        Err(e) => {
            eprintln!("Failed to update membership: {}", e);
            HttpResponse::InternalServerError().body("Failed to update membership")
        }
    }
}

#[delete("/memberships/{id}")]
pub async fn delete_membership(pool: web::Data<SqlitePool>, path: web::Path<i32>) -> impl Responder {
    let membership_id = path.into_inner();
    match sqlx::query("DELETE FROM memberships WHERE membershipId = ?")
        .bind(membership_id)
        .execute(pool.get_ref())
        .await {
            Ok(result) => {
                if result.rows_affected() == 0 {
                    return HttpResponse::NotFound().body("Membership not found");
                }
                HttpResponse::Ok().body("Membership deleted successfully")
            },
            Err(e) => {
                eprintln!("Failed to delete membership: {}", e);
                HttpResponse::InternalServerError().body("Failed to delete membership")
            }
        }
}


//return price*durationMonths for member_id
#[get("/memberships/payment/{id}")]
pub async fn actual_payment(pool:web::Data<SqlitePool>,path: web::Path<i32>)->impl Responder{
    let member_id=path.into_inner();
    match sqlx::query(
        "SELECT m.price, m.durationMonths FROM memberships m
        JOIN members mb ON m.membershipId = mb.membershipId
        WHERE mb.id = ?"
    )
    .bind(member_id)
    .fetch_one(pool.get_ref())  
    .await {
        Ok(row) => {
            let price: f64 = row.get("price");
            let duration_months: i32 = row.get("durationMonths");
            let total_payment = price * duration_months as f64;
            HttpResponse::Ok().json(total_payment)
        },
        Err(e) => {
            eprintln!("Failed to calculate actual payment: {}", e);
            HttpResponse::InternalServerError().body("Failed to calculate actual payment")
        }
    }

}
