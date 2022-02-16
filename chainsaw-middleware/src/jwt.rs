//

//!
//!
//!

use std::collections::HashSet;

pub use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use lazy_static::lazy_static;
use serde::Deserialize;

lazy_static! {
    // a blank decoding key as we disable signature verification
    static ref DECODING_KEY: DecodingKey = DecodingKey::from_secret(&[]);

    // a placeholder validation that turns every validation check off
    static ref VALIDATION: Validation = {
        let mut validation = Validation::new(Algorithm::HS256);

        validation.insecure_disable_signature_validation();
        validation.required_spec_claims = HashSet::new();
        validation.validate_exp = false;

        validation
    };
}

/// A subset of the claims within a JWT, limited to those we're interested in.
#[derive(Clone, Debug, Deserialize)]
pub struct Claims {
    /// The user ID of the user the JWT is for.
    pub sub: String,

    /// The application customer ID of the user the JWT is for.
    pub user_ctx: String,
}

/// Decode a JWT into a set of claims.
///
/// This performs zero validation upon the JWT for the signature, expiry, etc. as all such concerns
/// are beyond the scope of microservices and are instead deferred to Istio and Authz.
pub fn decode_jwt<S: AsRef<str>>(token: S) -> Result<Claims, JWTError> {
    let token = token.as_ref();
    let data = jsonwebtoken::decode::<Claims>(token, &DECODING_KEY, &VALIDATION)?;

    Ok(data.claims)
}
