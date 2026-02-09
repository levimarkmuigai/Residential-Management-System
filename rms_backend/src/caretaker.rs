use uuid::Uuid;

use crate::db;

pub fn build_dashboard(caretaker_id: Uuid) -> Result<String,String> {
    
    let building_id = db::get_caretakers_building(caretaker_id)?;

    let tenants = db::get_unassigned_tenant()?;

    println!("DEBUG: Building Id and Tenants Extracted");

    let mut tenant_option = String::from
        ("<option value=''>--Select Tenant --</option>");

    for (id, name) in tenants {
        tenant_option.push_str(&format!(
                "<option value='{}'>{}</option>", id,name));
    }

    let vacant_units = db::get_vacancies(building_id)?;

    println!("DEBUG: VACANT UNITS FETCHED");
    
    let mut unit_options = String::from
        ("<option value=''>--Select Unit --</option>");
    
    for (id, number) in vacant_units {
        unit_options.push_str(&format!(
                "<option value='{}'>{}</option>", id, number));
    }

    let mut html = std::fs::read_to_string("caretaker.html")
        .map_err(|e| e.to_string())?;

    html = html.replace("{{TENANT_OPTIONS}}", &tenant_option);
    html = html.replace("{{VACANT_UNITS}}", &unit_options);

    Ok(html)
}

