use actix_web::{HttpRequest, HttpResponse};
use lime_domain::healthcheck::status::{Status, StatusType};
use serde::Serialize;
use crate::utils::Context;

pub(super) async fn status(request: HttpRequest) -> HttpResponse {
    let ctx = Context::start(request);
    let status = Status::new(StatusType::Ok);

    ctx.end(
        HttpResponse::Ok().json(Response::from_model(status))
    )
}

#[derive(Serialize)]
struct Response {
    status: String,
}

impl Response {
    fn from_model(model: Status) -> Self {
        Self { status: model.status_type().to_string() }
    }
}