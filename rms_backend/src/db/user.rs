use crate::{
    db::utils::get_client,
    user::{
        fields::{Email, Id, Name, Password, PhoneNumber, Role},
        user::User,
    },
};

pub fn save(user: User) -> Result<(), String> {
    let mut client = get_client().map_err(|e| e.to_string())?;

    let mut transaction = client.transaction().map_err(|e| e.to_string())?;

    let sql_statement = "
    INSERT INTO users (id, role, first_name, last_name,
        email, phone_number, password_hash)
        VALUES($1, $2, $3, $4, $5, $6, $7)";

    transaction
        .execute(
            sql_statement,
            &[
                &user.id.value(),
                &user.role.value(),
                &user.first_name.value(),
                &user.last_name.value(),
                &user.email.value(),
                &user.phone_number.value(),
                &user.password.value(),
            ],
        )
        .map_err(|e| e.to_string())?;

    let role = user.role.value();
    let id = user.id.value();

    match role {
        "Landlord" => transaction.execute(
            "
                INSERT INTO landlords(user_id, business_name) VALUES($1)",
            &[&id, &"Pending Update"],
        ),

        "Caretaker" => transaction.execute(
            "
                INSERT INTO caretakers(user_id, national_id) VALUES($1,$2)",
            &[&id, &format!("TEMP-{}", id.to_string()[..8].to_uppercase())],
        ),

        "Tenant" => transaction.execute(
            "
                INSERT INTO tenants(user_id, payment_status) VALUES($1, $2)",
            &[&id, &"up-to-date"],
        ),

        _ => Ok(0),
    }
    .map_err(|e| {
        eprint!("CRITICLA DATABASE ERROR: {:?}", e);
        format!("Role initialization failed: {}", e)
    })?;

    transaction.commit().map_err(|e| e.to_string())?;

    Ok(())
}

pub fn find_by_email(email: &str) -> Option<User> {
    let mut client = get_client().expect("Failed to connect to DB");

    let sql_statement = "
        SELECT id,role,first_name, last_name,email,phone_number,password_hash 
        FROM users WHERE email = $1";

    let result = client
        .query_opt(sql_statement, &[&email])
        .expect("Failed to fetch user.");

    result.map(|row| User {
        id: Id { id: row.get("id") },
        role: Role {
            raw: row.get("role"),
        },
        first_name: Name {
            raw: row.get("first_name"),
        },
        last_name: Name {
            raw: row.get("last_name"),
        },
        email: Email {
            raw: row.get("email"),
        },
        phone_number: PhoneNumber {
            raw: row.get("phone_number"),
        },
        password: Password {
            raw: row.get("password_hash"),
        },
    })
}
