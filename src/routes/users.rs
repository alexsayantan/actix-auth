use actix_web::{web, HttpResponse};

pub fn users_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::get().to(|| HttpResponse::Ok().body("Users Service")))
            .route(web::post().to(|| HttpResponse::Ok().body("Users Service POST"))),
    );
}
