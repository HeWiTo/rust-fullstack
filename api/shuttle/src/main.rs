use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::Executor;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;

    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = web::Data::new(film_repository);
    
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
        .service(actix_files::Files::new("/", "static".to_string())
            .show_files_listing()
            .index_file("index.html"),
        );
    };

    Ok(config.into())
}
