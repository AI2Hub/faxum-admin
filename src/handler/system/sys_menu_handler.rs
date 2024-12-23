use axum::response::IntoResponse;
use axum::Json;
use diesel::associations::HasTable;
use diesel::sql_types::*;
use diesel::{sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error};

use crate::RB;

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::*;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_menu::*;
use crate::vo::system::sys_menu_vo::*;

/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn add_sys_menu(Json(req): Json<AddMenuReq>) -> impl IntoResponse {
    log::info!("add sys_menu params: {:?}", &req);

    let add_sys_menu_param = AddSysMenu {
        menu_name: req.menu_name,        //菜单名称
        menu_type: req.menu_type,        //菜单类型(1：目录   2：菜单   3：按钮)
        status: req.status,              //状态(1:正常，0:禁用)
        sort: req.sort,                  //排序
        parent_id: req.parent_id,        //父ID
        menu_url: req.menu_url,          //路由路径
        api_url: req.api_url,            //接口URL
        menu_icon: req.menu_icon,        //菜单图标
        remark: req.remark,              //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_menu::table())
                .values(add_sys_menu_param)
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

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn delete_sys_menu(Json(req): Json<DeleteMenuReq>) -> impl IntoResponse {
    log::info!("delete sys_menu params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            match sys_menu
                .filter(parent_id.eq(&req.id))
                .count()
                .get_result::<i64>(conn)
            {
                Ok(count) => {
                    if count > 0 {
                        error!("err:{}", "有下级菜单,不能直接删除".to_string());
                        return BaseResponse::<String>::err_result_msg(
                            "有下级菜单,不能直接删除".to_string(),
                        );
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

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_menu(Json(req): Json<UpdateMenuReq>) -> impl IntoResponse {
    log::info!("update sys_menu params: {:?}", &req);

    let update_sys_menu_param = UpdateSysMenu {
        id: req.id,                      //主键
        menu_name: req.menu_name,        //菜单名称
        menu_type: req.menu_type,        //菜单类型(1：目录   2：菜单   3：按钮)
        status: req.status,              //状态(1:正常，0:禁用)
        sort: req.sort,                  //排序
        parent_id: req.parent_id,        //父ID
        menu_url: req.menu_url,          //路由路径
        api_url: req.api_url,            //接口URL
        menu_icon: req.menu_icon,        //菜单图标
        remark: req.remark,              //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_menu)
                .filter(id.eq(&req.id))
                .set(update_sys_menu_param)
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

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_menu_status(Json(req): Json<UpdateMenuStatusReq>) -> impl IntoResponse {
    log::info!("update sys_menu_status params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_menu)
                .filter(id.eq_any(&req.ids))
                .set(status.eq(req.status))
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

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_menu_detail(
    Json(req): Json<QueryMenuDetailReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query sys_menu_detail params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_menu_sql = sql_query("SELECT * FROM sys_menu WHERE id = ?");
            let result = sys_menu_sql
                .bind::<Bigint, _>(&req.id)
                .get_result::<SysMenu>(conn);
            match result {
                Ok(x) => {
                    let data = QueryMenuDetailResp {
                        id: x.id,                               //主键
                        menu_name: x.menu_name,                 //菜单名称
                        menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                        status: x.status,       //状态(1:正常，0:禁用)
                        sort: x.sort,           //排序
                        parent_id: x.parent_id, //父ID
                        menu_url: x.menu_url,   //路由路径
                        api_url: x.api_url,     //接口URL
                        menu_icon: x.menu_icon, //菜单图标
                        remark: x.remark.unwrap_or_default(), //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    };

                    Ok(BaseResponse::<QueryMenuDetailResp>::ok_result_data(data))
                }
                Err(err) => Err(BaseResponse::<String>::err_result_msg(err.to_string())),
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_menu_list(Json(req): Json<QueryMenuListReq>) -> impl IntoResponse {
    log::info!("query sys_menu_list params: {:?}", &req);
    let query = sys_menu::table().into_boxed();

    //if let Some(i) = &req.status {
    //    query = query.filter(status.eq(i));
    //}

    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = query.load::<SysMenu>(conn);

            if let Ok(role_list) = result {
                for x in role_list {
                    sys_menu_list_data.push(MenuListDataResp {
                        id: x.id,                               //主键
                        menu_name: x.menu_name,                 //菜单名称
                        menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                        status: x.status,       //状态(1:正常，0:禁用)
                        sort: x.sort,           //排序
                        parent_id: x.parent_id, //父ID
                        menu_url: x.menu_url,   //路由路径
                        api_url: x.api_url,     //接口URL
                        menu_icon: x.menu_icon, //菜单图标
                        remark: x.remark.unwrap_or_default(), //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
            }

            BaseResponse::ok_result_page(sys_menu_list_data, 0)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::err_result_page(MenuListDataResp::new(), err.to_string())
        }
    }
}
