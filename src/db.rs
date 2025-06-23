use std::path::PathBuf;

use crate::light::{self, Light};

use std::sync::LazyLock;

use anyhow::Result;
use include_dir::{include_dir, Dir};
use rusqlite::{params, Connection};
use rusqlite_migration::Migrations;

static MIGRATIONS_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/migrations");

// Define migrations. These are applied atomically.
static MIGRATIONS: LazyLock<Migrations<'static>> = LazyLock::new(|| {
    Migrations::from_directory(&MIGRATIONS_DIR).expect("Failed to load Database migrations")
});

pub struct Database {
    connection: Connection,
}

impl Database {
    /// Initialize the database, uses the migrations in ../migrations/
    pub fn new(file_name: PathBuf) -> Self {
        let mut conn = Connection::open(file_name).expect("Failed to open Database file");

        // Update the database schema
        MIGRATIONS
            .to_latest(&mut conn)
            .expect("Failed to update db schema");

        // Create the object
        Self { connection: conn }
    }

    /// Add a light to the database
    pub fn add_light(&self, light_to_add: Light) -> Result<()> {
        let sql_result = self.connection.execute(
            "INSERT INTO Lights (coordinate_x, coordinate_y, coordinate_z, minimum_beam, maximum_beam, name, address) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                light_to_add.coordinates.x,
                light_to_add.coordinates.y,
                light_to_add.coordinates.z,
                light_to_add.minimum_beam,
                light_to_add.maximum_beam,
                light_to_add.name,
                light_to_add.address,
                ]);

        // Error handling
        match sql_result {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
