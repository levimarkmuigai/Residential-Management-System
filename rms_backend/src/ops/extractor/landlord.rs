use uuid::Uuid;

use crate::entities::landlord::LandlordDTO;

pub fn dto(body: String) -> Result<LandlordDTO, String> {
    let mut business_name = String::new();

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "name" => {
                    business_name = value.to_string().replace('+', " ");
                    println!("{} was extracted.", business_name);
                }
                _ => {}
            }
        }
    }

    Ok(LandlordDTO::new(business_name)?)
}

pub fn assignment_params(body: String) -> Result<(Uuid, Uuid), String> {
    let mut building_id = None;
    let mut caretaker_id = None;

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "building_id" => {
                    building_id = Some(Uuid::parse_str(value).map_err(|_| "Invalid building id")?);
                }

                "caretaker_id" => {
                    caretaker_id =
                        Some(Uuid::parse_str(value).map_err(|_| "Invalid caretaker id")?);
                }

                _ => {}
            }
        }
    }

    match (building_id, caretaker_id) {
        (Some(b), Some(c)) => Ok((b, c)),
        _ => Err("Missing one or more values".to_string()),
    }
}
