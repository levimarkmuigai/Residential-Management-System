use std::error::Error;

use crate::{entities::notice::NoticeDto, user::fields::Id};

pub fn dto(body: String) -> Result<NoticeDto, Box<dyn Error>> {
    let mut date = String::new();

    if let Some((key, value)) = body.split_once("=") {
        match key {
            "exit_date" => {
                date = value.to_string();
            }
            _ => {}
        }
    }

    if date.is_empty() {
        return Err("Missing exit_date in request body".into());
    }

    let id = Id::new();
    let notice_dto = NoticeDto::new(id, date);

    Ok(notice_dto)
}
