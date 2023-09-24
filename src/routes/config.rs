use actix_web::web;

use super::health::get_health;
use super::media::*;
use super::user::{create_user, get_user_by_id, get_users, update_user};

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_user)
        .service(get_users)
        .service(update_user)
        .service(get_user_by_id)
        .service(get_health)
        .service(get_media_by_region);
    cfg.service(scope);
}
