use uuid::Uuid;

use crate::{
    db,
    server::server::SessionStore,
    user::fields::{Id, Name},
};

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

pub fn update_profile(landlord_dto: LandlordDTO, session: &SessionStore) -> Result<String, String> {
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

    db::landlord::update_profile(landlord_entity)?;

    println!("Successfully updated landlords table.");

    let status_line = "HTTP/1.1 303 See Other";

    let location = "Location: /landlord";

    let response = format!(
        "{}\r\n{}\r\nContent-length: 0\r\nConnection: close\r\n\r\n",
        status_line, location
    );

    println!("Response: {}", response);

    Ok(response)
}

pub fn manage_buildings(lanlord_id: Uuid) -> Result<(String, String, String), String> {
    let buildings = db::building::get_stats(lanlord_id)?;

    let caretakers = db::caretaker::get().map_err(|e| e.to_string())?;

    let mut rows_html = String::new();

    for b in &buildings {
        let caretaker_name = db::building::get_caretaker(b.id)?;

        let display_name = caretaker_name.unwrap_or_else(|| "NOT ASSIGNED".to_string());

        rows_html.push_str(&format!(
            "<tr>
                <td>{name}</td>
                <td class='text-center'>{occ}/{total}</td>
                <td class='text-right'>{display_name}</td>
                </tr>",
            name = b.name,
            occ = b.occupied_units,
            total = b.units,
        ));
    }

    let mut caretaker_options = String::new();

    for (id, name) in caretakers {
        caretaker_options.push_str(&format!(
            "
                    <option value='{}'>{}</option>",
            id, name
        ));
    }

    let mut building_options = String::new();
    for b in &buildings {
        building_options.push_str(&format!(
            "
                <option value='{}'>{}</value>",
            b.id, b.name
        ));
    }

    Ok((rows_html, caretaker_options, building_options))
}
