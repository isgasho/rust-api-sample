use actix_web::{
    HttpResponse,
    web,
};
use super::super::service::develop::{service_developper_index};
use super::super::request::develop::RequestDevelopperIndex;
use super::super::response::develop::{response_developper_index};
use super::super::repository::develop::RepositoryDevelop;

pub fn developper_index(payload: web::Query<RequestDevelopperIndex>) -> HttpResponse {
    let repository_develop = RepositoryDevelop::new();
    let (domain_designs, total) = &service_developper_index(
        repository_develop,
        payload.page,
        payload.page_size,
    );
    response_developper_index(domain_designs, &total)
}