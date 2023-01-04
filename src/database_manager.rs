use anyhow::Result;
use sqlx::{mysql::MySqlPoolOptions, postgres::PgPoolOptions, sqlite::SqlitePoolOptions};
enum DatabaseType {
    MySQL,
    SQLite,
    PostgreSQL,
}

struct DatabaseInfo {
    host: String,
    database_type: DatabaseType,
    database_name: String,
    username: String,
    password: String,
}

impl Default for DatabaseInfo {
    fn default() -> DatabaseInfo {
        DatabaseInfo {
            host: "localhost".to_string(),
            database_type: DatabaseType::MySQL,
            database_name: "default".to_string(),
            username: "root".to_string(),
            password: "".to_string(),
        }
    }
}

fn build_connection_string() -> String {
    let database_info = DatabaseInfo::default();

    let connection_string = match database_info.database_type {
        DatabaseType::MySQL => {
            format!(
                "mysql://{}:{}@{}/{}",
                database_info.username,
                database_info.password,
                database_info.host,
                database_info.database_name
            )
        },
        DatabaseType::SQLite => {
            format!(
                "sqlite://{}",
                database_info.database_name
            )
        },
        DatabaseType::PostgreSQL => {
            format!(
                "postgresql://{}:{}@{}/{}",
                database_info.username,
                database_info.password,
                database_info.host,
                database_info.database_name
            )
        },
    };

    connection_string
}

fn connect_database(database_type: DatabaseType, connection_string: String) -> Result<()> {
    // create pool for databasse type
    



    Ok(())
}