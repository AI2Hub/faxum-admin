use crate::handler::system::sys_menu_handler;
use crate::AppState;
use axum::routing::post;
use axum::Router;

/*
 *构建菜单信息路由
 *author：刘飞华
 *date：2024/12/12 17:04:49
 */
pub fn build_sys_menu_route() -> Router<AppState> {
    Router::new()
        .route("/system/menu/addMenu", post(sys_menu_handler::add_sys_menu))
        .route(
            "/system/menu/deleteMenu",
            post(sys_menu_handler::delete_sys_menu),
        )
        .route(
            "/system/menu/updateMenu",
            post(sys_menu_handler::update_sys_menu),
        )
        .route(
            "/system/menu/updateMenuStatus",
            post(sys_menu_handler::update_sys_menu_status),
        )
        .route(
            "/system/menu/queryMenuDetail",
            post(sys_menu_handler::query_sys_menu_detail),
        )
        .route(
            "/system/menu/queryMenuList",
            post(sys_menu_handler::query_sys_menu_list),
        )
    //记得在main.rs中添加路由build_sys_menu_route()
}
