use crate::common::result::BaseResponse;
use crate::model::system::prelude::SysMenu;
use crate::model::system::sys_menu;
use crate::vo::system::sys_menu_vo::*;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use sea_orm::prelude::Expr;
use sea_orm::ActiveValue::Set;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};

/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn add_sys_menu(
    State(state): State<AppState>,
    Json(item): Json<AddMenuReq>,
) -> impl IntoResponse {
    log::info!("add sys_menu params: {:?}", &item);
    let conn = &state.conn;

    let sys_menu = sys_menu::ActiveModel {
        id: NotSet,                     //主键
        menu_name: Set(item.menu_name), //菜单名称
        menu_type: Set(item.menu_type), //菜单类型(1：目录   2：菜单   3：按钮)
        status_id: Set(item.status_id), //状态(1:正常，0:禁用)
        sort: Set(item.sort),           //排序
        parent_id: Set(item.parent_id), //父ID
        menu_url: Set(item.menu_url),   //路由路径
        api_url: Set(item.api_url),     //接口URL
        menu_icon: Set(item.menu_icon), //菜单图标
        remark: Set(item.remark),       //备注
        create_time: NotSet,            //创建时间
        update_time: NotSet,            //修改时间
    };

    let result = SysMenu::insert(sys_menu).exec(conn).await;

    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn delete_sys_menu(
    State(state): State<AppState>,
    Json(item): Json<DeleteMenuReq>,
) -> impl IntoResponse {
    log::info!("delete sys_menu params: {:?}", &item);
    let conn = &state.conn;

    if SysMenu::find_by_id(item.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg("菜单不存在,不能删除!".to_string());
    }

    if SysMenu::find()
        .filter(sys_menu::Column::ParentId.eq(item.id.clone()))
        .count(conn)
        .await
        .unwrap_or_default()
        > 0
    {
        return BaseResponse::<String>::err_result_msg("有下级菜单,不能直接删除!".to_string());
    }

    let result = SysMenu::delete_by_id(item.id.clone()).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn update_sys_menu(
    State(state): State<AppState>,
    Json(item): Json<UpdateMenuReq>,
) -> impl IntoResponse {
    log::info!("update sys_menu params: {:?}", &item);
    let conn = &state.conn;

    if SysMenu::find_by_id(item.id.clone())
        .one(conn)
        .await
        .unwrap_or_default()
        .is_none()
    {
        return BaseResponse::<String>::err_result_msg("菜单信息不存在,不能更新!".to_string());
    }

    let sys_menu = sys_menu::ActiveModel {
        id: Set(item.id),               //主键
        menu_name: Set(item.menu_name), //菜单名称
        menu_type: Set(item.menu_type), //菜单类型(1：目录   2：菜单   3：按钮)
        status_id: Set(item.status_id), //状态(1:正常，0:禁用)
        sort: Set(item.sort),           //排序
        parent_id: Set(item.parent_id), //父ID
        menu_url: Set(item.menu_url),   //路由路径
        api_url: Set(item.api_url),     //接口URL
        menu_icon: Set(item.menu_icon), //菜单图标
        remark: Set(item.remark),       //备注
        create_time: NotSet,            //创建时间
        update_time: NotSet,            //修改时间
    };

    let result = SysMenu::update(sys_menu).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn update_sys_menu_status(
    State(state): State<AppState>,
    Json(item): Json<UpdateMenuStatusReq>,
) -> impl IntoResponse {
    log::info!("update sys_menu_status params: {:?}", &item);
    let conn = &state.conn;

    let result = SysMenu::update_many()
        .col_expr(sys_menu::Column::StatusId, Expr::value(item.status))
        .filter(sys_menu::Column::Id.is_in(item.ids))
        .exec(conn)
        .await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(),
        Err(err) => BaseResponse::<String>::err_result_msg(err.to_string()),
    }
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn query_sys_menu_detail(
    State(state): State<AppState>,
    Json(item): Json<QueryMenuDetailReq>,
) -> impl IntoResponse {
    log::info!("query sys_menu_detail params: {:?}", &item);
    let conn = &state.conn;

    let result = SysMenu::find_by_id(item.id.clone()).one(conn).await;

    match result {
        Ok(d) => {
            let x = d.unwrap();

            let sys_menu = QueryMenuDetailResp {
                id: x.id,                                   //主键
                menu_name: x.menu_name,                     //菜单名称
                menu_type: x.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
                status_id: x.status_id,                     //状态(1:正常，0:禁用)
                sort: x.sort,                               //排序
                parent_id: x.parent_id,                     //父ID
                menu_url: x.menu_url.unwrap_or_default(),   //路由路径
                api_url: x.api_url.unwrap_or_default(),     //接口URL
                menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
                remark: x.remark.unwrap_or_default(),       //备注
                create_time: x.create_time.to_string(),     //创建时间
                update_time: x.update_time.to_string(),     //修改时间
            };

            BaseResponse::<QueryMenuDetailResp>::ok_result_data(sys_menu)
        }
        Err(err) => BaseResponse::<QueryMenuDetailResp>::err_result_data(
            QueryMenuDetailResp::new(),
            err.to_string(),
        ),
    }
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/16 17:59:37
 */
pub async fn query_sys_menu_list(
    State(state): State<AppState>,
    Json(item): Json<QueryMenuListReq>,
) -> impl IntoResponse {
    log::info!("query sys_menu_list params: {:?}", &item);
    let conn = &state.conn;

    let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();

    for x in SysMenu::find()
        .order_by_asc(sys_menu::Column::Sort)
        .all(conn)
        .await
        .unwrap_or_default()
    {
        sys_menu_list_data.push(MenuListDataResp {
            id: x.id,                                   //主键
            menu_name: x.menu_name,                     //菜单名称
            menu_type: x.menu_type,                     //菜单类型(1：目录   2：菜单   3：按钮)
            status_id: x.status_id,                     //状态(1:正常，0:禁用)
            sort: x.sort,                               //排序
            parent_id: x.parent_id,                     //父ID
            menu_url: x.menu_url.unwrap_or_default(),   //路由路径
            api_url: x.api_url.unwrap_or_default(),     //接口URL
            menu_icon: x.menu_icon.unwrap_or_default(), //菜单图标
            remark: x.remark.unwrap_or_default(),       //备注
            create_time: x.create_time.to_string(),     //创建时间
            update_time: x.update_time.to_string(),     //修改时间
        })
    }

    BaseResponse::ok_result_page(sys_menu_list_data, 0)
}
