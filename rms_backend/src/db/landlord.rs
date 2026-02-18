use uuid::Uuid;

use crate::{
    db::utils::get_client,
    entities::{building::Building, landlord::Landlord},
};

pub fn update_profile(landlord: Landlord) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let sql_statement = "INSERT INTO landlords(user_id, business_name)
        VALUES($1, $2)";

    client
        .execute(
            sql_statement,
            &[&landlord.id.value(), &landlord.business_name.value()],
        )
        .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn save_buidling(building: Building) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let mut transaction = client.transaction().map_err(|e| e.to_string())?;

    let sql_building = "
        INSERT INTO buildings(id,name,landlord_id,total_units_count)
       VALUES($1,$2,$3,$4)";

    let _row = transaction
        .execute(
            sql_building,
            &[
                &building.id.value(),
                &building.name.value(),
                &building.landlord_id.value(),
                &building.units,
            ],
        )
        .map_err(|e| e.to_string())?;

    let total_units = building.units;

    let sql_units = "
        INSERT INTO units(id, building_id, unit_number, is_occupied)
        VALUES($1,$2,$3,false)";

    let statement = transaction.prepare(sql_units).map_err(|e| e.to_string())?;

    let building_id = building.id.value();

    for i in 1..=total_units {
        let id = Uuid::new_v4();

        transaction
            .execute(&statement, &[&id, &building_id, &i])
            .map_err(|e| e.to_string())?;
    }

    transaction.commit().map_err(|e| e.to_string())?;
    Ok(())
}
