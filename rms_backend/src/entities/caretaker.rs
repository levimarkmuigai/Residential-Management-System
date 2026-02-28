use std::error::Error;

use chrono::NaiveDate;
use uuid::Uuid;

use crate::{
    db,
    entities::{notice::CaretakerNotice, request::CaretakerTask},
    server::server::SessionStore,
};

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

pub fn build_dashboard(
    caretaker_id: Uuid,
) -> Result<(String, String, String, String), Box<dyn Error>> {
    let building_id = db::caretaker::get_building(caretaker_id)?;

    let tenants = db::tenant::get_unassigned()?;

    let assign_units_div = assign_units(building_id, tenants)?;

    let notices_html = match db::caretaker::get_notices(caretaker_id) {
        Ok(caretaker_notices) => get_notices_html(caretaker_notices)?,
        Err(e) => {
            eprintln!("FATAL ERROR {:?}", e);

            return Err(e);
        }
    };

    let urgent_html = match db::caretaker::get_urgent_tasks(caretaker_id) {
        Ok(urgent_tasks) => get_urgent_html(urgent_tasks)?,
        Err(e) => {
            eprintln!("FATAL ERROR {:?}", e);

            return Err(e);
        }
    };

    let tasks_html = match db::caretaker::get_other_tasks(caretaker_id) {
        Ok(tasks) => get_tasks_html(tasks)?,
        Err(e) => {
            eprintln!("FATAL ERROR {:?}", e);

            return Err(e);
        }
    };

    Ok((assign_units_div, notices_html, urgent_html, tasks_html))
}

fn get_tasks_html(tasks: Vec<CaretakerTask>) -> Result<String, Box<dyn Error>> {
    let mut tasks_html = String::new();

    for t in tasks {
        let html = format!(
            r#"
            <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>
            <form action="/fix_request" method="POST">
            <input type="hidden" name="id" value="{}">
            <button type="submit" class="fix-btn">
            FIXED
            </button>
            </form>
            </td>
            </tr>
            "#,
            t.unit_no, t.tenant_name, t.description, t.timestamp, t.id
        );

        println!("DEBUG: TASKS ARE --> {:?}", t);

        tasks_html.push_str(&html);
    }

    Ok(tasks_html)
}

fn get_urgent_html(urgent_tasks: Vec<CaretakerTask>) -> Result<String, Box<dyn Error>> {
    let mut urgent_html = String::new();

    for u in urgent_tasks {
        let html = format!(
            r#"
            <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td>
            <form action="/fix_request" method="POST">
            <input type="hidden" name="id" value="{}">
            <button type="submit" class="fix-btn">
            FIXED
            </button>
            </form>
            </td>
            </tr>
            "#,
            u.unit_no, u.tenant_name, u.description, u.timestamp, u.id
        );

        println!("DEBUG: URGENT TASKS ARE ---> {:?}", u);

        urgent_html.push_str(&html);
    }

    Ok(urgent_html)
}

fn get_notices_html(caretaker_notices: Vec<CaretakerNotice>) -> Result<String, Box<dyn Error>> {
    let mut notices_html = String::new();

    for n in caretaker_notices {
        let output = format!(
            r#"
        <tr>
        <td>{}</td>
        <td>{}</td>
        <td>{}</td>
        <td>
        <div class="notice-btn">
        <form action="/notice/approve" method="POST">
        <input type="hidden" name="id" value="{}">
        <button type="submit" class="btn-blue small-btn">
        APPROVE
        </button>
        </form>
        <form action="/notice/reject" method="POST">
        <input type="hidden" name="id" value="{}">
        <button type="submit" class="btn-red small-btn">
        REJECT
        </button>
        </form>

        </div>
        </td>
        </tr>
        "#,
            n.tenant_name, n.unit_no, n.notice_date, n.id, n.id
        );

        println!("DEBUG: NOTICES ARE --> {:?}", n);
        notices_html.push_str(&output);
    }

    Ok(notices_html)
}

fn assign_units(building_id: Uuid, tenants: Vec<(Uuid, String)>) -> Result<String, Box<dyn Error>> {
    let vacant_units = db::building::get_vacancies(building_id)?;

    let mut tenant_options = String::new();

    for (id, name) in &tenants {
        tenant_options.push_str(&format!(r#"<option value="{}">{}</option>""#, id, name));
    }

    let mut unit_options = String::new();

    for (id, no) in &vacant_units {
        unit_options.push_str(&format!(r#"<option value="{}">{}</option>"#, id, no));
    }

    let output = format!(
        r#"
    "<div class="field">
    <label>AVAILABLE TENANTS</label>
    <select name="tenant_id">
    <option value="" disabled selected>-- Select Tenant --</option>
    {tenant_list}
    </select>
    </div>

    <div class="field">
    <label>VACANT UNITS (IN YOUR BUILDING)</label>
    <select name="unit_id">
    <option value="" disabled selected>-- Select Unit --</option>
    {unit_list}
    </select>
    </div>
    "#,
        tenant_list = tenant_options,
        unit_list = unit_options
    );
    Ok(output)
}

pub fn update_table(dto: CaretakerDto, sessions: &SessionStore) -> Result<String, String> {
    let sid = dto.session_id.ok_or("Session ID Not Found")?;

    let user_id = {
        let lock = sessions.lock().unwrap();
        lock.get(&sid).cloned().ok_or("Invalid Session Id")?
    };

    let national_id = dto.national_id;

    let hired_at = dto.hired_at;

    let details = Details::new(user_id, national_id, hired_at);

    db::caretaker::save_details(details)?;

    println!("DEBUG: *** DB UPDATED SUCCESSFULLY ***");

    let status = "HTTP/1.1 303 See Other";

    let location = "Location: /caretaker";

    let response = format!("{}\r\n{}\r\n\r\n", status, location);

    Ok(response)
}
