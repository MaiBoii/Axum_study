//쿠키 인증 미들웨어 작성 과정
use async_trait::async_trait;
use axum::RequestPartsExt;
use axum::extract::{FromRequest, FromRequestParts};
use axum::http::Request;
use axum::http::request::Parts;
use axum::middleware::Next;
use httpc_test::Cookie;
use axum::response::Response;
use tower_cookies::Cookies;
use lazy_regex::{self, regex_captures};

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};


pub async fn mw_require_auth<B>(
	ctx: Result<Ctx>,
	req: Request<B>,
	next: Next<B>,
) -> Result<Response> {
	println!("->> {:<12} - mw_require_auth - {ctx:?}", "MIDDLEWARE");

	ctx?;

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
        println!("->> {:>12} - Ctx", "EXTRACTOR");

        let cookies = parts.extract::<Cookies>().await.unwrap();

		//인증 토큰 - 
        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie)
            .and_then(parse_token)?;

        Ok(Ctx::new(user_id))
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