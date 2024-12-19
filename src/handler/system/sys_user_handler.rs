 use axum::Json;
use axum::response::IntoResponse;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, sql_query};
use diesel::associations::HasTable;
use diesel::sql_types::*;
use log::{debug, error};

use crate::{RB, schema};

use crate::common::result::BaseResponse;
use crate::model::system::sys_user_model::*;
use crate::schema::sys_user::*;
use crate::schema::sys_user::dsl::sys_user;
use crate::vo::system::*;
use crate::vo::system::sys_user_vo::*;


/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn add_sys_user(Json(req): Json<AddUserReq>) -> impl IntoResponse {
    log::info!("add sys_user params: {:?}", &req);

    let add_sys_user_param = AddSysUser {, //主键
        mobile: req.mobile, //手机
        user_name: req.user_name, //姓名
        password: req.password, //密码
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort, //排序
        remark: req.remark, //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_user::table()).values(add_sys_user_param).execute(conn);
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
 *删除用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn delete_sys_user(Json(req): Json<DeleteUserReq>) -> impl IntoResponse {
    log::info!("delete sys_user params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::delete(sys_user).filter(id.eq_any(&req.ids)).execute(conn);
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
 *更新用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_user(Json(req): Json<UpdateUserReq>) -> impl IntoResponse {
    log::info!("update sys_user params: {:?}", &req);

    let update_sys_user_param = UpdateSysUser {
        id: req.id, //主键
        mobile: req.mobile, //手机
        user_name: req.user_name, //姓名
        password: req.password, //密码
        status_id: req.status_id, //状态(1:正常，0:禁用)
        sort: req.sort, //排序
        remark: req.remark, //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间

    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_user).filter(id.eq(&req.id)).set(update_sys_user_param).execute(conn);
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
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_user_status(Json(req): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    log::info!("update sys_user_status params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_user).filter(id.eq_any(&req.ids)).set(status.eq(req.status)).execute(conn);
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
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_user_detail(Json(req): Json<QueryUserDetailReq>) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query sys_user_detail params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_user_sql = sql_query("SELECT * FROM sys_user WHERE id = ?");
            let result = sys_user_sql.bind::<Bigint, _>(&req.id).get_result::<SysUser>(conn);
            match result {
                Ok(x) => {
                let data  =QueryUserDetailResp {
                    id: x.id, //主键
                    mobile: x.mobile, //手机
                    user_name: x.user_name, //姓名
                    password: x.password.unwrap_or_default(), //密码
                    status_id: x.status_id, //状态(1:正常，0:禁用)
                    sort: x.sort, //排序
                    remark: x.remark.unwrap_or_default(), //备注
                    create_time: x.create_time.to_string(), //创建时间
                    update_time: x.update_time.to_string(), //修改时间
                  };

                 Ok(BaseResponse::<QueryUserDetailResp>::ok_result_data(data))
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
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_user_list(Json(req): Json<QueryUserListReq>) -> impl IntoResponse {
    log::info!("query sys_user_list params: {:?}", &req);
    let mut query = sys_user::table().into_boxed();

    //if let Some(i) = &req.status {
    //    query = query.filter(status_id.eq(i));
    //}

    debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string());

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = query.load::<SysUser>(conn);

            if let Ok(role_list) = result {
                for x in role_list {
                    sys_user_list_data.push(UserListDataResp {
                        id: x.id, //主键
                        mobile: x.mobile, //手机
                        user_name: x.user_name, //姓名
                        password: x.password.unwrap_or_default(), //密码
                        status_id: x.status_id, //状态(1:正常，0:禁用)
                        sort: x.sort, //排序
                        remark: x.remark.unwrap_or_default(), //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
            }

            BaseResponse::ok_result_page(sys_user_list_data, 0)
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::err_result_page(UserListDataResp::new(), err.to_string())
        }
    }
}

