use actix_web::{HttpResponse};
use super::super::response::health::{response_health};

pub fn health_index() -> HttpResponse {
    response_health()
}