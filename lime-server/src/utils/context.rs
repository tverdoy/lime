use std::time;
use std::time::SystemTime;
use actix_web::{HttpRequest, HttpResponse};
use uuid::{uuid, Uuid};

pub struct Context {
    created: SystemTime,
    logging_id: Uuid
}

impl Context {
    pub fn start(request: HttpRequest) -> Self {
        let _self = Self {created: SystemTime::now(), logging_id: Uuid::now_v7()};
        
        log::info!(target: "api", 
            logging_id = _self.logging_id.to_string(), 
            path = request.path(),
            method = request.method().as_str(); 
            "start");
        
        _self
    }
    
    pub fn end(&self, response: HttpResponse) -> HttpResponse {
        log::info!(target: "api", 
            logging_id = self.logging_id.to_string(), 
            status = response.status().as_str();
            "{:?}", response.body());
        
        response
    }
}