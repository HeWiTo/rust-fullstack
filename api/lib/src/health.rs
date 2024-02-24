use actix_web::{
    web::{
        self,
        ServiceConfig
    },
    HttpResponse
};

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    tracing::info!("Health check");
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
