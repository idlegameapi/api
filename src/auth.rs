#[derive(Debug)]
pub struct AuthorizationError;

impl warp::reject::Reject for AuthorizationError {}

impl From<std::string::FromUtf8Error> for AuthorizationError {
    fn from(_: std::string::FromUtf8Error) -> Self {
        AuthorizationError
    }
}

impl From<base64::DecodeError> for AuthorizationError {
    fn from(_: base64::DecodeError) -> Self {
        AuthorizationError
    }
}

pub struct Auth(String);

impl std::str::FromStr for Auth {
    type Err = AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.strip_prefix("Basic ") {
            Some(token) if !token.contains(" ") && !token.is_empty() => {
                let bytes = base64::decode(token)?;
                let valid = String::from_utf8(bytes)?;
                Ok(Self(valid))
            }
            _ => Err(AuthorizationError),
        }
    }
}
