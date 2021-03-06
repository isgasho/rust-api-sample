extern crate gcs_api;
extern crate json;
extern crate serde_json;

// extern crate diesel;
// extern crate r2d2;
// extern crate r2d2_diesel;

// use diesel::prelude::*;

use dotenv;
use actix_cors::Cors;
// use r2d2::Pool;
// use diesel::MysqlConnection;
// use r2d2_diesel::ConnectionManager;
// use self::r2d2::ManageConnection;

use actix_web::{
    http,
    middleware,
    web,
    App,
    HttpServer,
};
use gcs_api::controller::{
    health,
    design,
    pickup,
    develop,
    designer,
    developper,
    sample,
    user,
    job,
};

fn setup() {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
}

fn main() -> std::io::Result<()> {
    setup();
    let url = dotenv::var("HOST").unwrap().to_string() + ":" + &dotenv::var("PORT").unwrap();

    // let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URLが設定されていない。");
    // let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    // let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST", "OPTIONS", "HEAD", "DELETE", "PUT"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT, http::header::CONTENT_TYPE])
                    .max_age(3600),
            )
            // コネクションプール
            // .data(pool.clone())
            // ログ有効
            .wrap(middleware::Logger::default())
            // 制限
            .data(web::JsonConfig::default().limit(4096))
            // ルーティング
            .service(
                web::resource("/health")
                    .route(web::get()
                        .to_async(health::index)),
            )
            .service(
                web::resource("/sample")
                    .route(web::get()
                        .to_async(sample::index)),
            )
            .service(
                web::resource("/designs")
                    .route(web::get()
                        .to_async(design::index)),
            )
            .service(
                web::resource("/pickups")
                    .route(web::get()
                        .to_async(pickup::index)),
            )
            .service(
                web::resource("/users")
                    .route(web::get()
                        .to_async(user::index)),
            )
            .service(
                web::resource("/user")
                    .route(web::get()
                        .to_async(user::show)),
            )
            .service(
                web::resource("/designer")
                    .route(web::get()
                        .to_async(designer::index)),
            )
            .service(
                web::resource("/developper")
                    .route(web::get()
                        .to_async(developper::index)),
            )
            .service(
                web::resource("/develops")
                    .route(web::get()
                        .to_async(develop::index)),
            )
            .service(
                web::resource("/jobs")
                    .route(web::get()
                        .to_async(job::index)),
            )
            .service(
                web::resource("/job")
                    .route(web::get()
                        .to_async(job::show)),
            )
    })
        .bind(url)?
        .run()
}