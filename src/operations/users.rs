use crate::schemas::User;

pub trait ValidateUser {
    fn validate_jwt(jwt: String) -> User;

}
