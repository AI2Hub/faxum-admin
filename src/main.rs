use std::env;

use crate::middleware::auth::auth;
use crate::route::system::sys_menu_route::build_sys_menu_route;
use crate::route::system::sys_role_route::build_sys_role_route;
use crate::route::system::sys_user_route::build_sys_user_route;
use axum::{middleware as md, Router};
use diesel::r2d2::{self, ConnectionManager};
use diesel::MysqlConnection;
use dotenvy::dotenv;
use once_cell::sync::Lazy;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod schema;
pub mod utils;
pub mod vo;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub static RB: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("database_url").expect("database_url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    let app = Router::new().nest(
        "/api",
        Router::new()
            .merge(build_sys_user_route())
            .merge(build_sys_role_route())
            .merge(build_sys_menu_route())
            .route_layer(md::from_fn(auth)),
    );

    // axum 0.6.x
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service())
    //     .await
    //     .unwrap();

    // axum 0.7.x
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
