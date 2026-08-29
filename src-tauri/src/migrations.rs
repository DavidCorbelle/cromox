use tauri_plugin_sql::{Migration, MigrationKind};

pub fn get_migrations() -> Vec<Migration>{
    let migrations: Vec<Migration> = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE users_twitch (id VARCHAR(255) PRIMARY KEY, name VARCHAR(255), points BIGINT, time_watch_mins BIGINT);",
            kind: MigrationKind::Up,
        },
        Migration {
            version: 2, 
            description: "create_table_commands",
            sql:"CREATE TABLE commands_twitch ( id INTEGER, command_name TEXT NOT NULL UNIQUE, trigger TEXT NOT NULL UNIQUE, content_type TEXT NOT NULL, response_text TEXT, sound TEXT, permits TEXT, cooldown TEXT, integration TEXT, point_cost INTEGER, enabled INTEGER DEFAULT 1, PRIMARY KEY(id AUTOINCREMENT));",
            kind: MigrationKind::Up
        },
        Migration {
            version: 3, 
            description: "create_table_tokens",
            sql:"CREATE TABLE tokens_bot ( type_token TEXT, refresh_token TEXT, expires_in NUMERIC, expire_date TEXT, user_id TEXT) ;",
            kind: MigrationKind::Up
        }
    ];   
    return migrations;
}