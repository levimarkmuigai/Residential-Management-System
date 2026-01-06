use uuid::Uuid;

use sha2::{Sha256, Digest};

#[derive(Debug, PartialEq, Clone)]
pub struct Id(pub Uuid);

impl Id {

    pub fn generate_id() -> Uuid {
        Uuid::new_v4()
    }
}

impl From<Uuid> for Id {

    fn from(id: Uuid) -> Id {
        Id(id)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FirstName{

    pub raw: String,
}

impl FirstName {

    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for FirstName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("First name is empty");
        }

        Ok(Self{ 
            raw: value_trimmed.to_string(),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LastName{
    pub raw: String,
}

impl LastName {

    pub fn new(raw: String) -> Self {
        Self { raw }
    }

    pub fn value(&self) -> &str {

        &self.raw
    }
}

impl TryFrom<String> for LastName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value_trimmed = value.trim();

        if value_trimmed.is_empty() {
            return Err("Last name is empty");
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

    pub fn new(raw: String) -> Self {
        Self { raw }
    }

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

    pub fn new(raw: String) -> Self {
        Self { raw }
    }

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

    pub fn new(raw: String) -> Self {
        Self { raw }
    }

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

        let mut hasher = Sha256::new();

        hasher.update(unhashed_password);

        let hashed_password = hasher.finalize();

        let hashed_string_password = format!("{:x}", hashed_password);

        Ok(hashed_string_password)

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

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub id: Id,
    pub first_name: FirstName,
    pub last_name: LastName,
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

        let u_id = Id::from(id);
        let f_name = FirstName::try_from(first_name)?;
        let l_name = LastName::try_from(last_name)?;
        let u_role = Role::try_from(role)?;
        let u_email = Email::try_from(email)?;
        let p_number = PhoneNumber::try_from(phone_number)?;

        let hashed_password = Password::new(password)?;

        let u_password = Password::try_from(hashed_password)?;

        Ok(Self {
            id: u_id,
            first_name: f_name,
            last_name: l_name,
            role: u_role,
            email: u_email,
            phone_number: p_number,
            password: u_password,
        })
    }
}
