use std::{env};

use axum::{middleware, Router, routing::{get, post}};
use diesel::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use once_cell::sync::Lazy;

use crate::handler::{menu_handler, role_handler, user_handler};
use crate::utils::auth::auth;

pub mod model;
pub mod vo;
pub mod handler;
pub mod utils;
pub mod schema;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub static RB: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("database_url").expect("database_url must be set");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
});

#[tokio::main]
async fn main() {
    dotenv().ok();
    log4rs::init_file("src/config/log4rs.yaml", Default::default()).unwrap();


    let app = Router::new()
        .nest("/api", Router::new()
            .route("/login", post(user_handler::login))
            .route("/query_user_role", post(user_handler::query_user_role))
            .route("/update_user_role", post(user_handler::update_user_role))
            .route("/query_user_menu", get(user_handler::query_user_menu))
            .route("/user_list", post(user_handler::query_user_list))
            .route("/user_save", post(user_handler::add_user))
            .route("/user_delete", post(user_handler::delete_user))
            .route("/user_update", post(user_handler::update_user))
            .route("/update_user_password", post(user_handler::update_user_password))
            .route("/query_role_menu", post(role_handler::query_role_menu))
            .route("/update_role_menu", post(role_handler::update_role_menu))
            .route("/role_list", post(role_handler::query_role_list))
            .route("/role_save", post(role_handler::add_role))
            .route("/role_delete", post(role_handler::delete_role))
            .route("/role_update", post(role_handler::update_role))
            .route("/menu_list", post(menu_handler::query_menu_list))
            .route("/menu_save", post(menu_handler::add_menu))
            .route("/menu_delete", post(menu_handler::delete_menu))
            .route("/menu_update", post(menu_handler::update_menu))
            .route_layer(middleware::from_fn(auth)));


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

