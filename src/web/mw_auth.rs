//쿠키 인증 미들웨어 작성 과정
use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::{FromRequest, FromRequestParts, State};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};
use lazy_regex::{self, regex_captures};

use crate::ctx::Ctx;
use crate::model::ModelController;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
	//인증된 사용자 정보
	ctx: Result<Ctx>,
	//요청 객ㅊ[
	req: Request<B>,
	//미들웨어 스택 내 다음으로 실행될 핸들러or미들웨어를 나타냄
	next: Next<B>,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	//ctx가 ok면 진행, err면 끊음.
	ctx?;

	//다음으로 진행될 미들웨어에다 req담아서 실행
	Ok(next.run(req).await)
}

//토큰 검증 작업 전 최적화 미들웨어 
pub async fn mw_require_resolver<B>(
	//실제론 안 쓸건데 예제로 쓸거,
	_mc: State<ModelController>,
	cookies: Cookies,
	mut req: Request<B>,
	next:Next<B>,
) -> Result<Response>{
	println!("->> {:<12} - mw_ctx_resolver", "MIDDLEWARE");

	// axum의 쿠키 추출자 사용
	//let cookies = parts.extract::<Cookies>().await.unwrap();

	//인증 토큰
	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	//받아온 인증정보 추산하기
	let result_ctx = match auth_token
		.ok_or(Error::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok((user_id, _exp, _sign)) => {
			// 이따 할거 : 토큰 구성 요소 검증하기(리소스를 꽤 잡아먹는 작업이라 최적화 요망)
			Ok(Ctx::new(user_id))
		}
		Err(e) => Err(e),
	};
	
	// NoAuthTokenCookie 에러 조건과도 맞지 않은 무언가가 왔을 경우 Cookie 삭제
	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
	{
		cookies.remove(Cookie::named(AUTH_TOKEN))
	}

	// 요청 확장자에 ctx_result를 추가
	req.extensions_mut().insert(result_ctx);
	
	Ok(next.run(req).await)
}

/* --------------------------------- CTX 추출자 -------------------------------- */
// HTTP 요청으로부터 Ctx 타입을 추출하는 implment 짜기

// FromRequestParts 트레이트를 비동기적으로 구현하기
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    //HTTP 요청의 Parts와 State(상태)를 입력으로 받아 Result<Self> 타입을 반환함.

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:>12} - Ctx", "추출자");

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
    }
}
/* -------------------------------------------------------------------------- */



/* ------------------------------- 인증 데이터 파싱하기 ------------------------------ */

fn parse_token(token: String) -> Result<(u64, String, String)> {
	let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
		&token
	)
	.ok_or(Error::AuthFailTokenWrongFormat)?;

	let user_id: u64 = user_id
		.parse()
		.map_err(|_| Error::AuthFailTokenWrongFormat)?;

	Ok((user_id, exp.to_string(), sign.to_string()))
}

/* -------------------------------------------------------------------------- */