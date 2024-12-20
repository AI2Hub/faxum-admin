use axum::http::HeaderMap;
use axum::response::IntoResponse;
use axum::Json;
use diesel::associations::HasTable;
use diesel::sql_types::*;
use diesel::{sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, info, warn};
use std::collections::HashSet;

use crate::common::error::WhoUnfollowedError;
use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::{StringColumn, SysMenu};
use crate::model::system::sys_role_model::SysRole;
use crate::model::system::sys_user_model::*;
use crate::model::system::sys_user_role_model::{AddSysUserRole, SysUserRole};
use crate::schema::sys_menu::api_url;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_role::dsl::sys_role;
use crate::schema::sys_user::dsl::sys_user;
use crate::schema::sys_user::*;
use crate::schema::sys_user_role::dsl::sys_user_role;
use crate::schema::sys_user_role::{role_id, user_id};
use crate::utils::jwt_util::JWTToken;
use crate::vo::system::sys_user_vo::*;
use crate::{schema, RB};

/*
 *添加用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn add_sys_user(Json(req): Json<AddUserReq>) -> impl IntoResponse {
    log::info!("add sys_user params: {:?}", &req);

    let add_sys_user_param = AddSysUser {
        mobile: req.mobile,              //手机
        user_name: req.user_name,        //姓名
        password: req.password,          //密码
        status_id: req.status_id,        //状态(1:正常，0:禁用)
        sort: req.sort,                  //排序
        remark: req.remark,              //备注
        create_time: Default::default(), //创建时间
        update_time: Default::default(), //修改时间
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_user::table())
                .values(add_sys_user_param)
                .execute(conn);
            // debug!("SQL:{}", diesel::debug_query::<diesel::mysql::Mysql, _>(&result).to_string());
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
            let ids = req.ids.clone();
            //id为1的用户为系统预留用户,不能删除
            let mut delete_ids = vec![];
            for delete_id in ids {
                if delete_id == 1 {
                    warn!("err:{}", "不能删除超级管理员".to_string());
                    continue;
                }
                delete_ids.push(delete_id)
            }

            if delete_ids.len() == 0 {
                return BaseResponse::<String>::ok_result();
            }

            let query = diesel::delete(sys_user.filter(id.eq_any(delete_ids)));
            debug!(
                "SQL: {}",
                diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
            );
            let result = query.execute(conn);
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

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");

            match user_sql
                .bind::<Bigint, _>(req.id)
                .get_result::<SysUser>(conn)
            {
                Ok(s_user) => {
                    let update_sys_user = UpdateSysUser {
                        id: req.id.clone(),
                        status_id: req.status_id,
                        sort: req.sort,
                        mobile: req.mobile,
                        user_name: req.user_name,
                        remark: req.remark,
                        create_time: s_user.create_time,
                        update_time: Default::default(),
                    };

                    let query =
                        diesel::update(sys_user.filter(id.eq(req.id.clone()))).set(update_sys_user);
                    debug!(
                        "SQL:{}",
                        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
                    );
                    let result = query.execute(conn);
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
 *更新用户信息状态
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_sys_user_status(Json(req): Json<UpdateUserStatusReq>) -> impl IntoResponse {
    log::info!("update sys_user_status params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_user)
                .filter(id.eq_any(&req.ids))
                .set(status_id.eq(req.status))
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
 *查询用户信息详情
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_user_detail(
    Json(req): Json<QueryUserDetailReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    log::info!("query sys_user_detail params: {:?}", &req);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_user_sql = sql_query("SELECT * FROM sys_user WHERE id = ?");
            let result = sys_user_sql
                .bind::<Bigint, _>(&req.id)
                .get_result::<SysUser>(conn);
            match result {
                Ok(x) => {
                    let data = QueryUserDetailResp {
                        id: x.id,                               //主键
                        mobile: x.mobile,                       //手机
                        user_name: x.user_name,                 //姓名
                        status_id: x.status_id,                 //状态(1:正常，0:禁用)
                        sort: x.sort,                           //排序
                        remark: x.remark.unwrap_or_default(),   //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    };

                    Ok(BaseResponse::<QueryUserDetailResp>::ok_result_data(data))
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
 *查询用户信息列表
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_sys_user_list(Json(req): Json<QueryUserListReq>) -> impl IntoResponse {
    log::info!("query sys_user_list params: {:?}", &req);
    let mut query = sys_user::table().into_boxed();

    if let Some(i) = &req.status_id {
        query = query.filter(status_id.eq(i));
    }
    if let Some(i) = &req.mobile {
        query = query.filter(mobile.eq(i));
    }
    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    let mut sys_user_list_data: Vec<UserListDataResp> = Vec::new();
    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = query.load::<SysUser>(conn);

            if let Ok(role_list) = result {
                for x in role_list {
                    sys_user_list_data.push(UserListDataResp {
                        id: x.id,                               //主键
                        mobile: x.mobile,                       //手机
                        user_name: x.user_name,                 //姓名
                        status_id: x.status_id,                 //状态(1:正常，0:禁用)
                        sort: x.sort,                           //排序
                        remark: x.remark.unwrap_or_default(),   //备注
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

/*
 *后台用户登录
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn login(Json(req): Json<UserLoginReq>) -> impl IntoResponse {
    info!("user login params: {:?}", &req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let query = sys_user.filter(mobile.eq(&req.mobile));
            debug!(
                "SQL: {}",
                diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
            );

            if let Ok(user) = query.first::<SysUser>(conn) {
                info!("select_by_mobile: {:?}", user);

                if user.password.ne(&req.password) {
                    return BaseResponse::<String>::err_result_msg("密码不正确".to_string());
                }

                let btn_menu = query_btn_menu(user.id);

                if btn_menu.len() == 0 {
                    return BaseResponse::<String>::err_result_msg(
                        "用户没有分配角色或者菜单,不能登录".to_string(),
                    );
                }

                match JWTToken::new(user.id, &user.user_name, btn_menu).create_token("123") {
                    Ok(token) => BaseResponse::<String>::ok_result_data(token),
                    Err(err) => {
                        let er = match err {
                            WhoUnfollowedError::JwtTokenError(s) => s,
                            _ => "no math error".to_string(),
                        };

                        error!("err:{}", er.to_string());
                        BaseResponse::<String>::err_result_msg(er)
                    }
                }
            } else {
                error!("err:{}", "根据手机号查询用户异常".to_string());
                BaseResponse::<String>::err_result_msg("根据手机号查询用户异常".to_string())
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}

/*
 *查询用户按钮权限
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
fn query_btn_menu(u_id: i64) -> Vec<String> {
    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_role_sql =
                sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
            match user_role_sql
                .bind::<Bigint, _>(&u_id)
                .get_result::<SysUserRole>(conn)
            {
                Ok(_) => {
                    let sys_menu_result = sys_menu.select(api_url).load::<String>(conn);
                    sys_menu_result.unwrap_or_else(|_| Vec::new())
                }
                Err(_) => {
                    let result = sql_query(
                        "select u.api_url from sys_user_role t \
                    left join sys_role usr on t.role_id = usr.id \
                    left join sys_role_menu srm on usr.id = srm.role_id \
                    left join sys_menu u on srm.menu_id = u.id \
                    where t.user_id = ?",
                    )
                    .bind::<Bigint, _>(&u_id)
                    .load::<StringColumn>(conn);

                    match result {
                        Ok(btn_list) => {
                            let mut btn_list_data: Vec<String> = Vec::new();
                            for x in btn_list {
                                if x.api_url.clone().len() != 0 {
                                    btn_list_data.push(x.api_url);
                                }
                            }
                            btn_list_data
                        }
                        Err(_) => Vec::new(),
                    }
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Vec::new()
        }
    }
}

/*
 *查询用户角色
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_user_role(
    Json(req): Json<QueryUserRoleReq>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("query_user_role params: {:?}", req);
    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut user_role_ids: Vec<i64> = Vec::new();

            if let Ok(ids) = sys_user_role
                .filter(user_id.eq(&req.user_id))
                .select(role_id)
                .load::<i64>(conn)
            {
                user_role_ids = ids
            }

            let sys_role_result = sys_role.load::<SysRole>(conn);
            let mut sys_role_list: Vec<RoleList> = Vec::new();

            if let Ok(role_list) = sys_role_result {
                for x in role_list {
                    sys_role_list.push(RoleList {
                        id: x.id,
                        status_id: x.status_id,
                        sort: x.sort,
                        role_name: x.role_name,
                        remark: x.remark,
                        create_time: x.create_time.to_string(),
                        update_time: x.update_time.to_string(),
                    });
                }
            }

            Ok(BaseResponse::<QueryUserRoleResp>::ok_result_data(
                QueryUserRoleResp {
                    role_list: sys_role_list,
                    role_ids: user_role_ids,
                },
            ))
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_user_role(Json(req): Json<UpdateUserRoleReq>) -> impl IntoResponse {
    info!("update_user_role params: {:?}", req);
    let u_id = req.user_id;
    let role_ids = req.role_ids;

    if u_id == 1 {
        return BaseResponse::<String>::err_result_msg("不能修改超级管理员的角色".to_string());
    }

    match &mut RB.clone().get() {
        Ok(conn) => match diesel::delete(sys_user_role.filter(user_id.eq(u_id))).execute(conn) {
            Ok(_) => {
                let mut sys_role_user_list: Vec<AddSysUserRole> = Vec::new();
                for r_id in role_ids {
                    sys_role_user_list.push(AddSysUserRole {
                        role_id: r_id,
                        user_id: u_id.clone(),
                        create_time: Default::default(),
                    })
                }
                let result = diesel::insert_into(sys_user_role::table())
                    .values(&sys_role_user_list)
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

/*
 *更新用户信息
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn query_user_menu(headers: HeaderMap) -> Result<impl IntoResponse, impl IntoResponse> {
    let token = headers.get("Authorization").unwrap().to_str().unwrap();
    let split_vec = token.split_whitespace().collect::<Vec<_>>();
    if split_vec.len() != 2 || split_vec[0] != "Bearer" {
        return Err(BaseResponse::<String>::err_result_msg(
            "the token format wrong".to_string(),
        ));
    }
    let token = split_vec[1];
    let jwt_token_e = JWTToken::verify("123", &token);
    let jwt_token = match jwt_token_e {
        Ok(data) => data,
        Err(err) => {
            return match err {
                WhoUnfollowedError::JwtTokenError(er) => {
                    Err(BaseResponse::<String>::err_result_msg(er.to_string()))
                }
                _ => Err(BaseResponse::<String>::err_result_msg(
                    "other err".to_string(),
                )),
            };
        }
    };

    info!("query user menu params {:?}", jwt_token);

    match &mut RB.clone().get() {
        Ok(conn) => {
            match sql_query("select * from sys_user where id = ?")
                .bind::<Bigint, _>(jwt_token.id)
                .get_result::<SysUser>(conn)
            {
                Ok(user) => {
                    let user_role_sql =
                        sql_query("SELECT * FROM sys_user_role where user_id = ? and role_id = 1");
                    let sys_menu_list: Vec<SysMenu>;
                    match user_role_sql.bind::<Bigint, _>(&user.id).get_result::<SysUserRole>(conn) {
                         Ok(_) => {
                             match sys_menu.load::<SysMenu>(conn) {
                                 Ok(s_menus) => {
                                     sys_menu_list = s_menus;
                                 }
                                 Err(err) => {
                                     error!("err:{}", err.to_string());
                                     return Err(BaseResponse::<String>::err_result_msg(err.to_string()));
                                 }
                             }
                         }
                         Err(_) => {
                             match sql_query("select u.* from sys_user_role t left join sys_role usr on t.role_id = usr.id left join sys_role_menu srm on usr.id = srm.role_id left join sys_menu u on srm.menu_id = u.id where t.user_id = ? order by u.id asc")
                                 .bind::<Bigint, _>(&jwt_token.id)
                                 .load::<SysMenu>(conn) {
                                 Ok(s_menus) => {
                                     sys_menu_list = s_menus;
                                 }
                                 Err(err) => {
                                     error!("err:{}", err.to_string());
                                     return Err(BaseResponse::<String>::err_result_msg(err.to_string()));
                                 }
                             }
                         }
                     }

                    let mut sys_user_menu_list: Vec<MenuList> = Vec::new();
                    let mut btn_menu: Vec<String> = Vec::new();
                    let mut sys_menu_ids: HashSet<i64> = HashSet::new();

                    for x in sys_menu_list {
                        if x.menu_type != 3 {
                            sys_menu_ids.insert(x.parent_id.clone());
                            sys_menu_ids.insert(x.id.clone());
                        }

                        if x.api_url.clone().len() != 0 {
                            btn_menu.push(x.api_url);
                        }
                    }

                    let mut menu_ids = Vec::new();
                    for ids in sys_menu_ids {
                        menu_ids.push(ids)
                    }
                    match sys_menu
                        .filter(schema::sys_menu::id.eq_any(menu_ids))
                        .filter(schema::sys_menu::status_id.eq(1))
                        .order(crate::schema::sys_menu::sort.asc())
                        .distinct()
                        .load::<SysMenu>(conn)
                    {
                        Ok(menu_list) => {
                            for x in menu_list {
                                sys_user_menu_list.push(MenuList {
                                    id: x.id,
                                    parent_id: x.parent_id,
                                    name: x.menu_name,
                                    icon: x.menu_icon,
                                    api_url: x.api_url.clone(),
                                    menu_type: x.menu_type,
                                    path: x.menu_url,
                                });

                                if x.api_url.clone().len() != 0 {
                                    btn_menu.push(x.api_url);
                                }
                            }
                        }
                        Err(err) => {
                            error!("err:{}", err.to_string());
                            return Err(BaseResponse::<String>::err_result_msg(err.to_string()));
                        }
                    }

                    let resp = BaseResponse {
                         msg: "successful".to_string(),
                         code: 0,
                         data: Some(QueryUserMenuResp {
                             sys_menu:sys_user_menu_list,
                             btn_menu,
                             avatar: "https://gw.alipayobjects.com/zos/antfincdn/XAosXuNZyF/BiazfanxmamNRoxxVxka.png".to_string(),
                             name: user.user_name,
                         }),
                     };
                    Ok(Json(resp))
                }

                Err(err) => {
                    error!("err:{}", err.to_string());
                    Err(BaseResponse::<String>::err_result_msg(err.to_string()))
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            Err(BaseResponse::<String>::err_result_msg(err.to_string()))
        }
    }
}

/*
 *更新用户密码
 *author：刘飞华
 *date：2024/12/19 14:21:03
 */
pub async fn update_user_password(Json(item): Json<UpdateUserPwdReq>) -> impl IntoResponse {
    info!("update_user_pwd params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let user_sql = sql_query("SELECT * FROM sys_user where id = ? ");
            match user_sql
                .bind::<Bigint, _>(item.id)
                .get_result::<SysUser>(conn)
            {
                Ok(user) => {
                    if user.password != item.pwd {
                        error!("err:{}", "旧密码不正确".to_string());
                        return BaseResponse::<String>::err_result_msg("旧密码不正确".to_string());
                    }
                    let result = diesel::update(sys_user.filter(id.eq(item.id.clone())))
                        .set(schema::sys_user::password.eq(&item.re_pwd))
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
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(err.to_string())
        }
    }
}
