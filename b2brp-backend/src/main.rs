use actix_web::{middleware::Logger, web, App, HttpServer, Scope};
use middleware::auth_middleware::{AuthData, AuthenticationMiddlewareFactory};

mod api;
mod core;
mod db;
mod middleware;
mod schema;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();
    env_logger::init();

    let db_pool = db::create_pool();

    HttpServer::new(move || {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .wrap(Logger::new("%U %s %b"))
            .app_data(web::Data::new(db_pool.clone()))
            .service(api::public_api::scope())
            .service(
                api::private_api::scope().wrap(AuthenticationMiddlewareFactory::new(AuthData {
                    user_id: 0,
                })),
            )
    })
    .bind(("0.0.0.0", 4000))?
    .run()
    .await?;

    Ok(())
}
