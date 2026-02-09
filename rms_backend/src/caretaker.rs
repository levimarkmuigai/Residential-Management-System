use chrono::NaiveDate;
use uuid::Uuid;

use crate::{db, server::SessionStore};

#[derive(Debug, Clone, PartialEq)]
pub struct CaretakerDto {
    pub national_id: String,
    pub hired_at: NaiveDate,
    pub session_id: Option<String>,
}

impl CaretakerDto {
    pub fn new(national_id: String, hired_at: String) -> Result<Self, String> {
        let date = NaiveDate::parse_from_str(&hired_at, "%Y-%m-%d").map_err(|e| e.to_string())?;

        Ok(Self {
            national_id,
            hired_at: date,
            session_id: None,
        })
    }
}

pub struct Details {
    pub user_id: Uuid,
    pub national_id: String,
    pub hired_at: NaiveDate,
}

impl Details {
    pub fn new(user_id: Uuid, national_id: String, hired_at: NaiveDate) -> Self {
        Self {
            user_id,
            national_id,
            hired_at,
        }
    }
}

pub fn build_dashboard(caretaker_id: Uuid) -> Result<String, String> {
    let building_id = db::get_caretakers_building(caretaker_id)?;

    let tenants = db::get_unassigned_tenant()?;

    println!("DEBUG: Building Id and Tenants Extracted");

    let mut tenant_option = String::from("<option value=''>--Select Tenant --</option>");

    for (id, name) in tenants {
        tenant_option.push_str(&format!("<option value='{}'>{}</option>", id, name));
    }

    let vacant_units = db::get_vacancies(building_id)?;

    println!("DEBUG: VACANT UNITS FETCHED");

    let mut unit_options = String::from("<option value=''>--Select Unit --</option>");

    for (id, number) in vacant_units {
        unit_options.push_str(&format!("<option value='{}'>{}</option>", id, number));
    }

    let mut html = std::fs::read_to_string("caretaker.html").map_err(|e| e.to_string())?;

    html = html.replace("{{TENANT_OPTIONS}}", &tenant_option);
    html = html.replace("{{VACANT_UNITS}}", &unit_options);

    Ok(html)
}

pub fn update_table(dto: CaretakerDto, sessions: SessionStore) -> Result<String, String> {
    let sid = dto.session_id.ok_or("Session ID Not Found")?;

    let user_id = {
        let lock = sessions.lock().unwrap();
        lock.get(&sid).cloned().ok_or("Invalid Session Id")?
    };

    let national_id = dto.national_id;

    let hired_at = dto.hired_at;

    let details = Details::new(user_id, national_id, hired_at);

    db::handle_caretaker_table(details)?;

    println!("DEBUG: *** DB UPDATED SUCCESSFULLY ***");

    let status = "HTTP/1.1 303 See Other";

    let location = "Location: /caretaker";

    let response = format!("{}\r\n{}\r\n\r\n", status, location);

    Ok(response)
}
