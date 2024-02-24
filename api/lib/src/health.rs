use actix_web::{
    web::{
        self,
        ServiceConfig
    },
    HttpResponse
};

pub const API_VERSION: &str = "v0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    tracing::info!("Health check");
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

/*
 * The #[cfg(test)] annotation tells the compiler 
 * to only compile the code in this module 
 * if we are running tests.
 * */
#[cfg(test)]
mod tests {
    use actix_web::http::StatusCode;
    
    /*
     * The use super::*; statement imports all the items 
     * from the parent module even if they're not public 
     * (in this case, the health function).
     * */
    use super::*;

    /*
     * The #[actix_rt::test] annotation tells the compiler 
     * to run this test in the Actix runtime 
     * (giving you async support).
     * */
    #[actix_rt::test]
    async fn health_check_works() {
        let res = health().await;

        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);

        let data = res
            .headers()
            .get("version")
            .and_then(|h| h.to_str().ok());

        assert_eq!(data, Some(API_VERSION));
    }
}
