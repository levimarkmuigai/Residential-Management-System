use uuid::Uuid;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher,
        SaltString
    },
    Argon2
};

#[derive(Debug, PartialEq, Clone)]
pub struct Id{
    pub id: Uuid,
}

impl Id {

    pub fn new() -> Uuid {
        Uuid::new_v4()
    }

    pub fn value(&self)  -> &Uuid {
        &self.id
    }
}

impl From<Uuid> for Id {

    fn from(id: Uuid) -> Id {
        Id{id}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Name{

    pub raw: String,
}

impl Name {

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for Name {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("ℕame is empty");
        }

        Ok(Self{ 
            raw: value_trimmed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Role{
    pub raw: String,
}

impl Role {

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for Role {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Role is empty");
        }

        Ok(Self{ 
            raw: value_trimmed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Email{
    pub raw: String,
}

impl Email {

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for Email {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Email is empty");
        }

        Ok(Self{ 
            raw: value_trimmed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct PhoneNumber {
    pub raw: String,
}

impl PhoneNumber {

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for PhoneNumber {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Phone number is empty");
        }

        Ok(Self{ 
            raw: value_trimmed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Password {
    pub raw: String,
}

impl Password {

    pub fn new(password: String) -> Result<String, String> {

        let password_trimmed = password.trim();

        if password_trimmed.is_empty() {
            return Err("Password is empty".to_string());
        }

        let unhashed_password = password_trimmed.to_string();

        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let hashed_password = argon2
            .hash_password(unhashed_password.as_bytes(), &salt)
            .map_err(|e| e.to_string())?
            .to_string();

        Ok(hashed_password)

    }

    pub fn value(&self) -> &str {
        &self.raw
    }
}

impl TryFrom<String> for Password {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Password is empty");
        }

        Ok(Self{
            raw: value_trimmed.to_string(),
        })
    }
}

