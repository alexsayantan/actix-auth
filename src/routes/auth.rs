use actix_web::web;
use crate::handlers;


pub fn auth_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/auth")
            .route(web::get().to(handlers::auth::auth_handler())),
    );
}
