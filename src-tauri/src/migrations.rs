use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration>{
    let migrations: Vec<Migration> = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE users_twitch (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255), points BIGINT, time_watch_mins BIGINT);",
            kind: MigrationKind::Up,
        }
    ];   
    return migrations;
}