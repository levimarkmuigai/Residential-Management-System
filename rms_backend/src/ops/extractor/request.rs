use crate::{entities::request::RequestDto, user::fields::Id};
use std::error::Error;

pub fn dto(body: String) -> Result<RequestDto, Box<dyn Error>> {
    let mut issue_type = String::new();
    let mut description = String::new();

    for pair in body.split("&") {
        if let Some((key, value)) = pair.split_once("=") {
            match key {
                "issue_type" => {
                    issue_type = value.to_string().replace("+", " ");
                }
                "desc" => {
                    description = value.to_string().replace("+", " ");
                }
                _ => {}
            }
        }
    }

    if issue_type.is_empty() || description.is_empty() {
        return Err("Missing required fields: issue_type or desc".into());
    }

    let id = Id::new();
    let request = RequestDto::new(id, issue_type, description);

    Ok(request)
}
