use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, web, App, Error, HttpServer};
use api::user_service_config;
use config::Config;
use cors::get_cors_config;
use db::PgPool;

mod api;
mod config;
mod cors;
mod db;
mod models;
mod repository;
mod util;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    let config = Config::from_env().expect("loading server configuration");

    env_logger::init();

    db::init_migrations(&config.database_url)
        .await
        .expect("running database migrations");

    let pool: PgPool = db::new_pg_pool(&config.database_url)
        .await
        .expect("acquiring database connection");

    log::info!("starting server at http://{}:{}", config.host, config.port);

    HttpServer::new(move || {
        let cors = get_cors_config();

        let session_mw = SessionMiddleware::builder(
            CookieSessionStore::default(),
            Key::from(config.signin_key.clone().as_bytes()),
        )
        .cookie_name(config.session_cookie_name.to_string())
        .cookie_secure(false) // for local development
        .session_lifecycle(PersistentSession::default().session_ttl(
            actix_web::cookie::time::Duration::hours(config.session_ttl_hours),
        ))
        .build();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(session_mw)
            .wrap(cors)
            .wrap(Logger::default())
            .service(web::scope("/api").configure(user_service_config))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .workers(2)
    .run()
    .await?;

    Ok(())
}
