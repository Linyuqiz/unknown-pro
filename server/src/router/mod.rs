use crate::{config, service};
use ntex::web::{self, HttpRequest, HttpResponse, ServiceConfig};
use service::user::*;
use tracing::error;

async fn hello(_req: HttpRequest) -> &'static str {
    "Hello World"
}

async fn config_str(_req: HttpRequest) -> HttpResponse {
    config::init_config()
        .map(|config| HttpResponse::Ok().json(&config))
        .map_err(|err| {
            error!("get init config err: {:#?}", err);
        })
        .unwrap_or(HttpResponse::InternalServerError().finish())
}

pub fn register_router(service_config: &mut ServiceConfig) {
    // default handler
    service_config.service((
        web::resource("/hello").route(web::get().to(hello)),
        web::resource("/config_str").route(web::get().to(config_str)),
    ));

    // user handler
    service_config.service(web::scope("/api/v1").service((
        web::resource("/user/login").route(web::get().to(login)),
        web::resource("/user/logout").route(web::get().to(logout)),
        web::resource("/user/list").route(web::get().to(list_user)),
        web::resource("/user/get").route(web::get().to(get_user)),
        web::resource("/user/add").route(web::post().to(add_user)),
        web::resource("/user/update").route(web::post().to(update_user)),
        web::resource("/user/delete").route(web::post().to(delete_user)),
    )));
}
