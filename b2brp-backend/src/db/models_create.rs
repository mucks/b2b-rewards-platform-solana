#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::*;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;
#[derive(Serialize, Deserialize, Insertable)]
#[diesel(table_name = User)]
pub struct CreateUser {
    pub email: String,
    pub name: String,
    pub password: String,
}
