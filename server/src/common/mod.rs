use ntex::web::HttpResponse;
use serde::Serialize;

use crate::cerror;

pub mod response;

pub fn ok<T>(data: T) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::Ok().json(&response::Response::ok(data))
}

pub fn err<T>(err: cerror::Error) -> HttpResponse
where
    T: Serialize,
{
    HttpResponse::Ok().json(&response::Response::<()>::err(err.0, err.1))
}
