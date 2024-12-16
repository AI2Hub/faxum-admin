use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::handler::system::*;
/*
 *构建用户信息路由
 *author：刘飞华
 *date：2024/12/16 17:46:21
 */
pub fn build_sys_user_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add_sys_user", post(sys_user_handler::add_sys_user))
        .route("/delete_sys_user", post(sys_user_handler::delete_sys_user))
        .route("/update_sys_user", post(sys_user_handler::update_sys_user))
        .route("/update_sys_user_status", post(sys_user_handler::update_sys_user_status))
        .route("/query_sys_user_detail", post(sys_user_handler::query_sys_user_detail))
        .route("/query_sys_user_list", post(sys_user_handler::query_sys_user_list))
        //记得在main.rs中添加路由build_sys_user_route()
}
