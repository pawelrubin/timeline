use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use rocket::serde::{Serialize,Deserialize};

#[derive(Debug)]
pub enum AuthError {
    MissingHeader,
    InvalidHeader,
    InvalidToken,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserClaims {
    /// uid of the user.
    pub sub: String,
    /// email address of the user.
    pub email: String,
}

pub async fn decode_token(token: &str) -> Result<UserClaims, Box<dyn std::error::Error>> {
    let decoded_id_token = match jsonwebtoken::dangerous_insecure_decode::<UserClaims>(token) {
        Ok(value) => value.claims,
        Err(error) => return Err(std::boxed::Box::from(format!("{:?}", error))),
    };

    Ok(decoded_id_token)
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserClaims {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<UserClaims, Self::Error> {
        let auth_haeder_value = match req.headers().get_one("Authorization") {
            Some(value) => value,
            None => {
                return request::Outcome::Failure((Status::BadRequest, AuthError::MissingHeader))
            }
        };

        if let Some(token) = auth_haeder_value.strip_prefix("Bearer ") {
            match decode_token(token).await {
                Ok(value) => return request::Outcome::Success(value),
                Err(_) => {
                    return request::Outcome::Failure((Status::BadRequest, AuthError::InvalidToken))
                }
            };
        }

        request::Outcome::Failure((Status::BadRequest, AuthError::InvalidHeader))
    }
}
