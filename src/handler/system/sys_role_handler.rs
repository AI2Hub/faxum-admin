use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryTrait,
};
use sea_orm::prelude::Expr;
use crate::common::result::BaseResponse;
use crate::model::system::prelude::{SysMenu, SysRole, SysRoleMenu, SysUserRole};
use crate::model::system::{sys_role, sys_role_menu, sys_user_role};
use crate::vo::system::sys_role_vo::*;

/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn add_sys_role(
    State(state): State<AppState>,
    Json(item): Json<AddRoleReq>,
) -> impl IntoResponse {
    log::info!("add sys_role params: {:?}", &item);
    let conn = &state.conn;

    let sys_role = sys_role::ActiveModel {
        id: NotSet,                                   //主键
        role_name: Set(item.role_name),               //名称
        status_id: Set(item.status_id),               //状态(1:正常，0:禁用)
        sort: Set(item.sort),                         //排序
        remark: Set(item.remark.unwrap_or_default()), //备注
        create_time: NotSet,                          //创建时间
        update_time: NotSet,                          //修改时间
    };

    let result = SysRole::insert(sys_role).exec(conn).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除角色信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn delete_sys_role(
    State(state): State<AppState>,
    Json(item): Json<DeleteRoleReq>,
) -> impl IntoResponse {
    log::info!("delete sys_role params: {:?}", &item);
    let conn = &state.conn;

    let ids = item.ids.clone();

    if SysUserRole::find()
        .filter(sys_user_role::Column::RoleId.is_in(ids))
        .count(conn)
        .await
        .unwrap_or_default()
        > 0
    {
        return BaseResponse::<String>::err_result_msg("角色已被使用,不能直接删除!".to_string());
    }

    let result = SysRole::delete_many()
        .filter(sys_role::Column::Id.is_in(item.ids))
        .exec(conn)
        .await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn update_sys_role(
    State(state): State<AppState>,
    Json(item): Json<UpdateRoleReq>,
) -> impl IntoResponse {
    log::info!("update sys_role params: {:?}", &item);
    let conn = &state.conn;

    if SysRole::find_by_id(item.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg("角色信息不存在,不能更新!".to_string());
    }

    let sys_role = sys_role::ActiveModel {
        id: Set(item.id),                             //主键
        role_name: Set(item.role_name),               //名称
        status_id: Set(item.status_id),               //状态(1:正常，0:禁用)
        sort: Set(item.sort),                         //排序
        remark: Set(item.remark.unwrap_or_default()), //备注
        create_time: NotSet,                          //创建时间
        update_time: NotSet,                          //修改时间
    };

    let result = SysRole::update(sys_role).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn update_sys_role_status(
    State(state): State<AppState>,
    Json(item): Json<UpdateRoleStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_role_status params: {:?}", &item);
    let conn = &state.conn;

    let result = SysRole::update_many()
       .col_expr(sys_role::Column::StatusId, Expr::value(item.status))
       .filter(sys_role::Column::Id.is_in(item.ids))
       .exec(conn)
       .await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn query_sys_role_detail(
    State(state): State<AppState>,
    Json(item): Json<QueryRoleDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_role_detail params: {:?}", &item);
    let conn = &state.conn;

    let result = SysRole::find_by_id(item.id.clone())
        .one(conn)
        .await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_role = QueryRoleDetailResp {
                id: x.id,                                 //主键
                role_name: x.role_name,                            //名称
                status_id: x.status_id,                            //状态(1:正常，0:禁用)
                sort: x.sort,                                      //排序
                remark: x.remark,                                  //备注
                create_time: x.create_time.to_string(), //创建时间
                update_time: x.update_time.to_string(), //修改时间
            };

            BaseResponse::<QueryRoleDetailResp>::ok_result_data(sys_role)
        }
        Err(err) => BaseResponse::<QueryRoleDetailResp>::err_result_data(
            QueryRoleDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn query_sys_role_list(
    State(state): State<AppState>,
    Json(item): Json<QueryRoleListReq>,
) -> impl IntoResponse {
    log::info!("query sys_role_list params: {:?}", &item);
    let conn = &state.conn;

    let paginator = SysRole::find()
        .apply_if(item.role_name.clone(), |query, v| {
            query.filter(sys_role::Column::RoleName.eq(v))
        })
        .apply_if(item.status_id.clone(), |query, v| {
            query.filter(sys_role::Column::StatusId.eq(v))
        })
        .paginate(conn, item.page_size.clone());

    let total = paginator.num_items().await.unwrap_or_default();

    let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();

    for x in paginator
        .fetch_page(item.page_no.clone() - 1)
        .await
        .unwrap_or_default()
    {
        sys_role_list_data.push(RoleListDataResp {
            id: x.id,                               //主键
            role_name: x.role_name,                 //名称
            status_id: x.status_id,                 //状态(1:正常，0:禁用)
            sort: x.sort,                           //排序
            remark: x.remark,                       //备注
            create_time: x.create_time.to_string(), //创建时间
            update_time: x.update_time.to_string(), //修改时间
        })
    }

    BaseResponse::ok_result_page(sys_role_list_data, total)
}

/*
 *查询角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn query_role_menu(
    state: State<AppState>,
    Json(item): Json<QueryRoleMenuReq>,
) -> impl IntoResponse {
    log::info!("query_role_menu params: {:?}", &item);
    let conn = &state.conn;

    let mut menu_data_list: Vec<MenuList> = Vec::new();
    let mut role_menu_ids: Vec<i64> = Vec::new();

    for x in SysMenu::find().all(conn).await.unwrap_or_default() {
        menu_data_list.push(MenuList {
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
        for x in SysRoleMenu::find()
            .filter(sys_role_menu::Column::RoleId.eq(item.role_id.clone()))
            .all(conn)
            .await
            .unwrap_or_default()
        {
            role_menu_ids.push(x.menu_id);
        }
    }

    BaseResponse::<QueryRoleMenuData>::ok_result_data(QueryRoleMenuData {
        menu_ids: role_menu_ids,
        menu_list: menu_data_list,
    })
}

/*
 *更新角色关联的菜单
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn update_role_menu(
    state: State<AppState>,
    Json(item): Json<UpdateRoleMenuReq>,
) -> impl IntoResponse {
    log::info!("update_role_menu params: {:?}", &item);
    let conn = &state.conn;
    let role_id = item.role_id.clone();

    SysRoleMenu::delete_many()
        .filter(sys_role_menu::Column::RoleId.eq(role_id))
        .exec(conn)
        .await
        .unwrap();
    let mut menu_role: Vec<sys_role_menu::ActiveModel> = Vec::new();

    for id in &item.menu_ids {
        let menu_id = id.clone();
        menu_role.push(sys_role_menu::ActiveModel {
            id: NotSet,
            menu_id: Set(menu_id),
            role_id: Set(role_id.clone()),
            create_time: NotSet, //创建时间
        })
    }
    let result = SysRoleMenu::insert_many(menu_role).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}
