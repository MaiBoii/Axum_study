#![allow(unused)] //for beginning only

use std::net::SocketAddr;

pub use self::error::{Error, Result};

mod error;
mod web;
mod model;

use axum::{Router, middleware};
use axum::extract::Query;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};

use serde::{Serialize,Deserialize};
use serde_json;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    //라우터 모음
    let routes_all = Router::new()
        //Hello 라우터 
        .merge(routes_hello())
        //로그인 관련 라우터
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    // region:   ----서버 스타팅
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
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