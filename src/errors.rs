use warp::{Rejection, Reply};

use crate::auth::AuthorizationError;

pub trait ToInternalError<T> {
    fn to_internal_error(self) -> Result<T, Rejection>;
}

impl<T, E> ToInternalError<T> for Result<T, E> {
    fn to_internal_error(self) -> Result<T, Rejection> {
        self.map_err(|_| warp::reject::custom(InternalError))
    }
} 

#[derive(Debug)]
pub struct NotFound;

impl warp::reject::Reject for NotFound {}

#[derive(Debug)]
pub struct Conflict;

impl warp::reject::Reject for Conflict {}

#[derive(Debug)]
pub struct InternalError;

impl warp::reject::Reject for InternalError {}

#[derive(Debug)]
pub struct NotAuthorized;

impl warp::reject::Reject for NotAuthorized {}

#[derive(Debug)]
pub struct NotEnoughMoney;

impl warp::reject::Reject for NotEnoughMoney {}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.find::<NotEnoughMoney>().is_some() {
        Ok(crate::warp_reply!(
            "Not enough money to upgrade".to_owned(),
            FORBIDDEN
        ))
    } else if err.find::<NotFound>().is_some() {
        Ok(crate::warp_reply!(
            "There is no user with that name".to_owned(),
            NOT_FOUND
        ))
    } else if let Some(e) = err.find::<AuthorizationError>() {
        Ok(crate::warp_reply!(format!("{:?}", e), BAD_REQUEST))
    } else if err.find::<NotAuthorized>().is_some() {
        Ok(crate::warp_reply!(
            "The password is incorrect".to_owned(),
            UNAUTHORIZED
        ))
    }
    else if err.find::<Conflict>().is_some() {
        Ok(crate::warp_reply!(
            "The provided username already exists".to_owned(),
            CONFLICT
        ))
    } else {
        eprintln!("An unknown error occured: {:?}", err);
        Ok(crate::warp_reply!(
            "Something went wrong, please try again later.".to_owned(),
            INTERNAL_SERVER_ERROR
        ))
    }
}
