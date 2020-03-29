use diesel::PgConnection;
use diesel::RunQueryDsl;
use uuid::Uuid;
// use diesel::QueryDsl;
use chrono::{NaiveDateTime, Utc};
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
    pub external: Uuid,
    pub sender: Uuid,
    pub recipient: Uuid,
    pub issued: i32,
    pub gained: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct NewTransferCommand {
    pub sender: Uuid,
    pub recipient: Uuid,
    pub issued: i32,
    pub gained: i32,
}

fn time_now() -> NaiveDateTime {
    let now = Utc::now();
    now.naive_utc()
}

impl NewTransfer {
    pub fn create(
        cmd: &NewTransferCommand,
        conn: &PgConnection,
    ) -> Result<Transfer, diesel::result::Error> {
        let now = time_now();

        let transfer: NewTransfer = NewTransfer {
            external: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            sender: cmd.sender,
            recipient: cmd.recipient,
            issued: cmd.issued,
            gained: cmd.gained,
        };

        diesel::insert_into(transfers::table)
            .values(transfer)
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
