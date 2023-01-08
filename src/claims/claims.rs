use serde::{Deserialize, Serialize};

/// Claims are the registered claims defined in the RFC 7519.
///
/// The following claims are registered in the IANA "JSON Web Token
/// Claims" registry established by Section 10.1.  None of the claims
/// defined below are intended to be mandatory to use or implement in
/// all cases, but rather they provide a starting point for a set of
/// useful, interoperable claims.  Applications using JWTs should
/// define which specific claims they use and when they are required
/// or optional.
///
/// # Arguments
/// * `exp` - A string slice that holds the expiration date.
/// * `iat` - A string slice that holds the issued at date.
/// * `iss` - A string slice that holds the issuer.
/// * `sub` - A string slice that holds the subject.
/// * `aud` - A string slice that holds the audience.
///
/// # Examples
/// ```
/// use mini_functions::claims::claims::Claims;
///
/// let claims = Claims::new(
///     "2023-12-23T23:23:23.222222+00:00",
///     "2023-12-23T23:23:23.222222+00:00",
///     "issuer",
///     "subject",
///     "audience",
/// );
/// ```
///
#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub(crate) exp: String, // expiration date
    pub(crate) iat: String, // issued at date
    pub(crate) iss: String, // issuer
    pub(crate) sub: String, // subject
    pub(crate) aud: String, // audience
}

impl Claims {
    /// Create a new instance of `Claims`.
    /// # Arguments
    /// * `exp` - A string slice that holds the expiration date.
    /// * `iat` - A string slice that holds the issued at date.
    /// * `iss` - A string slice that holds the issuer.
    /// * `sub` - A string slice that holds the subject.
    /// * `aud` - A string slice that holds the audience.
    ///
    /// # Examples
    /// ```
    /// use mini_functions::claims::claims::Claims;
    ///
    /// let claims = Claims::new(
    ///     "2023-12-23T23:23:23.222222+00:00",
    ///     "2023-12-23T23:23:23.222222+00:00",
    ///     "issuer",
    ///     "subject",
    ///     "audience",
    /// );
    /// ```
    ///
    pub fn new(exp: &str, iat: &str, iss: &str, sub: &str, aud: &str) -> Self {
        Claims {
            exp: String::from(exp),
            iat: String::from(iat),
            iss: String::from(iss),
            sub: String::from(sub),
            aud: String::from(aud),
        }
    }

    /// Get the expiration date.
    pub fn exp(&self) -> &str {
        &self.exp
    }
    /// Get the issued at date.
    pub fn iat(&self) -> &str {
        &self.iat
    }
    /// Get the issuer.
    pub fn iss(&self) -> &str {
        &self.iss
    }
    /// Get the subject.
    pub fn sub(&self) -> &str {
        &self.sub
    }
    /// Get the audience.
    pub fn aud(&self) -> &str {
        &self.aud
    }
}
