use diesel::prelude::*;
use crate::schema::{big_proofs, small_proofs};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable)]
#[diesel(table_name = big_proofs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BigProof {
    pub id: i64,
    pub project_id: String,
    pub task_id: String,
    pub status: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Queryable)]
#[diesel(table_name = small_proofs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct SmallProof {
    pub id: i64,
    pub project_id: String,
    pub task_id: String,
    pub task_split_id: String,
    pub task_percentage: f64,
    pub status: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = big_proofs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewBigProof {
    pub project_id: String,
    pub task_id: String,
    pub status: String,
}


#[derive(Insertable, Debug, Serialize, Deserialize)]
#[diesel(table_name = small_proofs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSmallProof {
    pub project_id: String,
    pub task_id: String,
    pub task_split_id: String,
    pub task_percentage: f64,
    pub status: String,
}