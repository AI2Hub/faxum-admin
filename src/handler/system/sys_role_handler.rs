 use axum::Json;
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, sql_query};
use diesel::associations::HasTable;
use diesel::sql_types::*;
use log::{debug, error};

use crate::{RB, schema};

use crate::common::result::BaseResponse;
use crate::model::system::sys_role_model::*;
use crate::schema::sys_role::*;
use crate::schema::sys_role::dsl::sys_role;
use crate::vo::system::*;
use crate::vo::system::sys_role_vo::*;


/*
 *添加角色信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn add_sys_role(Json(req): Json<AddRoleReq>) -> impl IntoResponse {
    log::info!("add sys_role params: {:?}", &req);

    let add_sys_role_param = AddSysRole {, //主键
        role_name: req.role_name, //名称
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort, //排序
        remark: req.remark, //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_role::table()).values(add_sys_role_param).execute(conn);
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
 *删除角色信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn delete_sys_role(Json(req): Json<DeleteRoleReq>) -> impl IntoResponse {
    log::info!("delete sys_role params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::delete(sys_role).filter(id.eq_any(&req.ids)).execute(conn);
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
 *更新角色信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_role(Json(req): Json<UpdateRoleReq>) -> impl IntoResponse {
    log::info!("update sys_role params: {:?}", &req);

    let update_sys_role_param = UpdateSysRole {
        id: req.id, //主键
        role_name: req.role_name, //名称
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort, //排序
        remark: req.remark, //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间

    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_role).filter(id.eq(&req.id)).set(update_sys_role_param).execute(conn);
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
 *更新角色信息状态
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_role_status(Json(req): Json<UpdateRoleStatusReq>) -> impl IntoResponse {
    log::info!("update sys_role_status params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_role).filter(id.eq_any(&req.ids)).set(status.eq(req.status)).execute(conn);
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
 *查询角色信息详情
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_role_detail(Json(req): Json<QueryRoleDetailReq>) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query sys_role_detail params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_role_sql = sql_query("SELECT * FROM sys_role WHERE id = ?");
            let result = sys_role_sql.bind::<Bigint, _>(&req.id).get_result::<SysRole>(conn);
            match result {
                Ok(x) => {
                let data  =QueryRoleDetailResp {
                    id: x.id, //主键
                    role_name: x.role_name, //名称
                    status_id: x.status_id, //状态(1:正常，0:禁用)
                    sort: x.sort, //排序
                    remark: x.remark, //备注
                    create_time: x.create_time.to_string(), //创建时间
                    update_time: x.update_time.to_string(), //修改时间
                  };

                 Ok(BaseResponse::<QueryRoleDetailResp>::ok_result_data(data))
                 },
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
 *查询角色信息列表
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_role_list(Json(req): Json<QueryRoleListReq>) -> impl IntoResponse {
    log::info!("query sys_role_list params: {:?}", &req);
    let mut query = sys_role::table().into_boxed();

    //if let Some(i) = &req.status {
    //    query = query.filter(status_id.eq(i));
    //}

    debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

    let mut sys_role_list_data: Vec<RoleListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = query.load::<SysRole>(conn);

            if let Ok(role_list) = result {
                for x in role_list {
                    sys_role_list_data.push(RoleListDataResp {
                        id: x.id, //主键
                        role_name: x.role_name, //名称
                        status_id: x.status_id, //状态(1:正常，0:禁用)
                        sort: x.sort, //排序
                        remark: x.remark, //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
            }

            BaseResponse::ok_result_page(sys_role_list_data, 0)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::err_result_page(RoleListDataResp::new(), err.to_string())
        }
    }
}

