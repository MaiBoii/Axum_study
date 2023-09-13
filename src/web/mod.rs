pub mod mw_auth;
pub mod routes_login;
pub mod routes_ticket;

//따로 저장해둔 인증 토큰 정보 
//실제로 만들 때는 dotenv로 저장해둬야지
pub const AUTH_TOKEN: &str = "auth-token";