use actix_web::{web, HttpResponse};

pub fn health_checker_handler(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/health")
            .route(web::get().to(|| HttpResponse::Ok().body("Health Check"))),
    );
}
