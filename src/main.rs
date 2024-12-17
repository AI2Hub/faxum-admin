use crate::route::system::sys_menu_route::build_sys_menu_route;
use crate::route::system::sys_role_route::build_sys_role_route;
use crate::route::system::sys_user_route::build_sys_user_route;
use axum::{middleware as md, Router};
use middleware::auth::auth;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub mod common;
pub mod handler;
pub mod middleware;
pub mod model;
pub mod route;
pub mod utils;
pub mod vo;

#[derive(Clone)]
pub struct AppState {
    pub conn: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let shared_state = AppState { conn };
    let app = Router::new().nest(
        "/api",
        Router::new()
            .merge(build_sys_user_route())
            .merge(build_sys_role_route())
            .merge(build_sys_menu_route())
            .route_layer(md::from_fn(auth))
            .route_layer(md::from_fn(auth))
            .with_state(shared_state),
    );

    //axum 0.6.x
    // let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // log::info!("listening on {}", addr);
    // axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    //axum 0.7.x
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
