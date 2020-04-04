use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use diesel::PgConnection;
use diesel::RunQueryDsl;

use super::commands::NewProject as NewProjectCmd;
use super::schema::projects;

#[derive(Insertable, Deserialize)]
#[table_name = "projects"]
pub struct NewProject {
    pub external: Uuid,
    pub creator: Uuid,
    pub free: i32,
    pub frozen: i32,
    pub spent: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

fn time_now() -> NaiveDateTime {
    let now = Utc::now();
    now.naive_utc()
}

pub fn create_project(cmd: &NewProjectCmd, conn: &PgConnection) -> Result<Project, diesel::result::Error> {
    Ok(())
}

#[derive(Queryable, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Project {
    pub id: i32,
    pub external: Uuid,
    pub creator: Uuid,
    pub free: i32,
    pub frozen: i32,
    pub spent: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}