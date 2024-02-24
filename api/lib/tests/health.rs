use actix_web::{
    http::StatusCode,
    App,
    test,
};
use api_lib::health::{
    service,
    API_VERSION,
};

#[actix_rt::test]
async fn health_check_works() {
    let app = App::new().configure(service);
    let mut app = test::init_service(app).await;
    let req = test::TestRequest::get()
        .uri("/health")
        .to_request();

    let res = test::call_service(&mut app, req).await;

    assert!(res.status().is_success());
    assert_eq!(res.status(), StatusCode::OK);
    let data = res.headers().get("version").and_then(|h| h.to_str().ok());
    assert_eq!(data, Some(API_VERSION));
}
