use actix_web::{get, HttpResponse};

#[get("/health")]
async fn health() -> HttpResponse {
    tracing::info!("Health check");
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .finish()
}
