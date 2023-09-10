use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[derive(Debug)]
pub enum Error{
    LoginFail,

    // 인증 관련 에러
	AuthFailNoAuthTokenCookie,
	AuthFailTokenWrongFormat,
	AuthFailCtxNotInRequestExt,

    // 모델 관련 에러
    TicketDeleteFailIdNotFound {id: u64},
}

impl std::error::Error for Error {}


impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}