use crate::{
    db,
    user::fields::{Id, Name},
};
use crate::server::SessionStore;

use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Building {
    pub id: Id,
    pub name: Name,
    pub landlord_id: Id,
    pub units: i32,
}

impl Building {
    pub fn new(id: Uuid, name: String, landlord_id: Uuid, units: i32) -> Result<Self, String> {
        Ok(Self {
            id: Id::from(id),
            name: Name::try_from(name)?,
            landlord_id: Id::from(landlord_id),
            units,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuildingDTO {
    pub id: Id,
    pub name: Name,
    pub session_id: Option<String>,
    pub units: i32,
}

impl BuildingDTO {
    pub fn new(id: Uuid, name: String, units: i32) -> Result<Self, String> {
        Ok(Self {
            id: Id::from(id),
            name: Name::try_from(name)?,
            session_id: None,
            units,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Unit {
    pub id: Id,
    pub building_id: Id,
    pub unit_number: i32,
    pub tenant_id: Option<Id>,
    pub is_occupied: bool,
}

impl Unit {
    pub fn new(
        id: Uuid,
        building_id: Uuid,
        unit_number: i32,
        tenant_id: Option<Uuid>,
        is_occupied: bool,
    ) -> Result<Self, String> {
        Ok(Self {
            id: Id::from(id),
            building_id: Id::from(building_id),
            unit_number,
            tenant_id: tenant_id.map(Id::from),
            is_occupied,
        })
    }
}

pub fn insert_building(
    building_dto: BuildingDTO,
    session: &SessionStore,
) -> Result<String, String> {
    let name = building_dto.name;
    let units = building_dto.units;

    let session_id = building_dto.session_id
        .ok_or("No session id found")?;

    let landlord_uuid = {
        let lock = session.lock().unwrap();
        lock.get(&session_id)
            .cloned()
            .ok_or("Invalid or Expired Session")?
    };

    let landlord_id = Id::from(landlord_uuid);

    let building_id = building_dto.id;

    let building_entity = Building {
        id: building_id,
        name: name,
        landlord_id: landlord_id,
        units,
    };

    db::insert_buidling(building_entity)?;
    println!("Building inserted to Database Successfully");

    let status_line = "HTTP/1.1 303 See Other";

    let location = "Location: /landlord";
    
    let response = format!("{}\r\n{}\r\nContent-length: 0\r\nConnection: close\r\n\r\n", status_line, location);

    println!("Response: {}", response);

    Ok(response)
}

