use migration::{Migrator, MigratorTrait};
use ntex::web::{self, middleware::Logger, App, HttpResponse};
use server::{config, consts, router, utils};
use std::sync::Arc;
use tracing::info;

#[ntex::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();

    let app_config = config::init_config().expect("config init failed");
    let default_mysql = app_config
        .mysql
        .get(consts::DEFAULT_MYSQL)
        .expect("default mysql config not found");
    let mysql_database = utils::init_database(&default_mysql.dsn)
        .await
        .expect("database init failed");
    // database ping
    if mysql_database.ping().await.is_ok() {
        info!("database is ready");
    }
    // database migration
    Migrator::up(&mysql_database, None)
        .await
        .expect("database migration failed");

    web::server(move || {
        App::new()
            .state(Arc::new(mysql_database.clone()))
            .wrap(Logger::default())
            .configure(router::register_router)
            .default_service(web::to(|| async { HttpResponse::NotFound() }))
    })
    .bind(format!("{}:{}", app_config.host, app_config.port))?
    .run()
    .await
}
