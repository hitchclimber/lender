use actix_web::http::header::{CacheControl, CacheDirective};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use log::{error, info};
use postgis::ewkb::Point;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::media::{CreateMedia, Media, MediaCategory};
use crate::AppState;

/// Returns all media in a region. Will later on be a radius around a geolocation. We'll let the client do the
/// filtering for availability/media type etc....
#[get("/media/{location}")]
async fn get_media_by_region(
    data: web::Data<AppState>,
    location: web::Path<Point>,
    radius: Option<web::Path<u16>>,
) -> impl Responder {
    info!(
        "Received GET request to /media/{:?}",
        (location.x(), location.y())
    );

    // NOTE: if below doesn't work, this is a workaround:
    // "SELECT * FROM media WHERE ST_DistanceSphere(location, ST_MakePoint($1, $2)) <= $3",
    // in general, it doesn't matter what we get sent over, we can do all necessary conversions in
    // here, using proper types just makes it more elegant

    let result = sqlx::query_as!(
        Media,
        "SELECT media.*
        FROM media INNER JOIN user_item_relation AS uir ON media.id = uir.item_id
        INNER JOIN users ON uir.user_id = users.id
        WHERE ST_DistanceSphere(users.location, $1) <= $2",
        location.into_inner(),
        radius.unwrap_or(10_000)
    )
    .fetch_all(&data.pool)
    .await;
    match result {
        Ok(media) => HttpResponse::Ok()
            .insert_header(CacheControl(vec![CacheDirective::NoCache]))
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
    let item = item.into_inner();
    let category = MediaCategory::from_str(&item.category.to_owned()).unwrap();
    let result = sqlx::query_as!(
    Media,
    "INSERT INTO media (name, creator, year, category, user_id, available) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *",
    item.name,
    item.creator,
    item.year,
    item.category,
        item.user_id,
        item.available.unwrap_or(true)
).fetch_one(&data.pool).await;
    match result {
        Ok(media) => {
            info!("Created media item with id {}", media.id);
            HttpResponse::Ok().json(json!({
                "status": "success",
                "data": json!({"media": media})
            }))
        }
        Err(e) => {
            error!("Encountered error while creating media item: {:?}", e);
            HttpResponse::InternalServerError()
                .json(json!({"status": "fail", "message": format!("{:?}", e)}))
        }
    }
}

// TODO: Searching for media: This can have a longer TTL, since we won't expect new availbale items
// all the time! Also, figure out how to integrate Elasticsearch for fuzzy searching. Until then,
// normalise the media entries
