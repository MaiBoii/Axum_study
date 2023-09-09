use crate::{Error, Result, web};
use serde::Deserialize;
use axum::{Json, Router};
use axum::routing::post;
use serde_json::{json, Value};
use tower_cookies::{Cookies, Cookie};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    // TODO: 진짜 db, login 로직 구현하기
    if payload.username != "dummy_name" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO : 쿠키 세팅하기
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result" : {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}