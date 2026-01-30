use crate::user::fields::{Id, Name};

use crate::db;
use crate::server::SessionStore;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Landlord {
    pub id: Id,
    pub business_name: Name,
}

impl Landlord {
    pub fn new(id: Uuid, business_name: String) -> Result<Self, String> {
        Ok(Self {
            id: Id::from(id),
            business_name: Name::try_from(business_name)?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LandlordDTO {
    pub session_id: Option<String>,
    pub name: Name,
}

impl LandlordDTO {
    pub fn new(business_name: String) -> Result<Self, String> {
        Ok(Self {
            session_id: None,
            name: Name::try_from(business_name)?,
        })
    }
}

pub fn update_landlord(
    landlord_dto: LandlordDTO,
    session: &SessionStore,
) -> Result<String, String> {
    let dto_name = landlord_dto.name.value();

    let session_id = landlord_dto.session_id.ok_or("No Session Id found")?;

    let landlord_uuid = {
        let lock = session.lock().unwrap();
        lock.get(&session_id)
            .cloned()
            .ok_or("Invlaid or Expired Session")?
    };

    let landlord_id = Id::from(landlord_uuid);

    let name = Name::try_from(dto_name.to_string())?;

    let landlord_entity = Landlord {
        id: landlord_id,
        business_name: name,
    };

    db::update_landlord(landlord_entity)?;

    println!("Successfully updated landlords table.");

    let status_line = "HTTP/1.1 303 See Other";

    let location = "Location: /landlord";

    let response = format!("{}\r\n{}\r\nContent-length: 0\r\nConnection: close\r\n\r\n",
        status_line, location);

    println!("Response: {}", response);

    Ok(response)
}

