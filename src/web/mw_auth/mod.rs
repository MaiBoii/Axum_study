//쿠키 인증 미들웨어 작성 과정

use axum::http::Request;
use axum::middleware::Next;
use httpc_test::Cookie;
use axum::response::Response;
use tower_cookies::Cookies;

use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth<B>(
    cookies: Cookie,
    req: Request<B>, 
    next: Next<B>
) ->  Result<Response>{
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

    //쿠키 인증 실패시 에러처리 아니면 ok 시키는 예외처리 과정
    auth_token.ok_or(Error::AuthFailNoAuthTokenCookie)?;

    Ok(next.run(req).await)
}