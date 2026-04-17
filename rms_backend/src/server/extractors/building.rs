use crate::{entities::building::BuildingDTO, user::fields::Id};

pub fn dto(body: String) -> Result<BuildingDTO, Box<dyn std::error::Error>> {
    let mut name = String::new();
    let mut units_str = String::new();

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "building" => {
                    name = value.to_string().replace("+", " ");
                }
                "units" => {
                    units_str = value.to_string();
                }
                _ => {}
            }
        }
    }

    if name.is_empty() || units_str.is_empty() {
        return Err("Missing building name or units".into());
    }

    let number_of_units: i32 = units_str.parse()?;
    let id = Id::new();

    Ok(BuildingDTO::new(id, name, number_of_units)?)
}
