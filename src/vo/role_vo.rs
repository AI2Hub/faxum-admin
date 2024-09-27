use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct QueryRoleListReq {
    #[serde(rename = "current")]
    pub page_no: u64,
    #[serde(rename = "pageSize")]
    pub page_size: u64,
    pub role_name: Option<String>,
    pub status_id: Option<i8>,
}

#[derive(Debug, Serialize)]
pub struct QueryRoleListData {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub role_name: String,
    pub remark: String,
    pub create_time: String,
    pub update_time: String,
}

#[derive(Debug, Deserialize)]
pub struct AddRoleReq {
    pub role_name: String,
    pub sort: i32,
    pub status_id: i8,
    pub remark: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleReq {
    pub id: i64,
    pub sort: i32,
    pub status_id: i8,
    pub role_name: String,
    pub remark: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct DeleteRoleReq {
    pub ids: Vec<i64>,
}


#[derive(Debug, Deserialize)]
pub struct QueryRoleMenuReq {
    pub role_id: i64,
}

#[derive(Debug, Serialize)]
pub struct QueryRoleMenuData {
    pub role_menus: Vec<i64>,
    pub menu_list: Vec<MenuDataList>,
}

#[derive(Debug, Serialize)]
pub struct MenuDataList {
    pub id: i64,
    pub parent_id: i64,
    pub title: String,
    pub key: String,
    pub label: String,
    #[serde(rename = "isPenultimate")]
    pub is_penultimate: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoleMenuReq {
    pub menu_ids: Vec<i64>,
    pub role_id: i64,
}


