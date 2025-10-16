use std::path::Path;

use anyhow::{Result, Context};
use rusqlite::Connection;
use dioxus_logger::tracing::*;
pub struct SqliteDatabase {
    connection: Connection
}

impl SqliteDatabase {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        dioxus_logger::tracing::info!("New sql db");
        let connection = Connection::open(db_path)
            .context("Failed to open or create database file")?;

        Self::initialize_tables(&connection)
            .context("Failed to initialize database tables")?;
        Ok(Self { connection })
    }                
    
    fn initialize_tables(connection: &Connection) -> rusqlite::Result<()> {
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS workout (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                date DATE NOT NULL
            )",
            []
        ) {
            Ok(updated) => dioxus_logger::tracing::info!("Created table, {} rows were updated", updated),
            Err(err) => dioxus_logger::tracing::error!("did not create table: {}", err),
        } 
        
        match connection.execute(
            "CREATE TABLE IF NOT EXISTS exercise (
                id INTEGER PRIMARY KEY,
                workout_id INTEGER NOT NULL,
                name TEXT NOT NULL,
                sets INTEGER,
                reps INTEGER,
                FOREIGN KEY(workout_id) REFERENCES workout(id)
            )",
            []
        ) {
            Ok(updated) => dioxus_logger::tracing::info!("{} rows were updated", updated),
            Err(err) => dioxus_logger::tracing::error!("update failed: {}", err),
        }

        Ok(())
    }

    pub fn connection(&self) -> &Connection {
        &self.connection
    }
}