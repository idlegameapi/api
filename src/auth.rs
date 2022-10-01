pub enum AuthType {
    Basic,
    Bearer,
    Digest,
    HOBA,
    Mutual,
    Negotiate,
    VAPID,
    SCRAM
}

#[derive(Debug)]
pub struct AuthorizationError;
impl warp::reject::Reject for AuthorizationError {}

pub type Token = String;
pub struct AuthPair(AuthType, Token);

impl std::str::FromStr for AuthPair {
    type Err = AuthorizationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<_> = s.split(" ").collect();
        if s.len() > 2 { return Err(AuthorizationError) };

        let x = match s[0] {
            "Basic" => AuthType::Basic,
            "Bearer" => AuthType::Bearer,
            "Digest" => AuthType::Digest,
            "HOBA" => AuthType::HOBA,
            "Mutual" => AuthType::Mutual,
            "Negotiate" => AuthType::Negotiate,
            "VAPID" => AuthType::VAPID,
            "SCRAM" => AuthType::SCRAM,
            _ => return Err(AuthorizationError)
        };

        Ok(AuthPair(x, s[1].to_string()))
    }
}