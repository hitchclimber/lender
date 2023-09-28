use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde_json::json;
// use std::str::FromStr;
use uuid::Uuid;

use crate::models::media::{CreateMedia, Media, UpdateMedia};
// use crate::models::user::{Point, User};
use crate::AppState;

#[get("/media")]
async fn get_all_media(data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query_as!(Media, "SELECT * FROM media")
        .fetch_all(&data.pool)
        .await;
    match result {
        Ok(media) => {
            let collected_media = media.into_iter().map(|m| {
                json!({
                    "id": m.id,
                    "name": &m.name,
                    "creator": &m.creator,
                    "year": m.year,
                    "category": &m.category,
                    "available": m.available
                })
            });
            HttpResponse::Ok()
                .insert_header(CacheControl(vec![CacheDirective::MaxAge(1800)]))
                .json(json!({
                    "status": "success",
                    "media": collected_media.collect::<Vec<_>>()
                }))
        }
        Err(_) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Media not found"
        })),
    }
}

/// Returns all media in a region. Will later on be a radius around a geolocation. We'll let the client do the
/// filtering for availability/media type etc....
#[get("/media/{location}")]
async fn get_media_by_region(
    data: web::Data<AppState>,
    location: web::Path<String>,
) -> impl Responder {
    // NOTE: if below doesn't work, this is a workaround:
    // "SELECT * FROM media WHERE ST_DistanceSphere(location, ST_MakePoint($1, $2)) <= $3",
    //    (WHERE ST_DistanceSphere(users.location, ST_MakePoint($1, $2)) <= $3",)
    // in general, it doesn't matter what we get sent over, we can do all necessary conversions in
    // here, using proper types just makes it more elegant

    let result = sqlx::query_as!(
        Media,
        "SELECT media.*
        FROM media INNER JOIN user_item_relation AS uir ON media.id = uir.item_id
        INNER JOIN users ON uir.user_id = users.id
        WHERE users.location = $1",
        location.into_inner()
    )
    .fetch_all(&data.pool)
    .await;
    match result {
        Ok(media) => HttpResponse::Ok()
            .insert_header(CacheControl(vec![CacheDirective::MaxAge(300)]))
            .json(json!({
                "status": "success",
                "data": json!({"media": media})
            })),
        Err(_) => HttpResponse::NotFound().json(json!({
            "status": "fail",
            "message": "Media not found"
        })),
    }
}

#[post("/media/item")]
async fn create_media_item(
    item: web::Json<CreateMedia>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
    Media,
    "INSERT INTO media (name, creator, year, category, user_id, available) VALUES ($1, $2, $3, $4, $5, $6)",
    item.name,
    item.creator,
    item.year,
    item.category,
    item.user_id,
    item.available.unwrap_or(true)).fetch_one(&data.pool).await;
    match result {
        Ok(_) => HttpResponse::Ok().json(json!({"status": "success"})),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "fail", "message": format!("{:?}", e)})),
    }
}

#[delete("/media/item/{id}")]
async fn delete_media_item(id: web::Path<Uuid>, data: web::Data<AppState>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM media WHERE id = $1", id.into_inner())
        .execute(&data.pool)
        .await;
    match result {
        Ok(_) => {
            HttpResponse::Ok().json(json!({"status": "success", "message": "Deleted media item"}))
        }
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "fail", "message": format!("{:?}", e)})),
    }
}

#[put("/media/item/{id}")]
async fn update_media(
    body: web::Json<UpdateMedia>,
    id: web::Path<Uuid>,
    data: web::Data<AppState>,
) -> impl Responder {
    let result = sqlx::query_as!(
        Media,
        "UPDATE media SET name = COALESCE($1, name), \
        creator = COALESCE($2, creator), \
        year = COALESCE($3, year), \
        category = COALESCE($4, category), \
        available = COALESCE($5, available) \
        WHERE id = $6 RETURNING *",
        body.name,
        body.creator,
        body.year.unwrap_or(0),
        body.category,
        body.available,
        id.into_inner()
    )
    .fetch_one(&data.pool)
    .await;

    match result {
        Ok(media) => HttpResponse::Ok().json(json!({
            "status": "success",
            "data": json!({"media": media})
        })),
        Err(e) => HttpResponse::InternalServerError()
            .json(json!({"status": "fail", "message": format!("{:?}", e)})),
    }
}

// TODO: Searching for media: This can have a longer TTL, since we won't expect new availbale items
// all the time! Also, figure out how to integrate Elasticsearch for fuzzy searching. Until then,
// normalise the media entries
