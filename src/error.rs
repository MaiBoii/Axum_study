/* ------------------------------- 에러 관련 미들웨어 ------------------------------- */
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize)]
pub enum Error{
    LoginFail,

    // 인증 관련 에러
	AuthFailNoAuthTokenCookie,
	AuthFailTokenWrongFormat,
	AuthFailCtxNotInRequestExt,

    // 티켓 삭제 요청시에 대상이 없으면 내는 에러
    TicketDeleteFailIdNotFound {id: u64},
}

//코어 포매팅 디스플레이의 에러부분 구현 개조하기
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		// Axum reponse에 에러 메시지 placeholder 만들기
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// 응답에 에러 메시지 넣기
		response.extensions_mut().insert(self);

		response
	}
}