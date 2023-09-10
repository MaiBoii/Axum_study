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
        //Hello 라우터 
        .merge(routes_hello())
        //로그인 관련 라우터
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    // region:   ----서버 스타팅
    let addr = SocketAddr::from(([127,0,0,1], 8081));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
    }

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
    
}

#[derive(Debug, Deserialize)]
struct HelloParams{
    name: Option<String>
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
    
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello <strong>{name}!!</strong>"))
}