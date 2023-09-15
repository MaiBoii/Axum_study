/* ------------------------------- 에러 관련 미들웨어 ------------------------------- */
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
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

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			// -- Auth.
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}

			// -- Model.
			Self::TicketDeleteFailIdNotFound { .. } => {
				(StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
			}

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}


//enum 원소들을 전역 str 참조자들로 변환
#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	INVALID_PARAMS,
	SERVICE_ERROR,
}