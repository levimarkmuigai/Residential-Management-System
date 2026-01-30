use crate::user::fields::{
    Id,
    Name,
    Role,
    Email,
    PhoneNumber,
    Password
};

use uuid::Uuid;


#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Id,
    pub first_name: Name,
    pub last_name: Name,
    pub role: Role,
    pub email: Email,
    pub phone_number: PhoneNumber,
    pub password: Password,
}

impl User {

    pub fn new(
        id: Uuid,
        first_name: String,
        last_name: String,
        role: String,
        email: String,
        phone_number: String,
        password: String,
    ) -> Result<User, String> {
        let hashed_password = Password::new(password)?;

        Ok(Self {
            id: Id::from(id),
            first_name: Name::try_from(first_name)?,
            last_name: Name::try_from(last_name)?,
            role: Role::try_from(role)?,
            email: Email::try_from(email)?,
            phone_number: PhoneNumber::try_from(phone_number)?,
            password: Password::try_from(hashed_password)?,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UserCredentials {
    pub email: Email,
    pub password: Password,
}

impl UserCredentials {

    pub fn new(email: String,password: String) -> Result<UserCredentials, String> {

        Ok(Self {
            email: Email::try_from(email)?,
            password: Password::try_from(password)?,
        })
    }
}

