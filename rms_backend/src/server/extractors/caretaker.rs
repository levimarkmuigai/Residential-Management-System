use uuid::Uuid;

use crate::entities::caretaker::CaretakerDto;
use std::error::Error;

pub fn dto(body: String) -> Result<CaretakerDto, Box<dyn Error>> {
    let mut hired_at = String::new();
    let mut national_id = String::new();

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "national_id" => {
                    national_id = value.to_string().replace("+", " ");
                }
                "hired_at" => {
                    hired_at = value.to_string().replace("+", " ");
                }
                _ => {}
            }
        }
    }

    if national_id.is_empty() || hired_at.is_empty() {
        return Err("Missing national_id or hired_at".into());
    }

    Ok(CaretakerDto::new(national_id, hired_at)?)
}

pub fn assignment_params(body: String) -> Result<(Uuid, Uuid), Box<dyn std::error::Error>> {
    let mut unit_id = None;
    let mut tenant_id = None;

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "unit_id" => {
                    unit_id = Some(Uuid::parse_str(value).map_err(|_| "Invalid unit Uuid")?);
                }
                "tenant_id" => {
                    tenant_id = Some(Uuid::parse_str(value).map_err(|_| "Invalid tenant Uuid")?);
                }
                _ => {}
            }
        }
    }

    match (unit_id, tenant_id) {
        (Some(u), Some(t)) => Ok((u, t)),
        _ => Err("Missing one or more values (unit_id or tenant_id)".into()),
    }
}
