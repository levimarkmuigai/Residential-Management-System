use uuid::Uuid;

use crate::{db, server::SessionStore, user::fields::Id};

pub struct Request {
    pub id: Id,
    pub tenant_id: Id,
    pub issue_type: String,
    pub description: String,
    pub priority: String,
    pub status: String,
}

impl Request {

    pub fn new(id: Uuid, issue_type: String, 
        description: String, tenant_id: Uuid) -> Self {

            let priority = match issue_type.to_lowercase().as_str() {
                "power" | "water" => "high".to_string(),
                _ => "medium".to_string(),
            };

            Self {
                id: Id::from(id),
                tenant_id:  Id::from(tenant_id),
                issue_type: issue_type,
                description: description,
                priority: priority,
                status: "pending".to_string(),
            }
    }

    pub fn mark_resolved(&mut self) {
        self.status = "resolved".to_string();
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestDto {
    pub id: Id,
    pub issue_type: String,
    pub desc: String,
    pub session_id: Option<String>,
}

impl RequestDto {
    pub fn new(id: Uuid, issue_type: String, desc: String) -> Self {
        Self {
            id: Id::from(id),
            issue_type: issue_type,
            desc: desc,
            session_id: None,
        }
    }
}

pub fn send_request(request_dto: RequestDto, sessions: &SessionStore) 
    -> Result<String, String> {

        let session_id = request_dto.session_id
            .ok_or("Session Id not found")?;

        let tenant_uuid = {
            let lock = sessions
                .lock()
                .unwrap();
            lock.get(&session_id).cloned()
                .ok_or("Invalid session id")?
        };

        let id = *request_dto.id.value();

        let issue_type = request_dto.issue_type;

        let desc = request_dto.desc;

        let request = 
            Request::new(id, issue_type, desc, tenant_uuid);

        println!("Attempting to insert into database...");

        db::insert_request(request)?;

        println!("Request sent to Database");

        let status_line = "HTTP/1.1 303 See Other";

        let location = "Location: /tenant";

        let response = format!("{}\r\n{}\r\nContent-length: 0
            \r\nConnection: close\r\n\r\n", status_line, location
            );

        Ok(response)
}

