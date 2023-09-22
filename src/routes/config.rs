use crate::routes::health::health;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(health);
    cfg.service(scope);
}
