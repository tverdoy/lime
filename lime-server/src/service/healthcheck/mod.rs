use actix_web::{web, HttpRequest, HttpResponse};
use actix_web::dev::{AppService, HttpServiceFactory};

mod status;

pub trait Service {
    async fn status(request: HttpRequest) -> HttpResponse;
}

#[derive(Clone)]
pub(super) struct ServiceImpl {}

impl ServiceImpl {
    pub(super) fn new() -> Self {
        Self {}
    }
}

impl HttpServiceFactory for ServiceImpl {
    fn register(self, config: &mut AppService) {
        web::resource("/status").get(Self::status).register(config);
    }
}

impl Service for ServiceImpl {
    async fn status(request: HttpRequest) -> HttpResponse {
        status::status(request).await
    }
}
