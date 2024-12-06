use axum::response::IntoResponse;
use axum::Json;
use diesel::associations::HasTable;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error};

use crate::common::result::BaseResponse;
use crate::common::result_page::ResponsePage;
use crate::model::menu::SysMenu;
use crate::model::role::{AddSysRole, SysRole, UpdateSysRole};
use crate::model::role_menu::AddSysRoleMenu;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_role::dsl::sys_role;
use crate::schema::sys_role::{id, role_name, status_id};
use crate::schema::sys_role_menu::dsl::sys_role_menu;
use crate::schema::sys_role_menu::{menu_id, role_id};
use crate::schema::sys_user_role::dsl::sys_user_role;
use crate::vo::system::role_vo::*;
use crate::{schema, RB};

// 添加角色信息
pub async fn add_role(Json(req): Json<AddRoleReq>) -> impl IntoResponse {
    log::info!("add_role params: {:?}", &req);
    let add_sys_role = AddSysRole {
        status_id: req.status_id,
        sort: req.sort,
        role_name: req.role_name,
        remark: req.remark.unwrap(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_role::table())
                .values(add_sys_role)
                .execute(conn);
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

// 删除角色信息
pub async fn delete_role(Json(req): Json<DeleteRoleReq>) -> impl IntoResponse {
    log::info!("delete_role params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let ids = req.ids.clone();
            //查询角色有没有被使用了,如果使用了就不能删除
            match sys_user_role
                .filter(schema::sys_user_role::role_id.eq_any(ids))
                .count()
                .get_result::<i64>(conn)
            {
                Ok(count) => {
                    if count != 0 {
                        error!("err:{}", "角色已被使用,不能删除".to_string());
                        return BaseResponse::<String>::err_result_msg(
                            "角色已被使用,不能直接删除".to_string(),
                        );
                    }
                    let result = diesel::delete(sys_role.filter(id.eq_any(&req.ids))).execute(conn);
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

// 更新角色信息
pub async fn update_role(Json(req): Json<UpdateRoleReq>) -> impl IntoResponse {
    log::info!("update_role params: {:?}", &req);
    let update_sys_role = UpdateSysRole {
        id: req.id,
        status_id: req.status_id,
        sort: req.sort,
        role_name: req.role_name,
        remark: req.remark.unwrap_or_default(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_role)
                .filter(id.eq(&req.id))
                .set(update_sys_role)
                .execute(conn);
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

// 查询角色列表
pub async fn query_role_list(
    Json(req): Json<QueryRoleListReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query_role_list params: {:?}", &req);
    let mut query = sys_role::table().into_boxed();
    if let Some(i) = &req.role_name {
        query = query.filter(role_name.eq(i));
    }
    if let Some(i) = &req.status_id {
        query = query.filter(status_id.eq(i));
    }

    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = query.load::<SysRole>(conn);
            let mut list: Vec<QueryRoleListData> = Vec::new();
            if let Ok(role_list) = result {
                for role in role_list {
                    list.push(QueryRoleListData {
                        id: role.id,
                        sort: role.sort,
                        status_id: role.status_id,
                        role_name: role.role_name,
                        remark: role.remark,
                        create_time: role.create_time.to_string(),
                        update_time: role.update_time.to_string(),
                    })
                }
            }

            Ok(ResponsePage::ok_result_page(list, 10))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

// 查询角色关联的菜单
pub async fn query_role_menu(
    Json(req): Json<QueryRoleMenuReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query_role_menu params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut menu_data_list: Vec<MenuDataList> = Vec::new();
            let mut role_menu_ids: Vec<i64> = Vec::new();
            // 查询所有菜单
            match sys_menu.load::<SysMenu>(conn) {
                Ok(menu_list) => {
                    for menu in menu_list {
                        menu_data_list.push(MenuDataList {
                            id: menu.id.clone(),
                            parent_id: menu.parent_id,
                            title: menu.menu_name.clone(),
                            key: menu.id.to_string(),
                            label: menu.menu_name,
                            is_penultimate: menu.parent_id == 2,
                        });
                        role_menu_ids.push(menu.id)
                    }
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    return Err(BaseResponse::<String>::err_result_msg(err.to_string()));
                }
            }

            //不是超级管理员的时候,就要查询角色和菜单的关联
            if req.role_id != 1 {
                role_menu_ids.clear();

                match sys_role_menu
                    .filter(role_id.eq(req.role_id.clone()))
                    .select(menu_id)
                    .load::<i64>(conn)
                {
                    Ok(menu_ids) => role_menu_ids = menu_ids,
                    Err(err) => {
                        error!("err:{}", err.to_string());
                        return Err(BaseResponse::<String>::err_result_msg(err.to_string()));
                    }
                }
            }

            Ok(BaseResponse::ok_result_data(QueryRoleMenuData {
                role_menus: role_menu_ids,
                menu_list: menu_data_list,
            }))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

// 更新角色关联的菜单
pub async fn update_role_menu(Json(req): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &req);

    let r_id = req.role_id.clone();
    let menu_ids = req.menu_ids.clone();

    match &mut RB.clone().get() {
        Ok(conn) => match diesel::delete(sys_role_menu.filter(role_id.eq(r_id))).execute(conn) {
            Ok(_) => {
                let mut role_menu: Vec<AddSysRoleMenu> = Vec::new();

                for m_id in menu_ids {
                    role_menu.push(AddSysRoleMenu {
                        status_id: 1,
                        sort: 1,
                        menu_id: m_id.clone(),
                        role_id: r_id.clone(),
                    })
                }

                let result = diesel::insert_into(sys_role_menu::table())
                    .values(&role_menu)
                    .execute(conn);
                match result {
                    Ok(_u) => BaseResponse::<String>::ok_result(),
                    Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
                }
            }
            Err(err) => {
                error!("err:{}", err.to_string());
                BaseResponse::<String>::err_result_msg(err.to_string())
            }
        },
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}
