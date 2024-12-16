use std::sync::Arc;
use axum::Router;
use axum::routing::post;
use crate::AppState;
use crate::handler::system::*;
/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/16 17:46:21
 */
pub fn build_sys_menu_route() -> Router<Arc<AppState>> {
    Router::new()
        .route("/add_sys_menu", post(sys_menu_handler::add_sys_menu))
        .route("/delete_sys_menu", post(sys_menu_handler::delete_sys_menu))
        .route("/update_sys_menu", post(sys_menu_handler::update_sys_menu))
        .route("/update_sys_menu_status", post(sys_menu_handler::update_sys_menu_status))
        .route("/query_sys_menu_detail", post(sys_menu_handler::query_sys_menu_detail))
        .route("/query_sys_menu_list", post(sys_menu_handler::query_sys_menu_list))
        //记得在main.rs中添加路由build_sys_menu_route()
}
