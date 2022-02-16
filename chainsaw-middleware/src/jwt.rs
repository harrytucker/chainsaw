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
///
/// # Example
/// ```
/// # use chainsaw_middleware::jwt::JWTError;
/// use chainsaw_middleware::jwt::decode_jwt;
///
/// # fn example() -> Result<(), JWTError> {
/// let claims = decode_jwt("an.example.jwt")?;
///
/// println!("user id:     {}", claims.sub);
/// println!("customer id: {}", claims.user_ctx);
/// # Ok(())
/// # }
/// ```
pub fn decode_jwt<S: AsRef<str>>(token: S) -> Result<Claims, JWTError> {
    let token = token.as_ref();
    let data = jsonwebtoken::decode::<Claims>(token, &DECODING_KEY, &VALIDATION)?;

    Ok(data.claims)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_JWT: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJodHRwczovL2V4YW1wbGUuY29tIiwiYXVkIjoiYXVkIiwic3ViIjoiZXhhbXBsZUBleGFtcGxlLmNvbSIsInVzZXJfY3R4IjoiODkyMmFmYTY3MjMwMTFlYmJlMDFjYTMyZDMyYjZiNzciLCJpYXQiOjE2MTU5MTAzOTksImV4cCI6MTYxNTkxNzU5OX0.KGVrRxlehq_h6qmtGXM-i-2IKJbMRqrmSj6FDEXaHEo";

    #[test]
    fn test_decode_jwt() -> Result<(), JWTError> {
        let claims = decode_jwt(EXAMPLE_JWT)?;

        assert_eq!(claims.sub, "example@example.com");
        assert_eq!(claims.user_ctx, "8922afa6723011ebbe01ca32d32b6b77");

        Ok(())
    }
}
