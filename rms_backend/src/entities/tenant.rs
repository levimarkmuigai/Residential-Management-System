use std::error::Error;

use uuid::Uuid;

use crate::{
    db::tenant,
    entities::{notice::DisplayNotice, request::DbRequest},
    user::fields::Id,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Tenant {
    id: Id,
    payment_status: bool,
}

impl Tenant {
    pub fn new(id: Uuid, payment_status: bool) -> Self {
        Self {
            id: Id::from(id),
            payment_status,
        }
    }
}

pub fn dash_display(user_id: Uuid) -> Result<(String, String), Box<dyn Error>> {
    let db_request = match tenant::get_requests(user_id) {
        Ok(request) => request,
        Err(e) => {
            eprint!("ERROR GETTING DBREQUEST --> {:?}", e);
            return Err(e);
        }
    };

    let display_option = match tenant::get_notice(user_id) {
        Ok(option) => option,
        Err(e) => {
            eprint!("ERROR GETTING NOTICE --> {:?}", e);
            return Err(e);
        }
    };

    let request_rows = request_display(db_request)?;

    let notice_display = notice_display(display_option)?;

    Ok((request_rows, notice_display))
}

fn request_display(db_request: Vec<DbRequest>) -> Result<String, Box<dyn Error>> {
    let mut request_rows = String::new();

    for r in db_request {
        let issue_type = &r.issue_type;

        let description = &r.description;

        let submitted_at = &r.submitted_at.format("%m-%d %H:%M").to_string();

        let status = &r.status;

        println!(
            "DEBUG: {},{},{},{}",
            issue_type, description, submitted_at, status
        );

        let status_color = match status.as_str() {
            "pending" => "yellow",
            "resolved" => "green",
            _ => "yellow",
        };

        let output = format!(
            r#"
            <tr>
            <td>{}</td>
            <td>{}</td>
            <td>{}</td>
            <td class="status-cell">
            <div class="status-dot {}"></div>
            <span>{}</span>
            </td>
            </tr>
            "#,
            issue_type.to_uppercase(),
            description.to_uppercase(),
            submitted_at.to_uppercase(),
            status_color,
            status.to_uppercase()
        );

        request_rows.push_str(&output);
    }

    Ok(request_rows)
}

fn notice_display(display_option: Option<DisplayNotice>) -> Result<String, Box<dyn Error>> {
    let notice = match display_option {
        Some(n) => n,
        None => {
            return Ok(r#"
            <div class="stat-item notice">
                    <div class="label">NOTICE: <span class="value">--</span></div>
                </div>"#
                .to_string());
        }
    };

    let status = &notice.status;

    let status_color = match status.as_str() {
        "approved" => "green",
        "reject" => "yellow",
        _ => "",
    };

    let notice_display = format!(
        r#"
    <div class="stat-item notice">
    <div class="status-dot {}"></div>
    <div class="label">NOTICE: 
    <span class="value">{}</span>
    </div>
    </div>"#,
        status_color, status
    );

    Ok(notice_display)
}
