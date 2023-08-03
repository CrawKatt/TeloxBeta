use crate::dependencies::*;

#[derive(Serialize, Deserialize)]
pub struct UserDataSql {
    pub username: Option<String>,
    pub id: UserId,
    pub first_name: String,
    pub last_name: Option<String>,
}

/// # Errors
pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("database.sqlite")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_data (
            id INTEGER PRIMARY KEY,
            username VARCHAR(32),
            first_name VARCHAR(32),
            last_name VARCHAR(32)
        )",
        [],
    )?;

    Ok(conn)
}

// # Errors
// fn list_users_from_database_sql(conn: &Connection) -> Result<Vec<UserDataSql>> {
//
// let mut stmt = conn.prepare("SELECT id, username, first_name, last_name FROM
// user_data")?;
//
// let user_data_iter = stmt.query_map([], |row| {
//
// Ok(UserDataSql {
// id:         UserId(row.get(0)?),
// username:   row.get(1)?,
// first_name: row.get(2)?,
// last_name:  row.get(3)?,
// })
// })?;
//
// let mut user_data_vec = Vec::new();
//
// for user_data_result in user_data_iter {
//
// user_data_vec.push(user_data_result?);
// }
//
// Ok(user_data_vec)
// }

/// # Errors
/// # Panics
/// This code will panic if is not possible to connect to the database or if is not
/// possible to insert the user data
pub fn insert_user_to_sql(msg: &Message) -> ResponseResult<()> {
    let user = msg.from().unwrap();

    let conn = initialize_database().expect("Error al conectar con la Base de Datos");

    let user_id = user.id;
    println!("User ID: {user_id}");

    let username = user.username.clone().unwrap_or_default();
    println!("Username: {username}");

    let first_name = user.first_name.clone();

    let last_name = user.last_name.clone();

    conn.execute(
        "INSERT OR REPLACE INTO user_data (id, username, first_name, last_name) VALUES (?1, ?2, \
         ?3, ?4)",
        (
            Some(format!("{}", &user_id.0)),
            &username,
            first_name,
            last_name,
        ),
    )
    .expect("Error al insertar datos del usuario");

    println!("Inserting user data: {conn:#?}");

    let mut database_id = conn
        .prepare("SELECT id FROM user_data WHERE id = ?1 AND username = ?2")
        .expect("Error al leer datos del usuario");

    let mut rows = database_id
        .query(params![
            Some(format!("{}", &user_id.0)),
            username.split('@').next()
        ])
        .expect("Error al leer datos del usuario");

    let Some(row) = rows.next().expect("Error al leer datos del usuario") else {
        return Ok(())
    };

    let database_id: Option<u64> = row.get(0).expect("Error al leer datos del usuario");

    let database_id = database_id.unwrap_or_default();

    println!("Selecting user data ID:{database_id}");

    Ok(())
}

// # Errors
// fn insert_user_data_sql(
// conn: &Connection,
// user_id: UserId,
// username: Option<String>,
// first_name: String,
// last_name: Option<String>,
// ) -> Result<()> {
//
// conn.execute(
// "INSERT OR IGNORE INTO user_data (id, username, first_name, last_name) VALUES (?1, ?2,
// \ ?3, ?4)",
// (
// Some(format!("{}", &user_id.0)),
// username,
// first_name,
// last_name,
// ),
// )?;
//
// println!("Inserting user data: {conn:#?}");
//
// Ok(())
// }
//
//
// # Errors
// pub fn update_user_data_sql(
// conn: &Connection,
// user_id: UserId,
// username: Option<String>,
// first_name: Option<String>,
// last_name: Option<String>,
// ) -> Result<()> {
//
// conn.execute(
// "INSERT OR REPLACE INTO user_data (id, username, first_name, last_name) VALUES (?1, ?2,
// \ ?3, ?4)",
// (
// Some(format!("{}", &user_id.0)),
// username,
// first_name,
// last_name,
// ),
// )?;
//
// Ok(())
// }
