// 

//!
//!
//!

use tonic::Status;
use tower_http::auth::AuthorizeRequest;
use hyper::{header, Request, Response};

use crate::jwt::decode_jwt;

#[derive(Debug, Clone)]
pub enum Extensions {
    UserId(String),
    CustomerId(String),
    Jwt(String),
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ParseJWTAuth;

impl<B> AuthorizeRequest<B> for ParseJWTAuth {
    type ResponseBody = tonic::body::BoxBody;

    fn authorize(
        &mut self,
        request: &mut Request<B>,
    ) -> Result<(), Response<Self::ResponseBody>> {
        if let Some(token) = check_auth(&request) {
            match decode_jwt(&token) {
                Ok(claims) => {
                    request.extensions_mut().insert(Extensions::Jwt(token));
                    request.extensions_mut().insert(Extensions::UserId(claims.sub));
                    request.extensions_mut().insert(Extensions::CustomerId(claims.user_ctx));

                    Ok(())
                }
                Err(ref error) => {
                    tracing::info!(%error, "failed to decode jwt");
                    Err(Status::invalid_argument("invalid jwt received").to_http())
                }
            }
        } else {
            Err(Status::invalid_argument("missing or invalid authorization header received").to_http())
        }
    }
}

fn check_auth<B>(request: &Request<B>) -> Option<String> {
    if let Some(value) = request.headers().get(header::AUTHORIZATION) {
        match value.to_str() {
            Ok(value) => {
                if value.to_ascii_lowercase().starts_with("bearer ") {
                    if let Some((_, token)) = value.split_once(" ") {
                        Some(token.to_string())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Err(ref error) => {
                tracing::info!(%error, "invalid auth value detected");
                None
            }
        }
    } else {
        None
    }
}
