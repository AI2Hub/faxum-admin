use axum::Json;
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use diesel::associations::HasTable;
use log::{debug, error};
use crate::common::result::BaseResponse;
use crate::model::menu::{SysMenu, AddSysMenu, UpdateSysMenu};
use crate::RB;
use crate::schema::sys_menu::{id, parent_id, sort, status_id};
use crate::schema::sys_menu::dsl::sys_menu;
use crate::vo::system::menu_vo::{*};

// 添加菜单
pub async fn add_menu(Json(req): Json<MenuSaveReq>) -> impl IntoResponse {
    log::info!("add_menu params: {:?}", &req);

    let menu_add = AddSysMenu {
        status_id: req.status_id,
        sort: req.sort,
        parent_id: req.parent_id,
        menu_name: req.menu_name,
        menu_url: req.menu_url,
        api_url: req.api_url,
        menu_icon: req.icon,
        remark: req.remark,
        menu_type: req.menu_type,
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_menu::table()).values(menu_add).execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

// 删除菜单信息
pub async fn delete_menu(Json(req): Json<MenuDeleteReq>) -> impl IntoResponse {
    log::info!("delete_menu params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            match sys_menu.filter(parent_id.eq(&req.id)).count().get_result::<i64>(conn) {
                Ok(count) => {
                    if count > 0 {
                        error!("err:{}", "有下级菜单,不能直接删除".to_string());
                        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除".to_string());
                    }
                    let result = diesel::delete(sys_menu.filter(id.eq(&req.id))).execute(conn);
                    match result {
                        Ok(_u) => BaseResponse::<String>::ok_result(),
                        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                    }
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    BaseResponse::<String>::err_result_msg(err.to_string())
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

// 更新菜单
pub async fn update_menu(Json(req): Json<MenuUpdateReq>) -> impl IntoResponse {
    log::info!("update_menu params: {:?}", &req);

    let update_sys_menu = UpdateSysMenu {
        id: req.id,
        status_id: req.status_id,
        sort: req.sort,
        parent_id: req.parent_id,
        menu_name: req.menu_name,
        menu_url: req.menu_url,
        api_url: req.api_url,
        menu_icon: req.icon,
        remark: req.remark,
        menu_type: req.menu_type,
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_menu).filter(id.eq(&req.id)).set(update_sys_menu).execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(),
                Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}


// 查询菜单
pub async fn query_menu_list(Json(req): Json<MenuListReq>) -> impl IntoResponse {
    log::info!("query_menu_list params: {:?}", &req);
    let mut menu_list: Vec<MenuListData> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut query = sys_menu::table().into_boxed();
            if let Some(i) = &req.status_id {
                query = query.filter(status_id.eq(i));
            }
            query = query.order(sort.asc());
            debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

            if let Ok(menus) = query.load::<SysMenu>(conn) {
                for menu in menus {
                    menu_list.push(MenuListData {
                        id: menu.id,
                        sort: menu.sort,
                        status_id: menu.status_id,
                        parent_id: menu.parent_id,
                        menu_name: menu.menu_name.clone(),
                        label: menu.menu_name,
                        menu_url: menu.menu_url,
                        icon: menu.menu_icon.unwrap_or_default(),
                        api_url: menu.api_url,
                        remark: menu.remark.unwrap_or_default(),
                        menu_type: menu.menu_type,
                        create_time: menu.create_time.to_string(),
                        update_time: menu.update_time.to_string(),
                    })
                }
            }

            BaseResponse::ok_result_page(menu_list, 0)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::err_result_page(menu_list, err.to_string())
        }
    }
}