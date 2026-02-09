use uuid::Uuid;

use crate::{notice::NoticeDto, request::RequestDto, user::{ 
    fields::Id,
    user::{User,UserCredentials}
}};
use crate::landlord::LandlordDTO;
use crate::building::BuildingDTO;

pub fn extract_data(text: String) -> Result<User, String> {

    let mut first_name_buffer = String::new();
    let mut last_name_buffer = String::new();
    let mut role_buffer = String::new();
    let mut email_buffer = String::new();
    let mut phone_number_buffer = String::new();
    let mut password_buffer = String::new();

    for pair in text.split("&") {

        if let Some((key, value)) = pair.split_once("="){

            match key {
                    "first_name" => {
                        first_name_buffer = value.to_string();
                    }

                    "last_name" => {
                        last_name_buffer = value.to_string();
                    }

                    "role" => {
                        role_buffer = value.to_string();
                    }

                    "email" => {
                        email_buffer = value.to_string().replace("%40", "@");
                    }

                    "phone_number" => {
                        phone_number_buffer = value.to_string();
                    }

                    "password" => {
                        password_buffer = value.to_string();
                    }

                    _ => {}
                }
            } else {
                    return Err("Failed to match keys and values".to_string());
            }

        }

    let user_id = Id::new();

    Ok(User::new(
        user_id,
        first_name_buffer,
        last_name_buffer,
        role_buffer,
        email_buffer,
        phone_number_buffer,
        password_buffer,
    )?)
}


pub fn data_for_auth(text: String) -> Result<UserCredentials, String> {

    let mut email_buffer = String::new();
    let mut password_buffer = String::new();

        for pair in text.split("&") {

            if let Some((key, value)) = pair.split_once("=") {

                match key {
                    "email" => {
                      email_buffer = value.to_string().replace("%40", "@");
                    }

                    "password" => {
                        password_buffer = value.to_string();
                    }

                    _ => {}
                }
            }
        }
    Ok(UserCredentials::new(
            email_buffer,
            password_buffer
    )?)
}

pub fn extract_landlord(text: String) -> Result<LandlordDTO, String> {
    let mut business_name = String::new();

    for pair in text.split("&") {
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

pub fn extract_building(text: String) -> Result<BuildingDTO, String> {

    let mut name = String::new();
    let mut units = String::new();

    for pair in text.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "building" => {
                    name = value.to_string();
                }
                "units" => {
                    units = value.to_string();
                }
                _ => {}
            }
        }
    }

    let number_of_units: i32 = units.parse().unwrap();
    let id = Id::new();

    Ok(BuildingDTO::new(
        id,
        name,
        number_of_units,
    )?)
}


pub fn extract_request(text: String) -> RequestDto {

    let mut issue_type = String::new();
    let mut description = String::new();

    for pair in text.split("&") {
        if let Some((key,value)) =
            pair.split_once("=") {
                match key {
                    "issue_type" => {
                        issue_type = value.to_string()
                            .replace("+", " ");
                    }
                    "desc" => {
                        description = value.to_string();
                    }
                    _ => {}
                }
        }
    }

    let id = Id::new();

     let request = RequestDto::new(
         id,
         issue_type,
         description
         );

     request
}

pub fn extract_notice(text: String) -> NoticeDto {

    let mut date = String::new();

    if let Some((key,value)) = text.split_once("=") {
        match key {
            "exit_date" => {
                date = value.to_string();
            }

            _ => {}
        }
    }

    let id  = Id::new();

    let notice_dto = NoticeDto::new(id, date);

    notice_dto
}

pub fn extract_tenant_assign(text: String) -> Result<(Uuid,Uuid), String> {

    let mut unit_id = None;
    let mut tenant_id = None;

    for pair in text.split("&") {
        if let Some((key,value)) = pair
            .split_once("=") { 
                match key {
                    "unit_id" => {
                        unit_id = Some(Uuid::parse_str(value)
                            .map_err(|_| "Invalid unit Uuid")?);
                    }

                    "tenant_id" => {
                        tenant_id = Some(Uuid::parse_str(value)
                            .map_err(|_| "Invalid tenant Uuid")?);
                    }

                    _ => {}
            }
        }
    }

    match (unit_id, tenant_id) {
        (Some(u), Some(t)) => Ok((u,t)),
        _ => Err("Missing on or more values".to_string()),
    }
}

pub fn extract_caretaker_assign(text: String) -> Result<(Uuid,Uuid),String> {

    let mut building_id = None;
    let mut caretaker_id = None;

    for pair in text.split("&") {
        if let Some((key,value)) = 
            pair.split_once("=") {
                match key {
                    "building_id" => {
                        building_id = Some(Uuid::parse_str(value)
                            .map_err(|_| "Invalid building id")?);
                    }

                    "caretaker_id" => {
                        caretaker_id = Some(Uuid::parse_str(value)
                            .map_err(|_| "Invalid caretaker id")?);
                    }

                    _ => {}
                }
        }
    }

    match (building_id, caretaker_id) {
        (Some(b), Some(c)) => Ok((b,c)),
        _ => Err("Missing one or more values".to_string()),
    }
}

