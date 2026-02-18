use chrono::{NaiveDate, Utc};
use uuid::Uuid;

use crate::{db, server::server::SessionStore, user::fields::Id};

#[derive(Debug, Clone, PartialEq)]
pub struct Notice {
    pub id: Id,
    pub tenant_id: Id,
    pub date: NaiveDate,
    pub status: String,
}

impl Notice {
    pub fn new(id: Id, tenant_id: Uuid, date: NaiveDate) -> Self {
        Self {
            id,
            tenant_id: Id::from(tenant_id),
            date,
            status: "pending".to_string(),
        }
    }

    pub fn approve_notice(&mut self) {
        self.status = "approved".to_string();
    }

    pub fn reject_notice(&mut self) {
        self.status = "rejected".to_string();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoticeDto {
    pub id: Id,
    pub session_id: Option<String>,
    pub date: String,
}

impl NoticeDto {
    pub fn new(id: Uuid, date: String) -> Self {
        Self {
            id: Id::from(id),
            session_id: None,
            date,
        }
    }
}

pub fn send_notice(notice_dto: NoticeDto, sessions: &SessionStore) -> Result<String, String> {
    let id = notice_dto.id;

    let date_as_string = notice_dto.date;

    let session_id = notice_dto.session_id.ok_or("Session ID not found")?;

    let tenant_uuid: Uuid = {
        let lock = sessions.lock().unwrap();

        lock.get(&session_id)
            .cloned()
            .ok_or("Session Id invalid or expired")?
    };

    let date = NaiveDate::parse_from_str(&date_as_string, "%Y-%m-%d").map_err(|e| e.to_string())?;

    let today = Utc::now().date_naive();
    let notice_period = chrono::Duration::days(30);

    if date < today + notice_period {
        return Err("Vacation date should be at least 30 days from today".to_string());
    }

    let notice = Notice::new(id, tenant_uuid, date);

    println!("Build the notice struct: {:?}", notice);

    db::tenant::send_notice(notice)?;

    println!("Successfully added the notice to DB");

    let status_line = "HTTP/1.1 303 See Other";

    let location = "Location: /tenant";

    let response = format!(
        "{}\r\n{}\r\n
        Content-length: 0\r\nConnection: close\r\n\r\n",
        status_line, location
    );

    Ok(response)
}
