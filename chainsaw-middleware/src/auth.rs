//

//!
//!
//!

use hyper::{header, Request, Response};
use tonic::Status;
use tower_http::auth::AuthorizeRequest;

use crate::jwt::decode_jwt;

#[derive(Debug, Clone)]
pub struct UserIdExtension(String);

#[derive(Debug, Clone)]
pub struct CustomerIdExtension(String);

#[derive(Debug, Clone)]
pub struct JwtExtension(String);

#[derive(Clone, Debug, Default)]
pub struct ParseJWTAuth {
    paths: Vec<String>,
}

impl ParseJWTAuth {
    ///
    ///
    ///
    pub fn new(paths: Vec<String>) -> Self {
        Self { paths }
    }
}

impl<B> AuthorizeRequest<B> for ParseJWTAuth {
    type ResponseBody = tonic::body::BoxBody;

    fn authorize(&mut self, request: &mut Request<B>) -> Result<(), Response<Self::ResponseBody>> {
        let path = request_path(&request);

        if !self.paths.contains(&path) {
            tracing::info!(%path, "path does not require authentication");
            return Ok(());
        }

        if let Some(token) = check_auth(&request) {
            match decode_jwt(&token) {
                Ok(claims) => {
                    request.extensions_mut().insert(JwtExtension(token));
                    request.extensions_mut().insert(UserIdExtension(claims.sub));
                    request
                        .extensions_mut()
                        .insert(CustomerIdExtension(claims.user_ctx));

                    Ok(())
                }
                Err(ref error) => {
                    tracing::info!(%error, "failed to decode jwt");
                    Err(Status::invalid_argument("invalid jwt received").to_http())
                }
            }
        } else {
            Err(
                Status::invalid_argument("missing or invalid authorization header received")
                    .to_http(),
            )
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

fn request_path<B>(request: &Request<B>) -> String {
    request.uri().path().to_string()
}
