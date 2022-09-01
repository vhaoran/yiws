use crate::dao;
use crate::model::CodeLib;
use axum::extract::Query;
use serde::Deserialize;
use serde::Serialize;

pub async fn add_code(Query(params): Query<ParamsDays>) -> String {
    let days = params.days.unwrap_or(7);
    //
    let src = CodeLib::new(days);
    let r = dao::code_lib::insert_one(src.clone(), None).await;
    if r.is_err() {
        return "add fail".to_string();
    }

    let code = src.id.unwrap_or("没有激活".to_string());
    let day = src.expired_str.unwrap_or("遥遥无期".to_string());

    // let s = serde_json::to_string(&src).unwrap_or("".to_string());
    let s = format!("激活码:{code}: 过期时间： {day}");

    s
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ParamsDays {
    pub days: Option<i64>,
}
