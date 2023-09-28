use crate::models::user::{CreateUser, UpdateUser, User};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::{error, info};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;

#[get("/users")]
async fn get_users(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&data.pool)
        .await;

    // not using matching because we don't need to unwrap if we get an error
    if result.is_err() {
        error!("Failed to fetch users: {:?}", result.err());
        return HttpResponse::InternalServerError().json(json!({
            "error": "Failed to fetch users",
            "message": "Something bad happened while trying to fetch users"
        }));
    }

    let users = result.unwrap();
    info!("Fetched {} users", users.len());

    HttpResponse::Ok().json(json!({
        "status": "success",
        "no of users": users.len(),
        "data": users
    }))
}

#[get("users/user/{id}")]
async fn get_user_by_id(data: web::Data<AppState>, id: web::Path<Uuid>) -> impl Responder {
    info!("Receieved GET request to /users/user/{}", id);
    let result = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id.into_inner())
        .fetch_one(&data.pool)
        .await;
    match result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": json!({"user": user})
        })),
        Err(_) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "User not found"
        })),
    }
}

#[post("/users/user")]
async fn create_user(body: web::Json<CreateUser>, data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(
        User,
        "INSERT INTO users (first_name, last_name, email, password) VALUES ($1, $2, $3, $4) RETURNING *",
        body.first_name.to_string(),
        body.last_name.to_string(),
        body.email.to_string(),
        body.password.to_string()
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": json!({"user": user})})),
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                error!("Failed to create user: {:?}", &e);
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Duplicate key"}));
            }
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("{:?}", e)}))
        }
    }
}

#[put("/users/user/{id}")]
async fn update_user(
    body: web::Json<UpdateUser>,
    id: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let now = chrono::Utc::now();

    let result = sqlx::query_as!(
        User,
        "UPDATE users SET first_name = COALESCE($1, first_name), \
        last_name = COALESCE($2, last_name), \
        email = COALESCE($3, email), \
        password = COALESCE($4, password), \
        updated_at = $5 \
        WHERE id = $6 RETURNING *",
        body.first_name,
        body.last_name,
        body.email,
        body.password,
        now,
        id.into_inner()
    )
    .fetch_one(&data.pool)
    .await;
    match result {
        Ok(user) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": json!({"user": user})})),
        Err(e) => {
            if e.to_string().contains("duplicate key") {
                error!("Encountered duplicate key");
                return HttpResponse::BadRequest()
                    .json(json!({"status": "fail", "message": "Duplicate key"}));
            }
            HttpResponse::InternalServerError()
                .json(json!({"status": "error", "message": format!("{:?}", e)}))
        }
    }
}

#[delete("/users/user/{id}")]
async fn delete_user(id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = $1", id.into_inner())
        .execute(&data.pool)
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({
            "status": "success",
            "message": "User deleted successfully"
        })),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "error", "message": format!("{:?}", e)})),
    }
}
