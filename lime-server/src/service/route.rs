use actix_web::{web, Scope};
use super::healthcheck;

pub fn get_route() -> Scope {
    register_v1()
}

fn register_v1() -> Scope {
    let mut v1 = web::scope("/v1.0");

    let healthcheck_service = healthcheck::ServiceImpl::new();
    v1 = v1.service(web::scope("/healthcheck").service(healthcheck_service));

    v1
}