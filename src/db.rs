use std::path::PathBuf;

use crate::{
    light::Light,
    path::{
        bezier::Bezier,
        cubic_bezier::CubicBezier,
        line::{self, Line},
        PathEnum,
    },
};

use std::sync::LazyLock;

use anyhow::{bail, Result};
use include_dir::{include_dir, Dir};
use isx::prelude::IsDefault;
use rusqlite::{params, Connection};
use rusqlite_migration::Migrations;
use vector3d::Vector3d;

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
            .expect("Failed to apply db migrations");

        // Create the object
        Self { connection: conn }
    }

    /// Add a light to the database
    pub fn add_light(&self, light_to_add: &Light) -> Result<()> {
        if light_to_add.is_default() {
            bail!("Light has default values");
        }

        self.connection.execute(
            "INSERT INTO Lights (coordinate_x, coordinate_y, coordinate_z, minimum_beam, maximum_beam, name, address) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                light_to_add.coordinates.x,
                light_to_add.coordinates.y,
                light_to_add.coordinates.z,
                light_to_add.minimum_beam,
                light_to_add.maximum_beam,
                light_to_add.name,
                light_to_add.address,
                ])?;

        Ok(())
    }

    pub fn get_lights(&self) -> Result<Vec<Light>> {
        let mut statement = self.connection.prepare("SELECT * FROM Lights")?;

        let light_iterator = statement.query_map([], |row| {
            Ok(Light {
                coordinates: Vector3d::new(row.get(1)?, row.get(2)?, row.get(3)?),
                minimum_beam: row.get(4)?,
                maximum_beam: row.get(5)?,
                name: row.get(6)?,
                address: row.get(7)?,
            })
        })?;

        Ok(light_iterator.collect::<Result<Vec<_>, _>>()?)
    }

    /// Add a named line to the database
    pub fn add_line(&self, line_to_add: &Line) -> Result<()> {
        if line_to_add.is_default() {
            bail!("Line has default values");
        }

        if line_to_add.name.is_default() {
            bail!("Line has no name");
        }

        self.connection.execute(
            "INSERT INTO Lines (start_x, start_y, start_z, end_x, end_y, end_z, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                line_to_add.start.x,
                line_to_add.start.y,
                line_to_add.start.z,
                line_to_add.end.x,
                line_to_add.end.y,
                line_to_add.end.z,
                line_to_add.name,
            ],
        )?;

        Ok(())
    }

    pub fn get_lines(&self) -> Result<Vec<Line>> {
        let mut statement = self.connection.prepare("SELECT * FROM Lines")?;

        let line_iterator = statement.query_map([], |row| {
            Ok(Line {
                start: Vector3d::new(row.get(1)?, row.get(2)?, row.get(3)?),
                end: Vector3d::new(row.get(4)?, row.get(5)?, row.get(6)?),
                name: row.get(7)?,
            })
        })?;

        Ok(line_iterator.collect::<Result<Vec<_>, _>>()?)
    }

    /// Add a named bezier to the database
    pub fn add_bezier(&self, bezier_to_add: &Bezier) -> Result<()> {
        if bezier_to_add.is_default() {
            bail!("Bezier curve has default values");
        }

        self.connection.execute(
            "INSERT INTO BezierCurves (start_x, start_y, start_z, midpoint_x, midpoint_y, midpoint_z, end_x, end_y, end_z, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                bezier_to_add.start.x,
                bezier_to_add.start.y,
                bezier_to_add.start.z,
                bezier_to_add.midpoint.x,
                bezier_to_add.midpoint.y,
                bezier_to_add.midpoint.z,
                bezier_to_add.end.x,
                bezier_to_add.end.y,
                bezier_to_add.end.z,
                bezier_to_add.name,
            ],
        )?;

        Ok(())
    }

    pub fn get_beziers(&self) -> Result<Vec<Bezier>> {
        let mut statement = self.connection.prepare("SELECT * FROM BezierCurves")?;

        let line_iterator = statement.query_map([], |row| {
            Ok(Bezier {
                start: Vector3d::new(row.get(1)?, row.get(2)?, row.get(3)?),
                midpoint: Vector3d::new(row.get(4)?, row.get(5)?, row.get(6)?),
                end: Vector3d::new(row.get(7)?, row.get(8)?, row.get(9)?),
                name: row.get(10)?,
            })
        })?;

        Ok(line_iterator.collect::<Result<Vec<_>, _>>()?)
    }

    /// Add a named cubic bezier to the database
    pub fn add_cubic_bezier(&self, cubic_bezier_to_add: &CubicBezier) -> Result<()> {
        if cubic_bezier_to_add.is_default() {
            bail!("Cubic bezier curve has default values");
        }

        self.connection.execute(
            "INSERT INTO CubicBezierCurves (start_x, start_y, start_z, end_x, end_y, end_z, handle_1_x, handle_1_y, handle_1_z, handle_2_x, handle_2_y, handle_2_z, name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                cubic_bezier_to_add.start.x,
                cubic_bezier_to_add.start.y,
                cubic_bezier_to_add.start.z,
                cubic_bezier_to_add.end.x,
                cubic_bezier_to_add.end.y,
                cubic_bezier_to_add.end.z,
                cubic_bezier_to_add.handle_1.x,
                cubic_bezier_to_add.handle_1.y,
                cubic_bezier_to_add.handle_1.z,
                cubic_bezier_to_add.handle_2.x,
                cubic_bezier_to_add.handle_2.y,
                cubic_bezier_to_add.handle_2.z,
                cubic_bezier_to_add.name,
            ]
        )?;

        Ok(())
    }

    pub fn get_cubic_beziers(&self) -> Result<Vec<CubicBezier>> {
        let mut statement = self.connection.prepare("SELECT * FROM CubicBezierCurves")?;

        let line_iterator = statement.query_map([], |row| {
            Ok(CubicBezier {
                start: Vector3d::new(row.get(1)?, row.get(2)?, row.get(3)?),
                end: Vector3d::new(row.get(4)?, row.get(5)?, row.get(6)?),
                handle_1: Vector3d::new(row.get(7)?, row.get(8)?, row.get(9)?),
                handle_2: Vector3d::new(row.get(10)?, row.get(11)?, row.get(12)?),
                name: row.get(13)?,
            })
        })?;

        Ok(line_iterator.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn get_paths(&self) -> Result<Vec<PathEnum>> {
        let mut lines: Vec<PathEnum> = self.get_lines()?.into_iter().map(PathEnum::Line).collect();
        let mut bezier_curves: Vec<PathEnum> = self
            .get_beziers()?
            .into_iter()
            .map(PathEnum::Bezier)
            .collect();
        let mut cubic_bezier_curves: Vec<PathEnum> = self
            .get_cubic_beziers()?
            .into_iter()
            .map(PathEnum::CubicBezier)
            .collect();

        let mut out_vec: Vec<PathEnum> = vec![];
        out_vec.append(&mut lines);
        out_vec.append(&mut bezier_curves);
        out_vec.append(&mut cubic_bezier_curves);

        Ok(out_vec)
    }
}
