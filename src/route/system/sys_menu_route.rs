use crate::handler::system::*;
use crate::AppState;
use axum::routing::post;
use axum::Router;
/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/16 17:46:21
 */
pub fn build_sys_menu_route() -> Router<AppState> {
    Router::new()
        .route("/add_menu", post(sys_menu_handler::add_sys_menu))
        .route("/delete_menu", post(sys_menu_handler::delete_sys_menu))
        .route("/update_menu", post(sys_menu_handler::update_sys_menu))
        .route(
            "/update_menu_status",
            post(sys_menu_handler::update_sys_menu_status),
        )
        .route(
            "/query_menu_detail",
            post(sys_menu_handler::query_sys_menu_detail),
        )
        .route(
            "/query_menu_list",
            post(sys_menu_handler::query_sys_menu_list),
        )
    //记得在main.rs中添加路由build_sys_menu_route()
}
