use actix_web::HttpResponse;
use super::super::domain::develop::DomainDevelopper;
use serde_derive::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Developper {
    id: i32,
    title: String,
    user_id: i32,
    user_name: String,
    user_image: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DevelopperIndex {
    total: i32,
    developpers: Vec<Developper>,
}

pub fn response_developper_index(domain_developpers: &Vec<DomainDevelopper>, total: &i32) -> HttpResponse {
    let mut developpers = Vec::new();
    for domain_developper in domain_developpers {
        developpers.push(Developper {
            id: *domain_developper.id(),
            title: domain_developper.title().to_string(),
            user_id: *domain_developper.user_id(),
            user_name: domain_developper.user_name().to_string(),
            user_image: domain_developper.user_image().to_string(),
        });
    }
    let response_developpers = DevelopperIndex {
        total: *total,
        developpers,
    };

    HttpResponse::Ok().json(response_developpers)
}