use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::common::result::BaseResponse;
use crate::common::result_page::ResponsePage;
use crate::model::{sys_role, sys_role_menu, sys_user_role};
use crate::model::prelude::{SysMenu, SysRole, SysRoleMenu, SysUserRole};
use crate::vo::system::role_vo::*;

// 查询角色列表
pub async fn role_list(state: State<AppState>, Json(item): Json<RoleListReq>) -> impl IntoResponse {
    log::info!("role_list params: {:?}", &item);
    let conn = &state.conn;

    let paginator = SysRole::find()
        .apply_if(item.role_name.clone(), |query, v| {
            query.filter(sys_role::Column::RoleName.eq(v))
        })
        .apply_if(item.status_id.clone(), |query, v| {
            query.filter(sys_role::Column::StatusId.eq(v))
        }).paginate(conn, item.page_size.clone());

    let total = paginator.num_items().await.unwrap_or_default();


    let mut role_list: Vec<RoleListData> = Vec::new();

    for role in paginator.fetch_page(item.page_no.clone() - 1).await.unwrap_or_default() {
        role_list.push(RoleListData {
            id: role.id,
            sort: role.sort,
            status_id: role.status_id,
            role_name: role.role_name,
            remark: role.remark,
            create_time: role.create_time.to_string(),
            update_time: role.update_time.to_string(),
        })
    }

    ResponsePage::<Vec<RoleListData>>::ok_result_page(role_list, total)
}

// 添加角色信息
pub async fn role_save(state: State<AppState>, Json(role): Json<RoleSaveReq>) -> impl IntoResponse {
    log::info!("role_save params: {:?}", &role);
    let conn = &state.conn;


    let sys_role = sys_role::ActiveModel {
        id: NotSet,
        status_id: Set(role.status_id),
        sort: Set(role.sort),
        role_name: Set(role.role_name),
        remark: Set(role.remark.unwrap_or_default()),
        ..Default::default()
    };

    let result=SysRole::insert(sys_role).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 更新角色信息
pub async fn role_update(state: State<AppState>, Json(role): Json<RoleUpdateReq>) -> impl IntoResponse {
    log::info!("role_update params: {:?}", &role);
    let conn = &state.conn;


    if SysRole::find_by_id(role.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return BaseResponse::<String>::err_result_msg("角色不存在,不能更新!".to_string());
    }
    let sys_role = sys_role::ActiveModel {
        id: Set(role.id),
        status_id: Set(role.status_id),
        sort: Set(role.sort),
        role_name: Set(role.role_name),
        remark: Set(role.remark.unwrap_or_default()),
        ..Default::default()
    };

    let result=SysRole::update(sys_role).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 删除角色信息
pub async fn role_delete(state: State<AppState>, Json(item): Json<RoleDeleteReq>) -> impl IntoResponse {
    log::info!("role_delete params: {:?}", &item);
    let conn = &state.conn;

    let ids = item.ids.clone();

    if SysUserRole::find().filter(sys_user_role::Column::RoleId.is_in(ids)).count(conn).await.unwrap_or_default() > 0 {
        return BaseResponse::<String>::err_result_msg("角色已被使用,不能直接删除!".to_string());
    }

    let result=SysRole::delete_many().filter(sys_role::Column::Id.is_in(item.ids.clone())).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

// 查询角色关联的菜单
pub async fn query_role_menu(state: State<AppState>, Json(item): Json<QueryRoleMenuReq>) -> impl IntoResponse {
    log::info!("query_role_menu params: {:?}", &item);
    let conn = &state.conn;

    let mut menu_data_list: Vec<MenuDataList> = Vec::new();
    let mut role_menu_ids: Vec<i64> = Vec::new();

    for x in SysMenu::find().all(conn).await.unwrap_or_default() {
        menu_data_list.push(MenuDataList {
            id: x.id.clone(),
            parent_id: x.parent_id,
            title: x.menu_name.clone(),
            key: x.id.to_string(),
            label: x.menu_name,
            is_penultimate: x.parent_id == 2,
        });
        role_menu_ids.push(x.id)
    }

    //不是超级管理员的时候,就要查询角色和菜单的关联
    if item.role_id != 1 {
        role_menu_ids.clear();
        for x in SysRoleMenu::find().filter(sys_role_menu::Column::RoleId.eq(item.role_id.clone())).all(conn).await.unwrap_or_default() {
            role_menu_ids.push(x.menu_id);
        }
    }

    BaseResponse::<QueryRoleMenuData>::ok_result_data(QueryRoleMenuData { role_menus: role_menu_ids, menu_list: menu_data_list })
}

// 更新角色关联的菜单
pub async fn update_role_menu(state: State<AppState>, Json(item): Json<UpdateRoleMenuReq>) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let conn = &state.conn;
    let role_id = item.role_id.clone();

    SysRoleMenu::delete_many().filter(sys_role_menu::Column::RoleId.eq(role_id)).exec(conn).await.unwrap();
    let mut menu_role: Vec<sys_role_menu::ActiveModel> = Vec::new();

    for id in &item.menu_ids {
        let menu_id = id.clone();
        menu_role.push(sys_role_menu::ActiveModel {
            id: NotSet,
            status_id: Set(1),
            sort: Set(1),
            menu_id: Set(menu_id),
            role_id: Set(role_id.clone()),
            ..Default::default()
        })
    }
    let result=SysRoleMenu::insert_many(menu_role).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}
