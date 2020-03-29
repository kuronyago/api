use diesel::PgConnection;
use diesel::RunQueryDsl;
use uuid::Uuid;
// use diesel::QueryDsl;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
// use chrono::prelude::*;
// use diesel::expression::AsExpression;
// use diesel::types::NotNull;

use super::schema::transfers;
// use super::schema::transfers::dsl;
// use super::schema::transfers::dsl::*;

#[derive(Insertable, Deserialize)]
#[table_name = "transfers"]
pub struct NewTransfer {
    pub sender: Uuid,
    pub recipient: Uuid,
    pub issued: i32,
    pub gained: i32,
}

impl NewTransfer {
    pub fn create(&self, conn: &PgConnection) -> Result<Transfer, diesel::result::Error> {
        diesel::insert_into(transfers::table)
            .values(self)
            .get_result(conn)
    }
}

#[derive(Queryable, Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Transfer {
    pub id: i32,
    pub external: Uuid,
    pub sender: Uuid,
    pub recipient: Uuid,
    pub issued: i32,
    pub gained: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
