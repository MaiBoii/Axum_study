#![allow(unused)] //for beginning only

use std::net::SocketAddr;
use crate::model::ModelController;
pub use self::error::{Error, Result};

use axum::{Router, middleware};
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};

use serde::{Serialize,Deserialize};
use serde_json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod ctx;
mod error;
mod web;
mod model;

#[tokio::main]
async fn main() -> Result<()>{
    // 모델 컨트롤러 초기화 
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_ticket::routes(mc.clone())
		.route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    //라우터 모음
    let routes_all = Router::new()
        //Hello 라우터를 현재 라우터에 병합함.
        .merge(routes_hello())
        //로그인 관련 라우터들을 현재 라우터에 병합
        .merge(web::routes_login::routes())
        //routes_apis라는 API 관련 라우터를 /api 경로에 중첩시켜 
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper)) //미들웨어 맵 response
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

/* --------------------------------- 서버 스타팅 --------------------------------- */
    //
    let addr = SocketAddr::from(([192,168,1,197], 8081));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
    }

// 
async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}

//
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

//HelloParams의 역직렬화 기능 추가
#[derive(Debug, Deserialize)]
//문자열 형태로 name 지정
struct HelloParams{
    name: Option<String>
}

//hello 페이지 연결
fn routes_hello() -> Router {
    //라우터 추가
    Router::new()
        // /hello 라우터로 접속하면 get 요청으로 handler_hello 실행
        .route("/hello", get(handler_hello))   
}

//handler_hello는 요청에서 받아온 HelloParams를 해독해서 실행
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    //params.name의 역참조로 param이 있으면 unwrap(), 없으면 디폴트값으로 World 반환
    let name = params.name.as_deref().unwrap_or("World");
    //HTML 응답
    Html(format!("Hello <strong>{name}!!</strong>"))
}