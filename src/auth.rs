use base64::DecodeError;
use std::string::FromUtf8Error;

#[derive(Debug, PartialEq, Eq)]
pub enum InvalidBase64 {
    InvalidByte { index: usize, offending_byte: u8 },
    InvalidLength,
    InvalidLastSymbol { index: usize, offending_byte: u8 },
}

impl From<base64::DecodeError> for InvalidBase64 {
    fn from(err: base64::DecodeError) -> Self {
        match err {
            DecodeError::InvalidByte(index, offending_byte) => InvalidBase64::InvalidByte {
                index,
                offending_byte,
            },
            DecodeError::InvalidLength => InvalidBase64::InvalidLength,
            DecodeError::InvalidLastSymbol(index, offending_byte) => {
                InvalidBase64::InvalidLastSymbol {
                    index,
                    offending_byte,
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum AuthorizationError {
    #[error("the provided base64 is invalid: {0}")]
    InvalidBase64(InvalidBase64),
    #[error("the decoded UTF-8 is invalid: {0}")]
    InvalidUTF8(FromUtf8Error),
    #[error("the authentication format was not Basic")]
    NotBasicAuthentication,
}

impl warp::reject::Reject for AuthorizationError {}

impl From<FromUtf8Error> for AuthorizationError {
    fn from(err: FromUtf8Error) -> Self {
        AuthorizationError::InvalidUTF8(err)
    }
}

impl From<base64::DecodeError> for AuthorizationError {
    fn from(err: base64::DecodeError) -> Self {
        AuthorizationError::InvalidBase64(err.into())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

pub fn validate_header(s: &str) -> Result<Auth, AuthorizationError> {
    match s.strip_prefix("Basic ") {
        Some(token) if !token.contains(' ') && !token.is_empty() => {
            let bytes = base64::decode(token)?;
            let valid = String::from_utf8(bytes)?;
            let (username, password) = valid.split_once(':').unwrap();

            Ok(Auth {
                username: username.to_string(),
                password: password.to_string(),
            })
        }
        _ => Err(AuthorizationError::NotBasicAuthentication),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation() {
        let expected = Auth {
            username: "username".to_string(),
            password: "password".to_string(),
        };

        let got = validate_header("Basic dXNlcm5hbWU6cGFzc3dvcmQ=").unwrap();

        assert_eq!(expected, got);
    }
}
