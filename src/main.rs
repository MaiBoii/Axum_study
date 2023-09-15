#![allow(unused)] //for beginning only

use std::net::SocketAddr;
use crate::model::ModelController;
use crate::log::log_request;
pub use self::error::{Error, Result};

use axum::{Router, middleware, Json};
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::http::{Method, Uri};

use ctx::Ctx;
use serde::{Serialize,Deserialize};
use serde_json::{self, json};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

mod ctx;
mod error;
mod web;
mod model;
mod log;

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
        .layer(middleware::from_fn_with_state( //CookieManagerLayer로 옮기기 전에 최적화
            mc.clone(),
            web::mw_auth::mw_require_resolver
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

/* --------------------------------- 서버 스타팅 --------------------------------- */
    //몰랐는데 이렇게 명시한 경로로만 접속해야 되더라. 
    let addr = SocketAddr::from(([192,168,1,197], 8081));
    println!("->>{addr}에서 듣고 있습니다....\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())
    }

// RESPONSE 핸들러 알림 (12자 내로만)
async fn main_response_mapper(
    ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
    res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = Uuid::new_v4();

    //앞으로 일어날 수 도 있는 응답 에러 명시
    let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

    //클라이언트 에러 발생시 새로운 응답 생성
	let error_response =
        //클라이언트 상태 에러
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

				println!("    ->> 클라이언트 에러 내용: {client_error_body}");

				// Build the new response from the client_error_body
				(*status_code, Json(client_error_body)).into_response()
			});
    // Build and log the server log line.
	let client_error = client_status_error.unzip().1;
	// TODO: Need to hander if log_request fail (but should not fail request)
    let _ =
		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	println!();
	error_response.unwrap_or(res)
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

/* ----------------------------------- Say Hello --------------------------------------- */

//hello 페이지 연결
fn routes_hello() -> Router {
    //라우터 추가
    Router::new()
        // /hello 라우터로 접속하면 get 요청으로 handler_hello 실행
        .route("/hello", get(handler_hello))   
}

/* -------------------------------------------------------------------------- */

//handler_hello는 요청에서 받아온 HelloParams를 해독해서 실행
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    //params.name의 역참조로 param이 있으면 unwrap(), 없으면 디폴트값으로 World 반환
    let name = params.name.as_deref().unwrap_or("World");
    //HTML 응답
    Html(format!("Hello <strong>{name}!!</strong>"))
}