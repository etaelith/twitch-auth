use rusqlite::{params, Connection, Result};

use crate::data_structs::ResponseStatus;

pub fn insert_twitch(
    user_id: i64,
    twitch_connection_name: String,
) -> Result<ResponseStatus, rusqlite::Error> {
    match connect_database() {
        Ok(conn) => {
            let _ = insert_twitch_tag(&conn, user_id.clone(), &twitch_connection_name);
            Ok(ResponseStatus {
                success: true,
                success_description: Some("User and Twitch tag inserted successfully".to_string()),
                error_message: None,
            })
        }
        Err(conn_err) => {
            eprintln!("Error connecting db: {}", conn_err);
            Err(conn_err)
        }
    }
}

pub fn connect_database() -> Result<Connection> {
    match Connection::open(std::env::var("DB_PATH").expect("missing DB_PATH")) {
        Ok(conn) => Ok(conn),
        Err(err) => {
            eprintln!("Error al abrir la conextion a la base de datos: {:?}", err);
            Err(err)
        }
    }
}

pub fn insert_twitch_tag(
    conn: &Connection,
    id_user: i64,
    user_twitch: &str,
) -> Result<ResponseStatus, rusqlite::Error> {
    match conn.execute(
        "UPDATE users SET user_twitch = ?1 WHERE user_id = ?2",
        params![user_twitch, id_user],
    ) {
        Ok(_) => Ok(ResponseStatus {
            success: true,
            success_description: Some(format!("Agreed {}", user_twitch)),
            error_message: None,
        }),
        Err(err) => Err(err.into()),
    }
}
