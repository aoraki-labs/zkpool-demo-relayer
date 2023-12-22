
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use parking_lot::Mutex;
use std::sync::Arc;
use crate::models::{BigProof, SmallProof,NewBigProof, NewSmallProof};
use crate::schema::{big_proofs,small_proofs};
use diesel::sql_types::{Varchar, Float8};

use lazy_static::lazy_static;

#[derive(QueryableByName)]
struct BigProofStatus {
    #[sql_type = "Varchar"]
    status: String,
}

#[derive(QueryableByName)]
struct SmallProofStatusAndPercentage {
    #[sql_type = "Varchar"]
    status: String,
    #[sql_type = "Float8"]
    task_percentage: f64,
}

type DbPool = Arc<tokio::sync::Mutex<PgConnection>>;

lazy_static! {
    pub static ref DB_POOL: DbPool = {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect("Error connecting to database");
        Arc::new(tokio::sync::Mutex::new(connection))
    };
}

pub async fn add_big_proof(project_id: &str, task_id: &str) -> Result<(), String> {
    let mut conn = DB_POOL.lock().await;

    let new_proof = NewBigProof {
        project_id: project_id.to_owned(),
        task_id: task_id.to_owned(),
        status: "created".to_owned(),
    };

    diesel::insert_into(big_proofs::dsl::big_proofs)
        .values(&new_proof)
        .execute(&mut *conn)
        .map_err(|err| format!("Error adding big proof: {}", err))?;

    Ok(())
}

pub async fn set_big_proof_status(project_id: &str, task_id: &str, status: &str) -> Result<(), String> {
    let mut conn = DB_POOL.lock().await;

    diesel::update(big_proofs::dsl::big_proofs.filter(big_proofs::project_id.eq(project_id).and(big_proofs::task_id.eq(task_id))))
        .set(big_proofs::status.eq(status))
        .execute(&mut *conn)
        .map_err(|err| format!("Error setting big proof status: {}", err))?;

    Ok(())
}

pub async fn add_small_proof(project_id: &str, task_id: &str, split_id: &str) -> Result<(), String> {
    let mut conn = DB_POOL.lock().await;
    let new_proof = NewSmallProof {
        project_id: project_id.to_owned(),
        task_id: task_id.to_owned(),
        task_split_id: split_id.to_owned(),
        task_percentage: 0.0,
        status: "created".to_owned(),
    };

    diesel::insert_into(small_proofs::dsl::small_proofs)
        .values(&new_proof)
        .execute(&mut *conn)
        .map_err(|err| format!("Error adding small proof: {}", err))?;

    Ok(())
}

pub async fn set_small_proof_status_and_percentage(project_id: &str, task_id: &str, split_id: &str, status: &str, percentage: f64) -> Result<(), String> {
    let mut conn = DB_POOL.lock().await;
    diesel::update(small_proofs::dsl::small_proofs.filter(small_proofs::project_id.eq(project_id).and(small_proofs::task_id.eq(task_id)).and(small_proofs::task_split_id.eq(split_id))))
        .set((small_proofs::status.eq(status), small_proofs::task_percentage.eq(percentage)))
        .execute(&mut *conn)
        .map_err(|err| format!("Error setting small proof status and percentage: {}", err))?;

    Ok(())
}

pub async fn get_big_proof_status(project_id: &str, task_id: &str) -> Result<String, String> {
    let mut conn = DB_POOL.lock().await;

    let query = "SELECT status FROM big_proofs WHERE project_id = $1 AND task_id = $2";
    let result = diesel::sql_query(query)
        .bind::<Varchar, _>(project_id)
        .bind::<Varchar, _>(task_id)
        .get_result::<BigProofStatus>(&mut *conn)
        .map_err(|err| format!("Error getting big proof status: {}", err))?;

    Ok(result.status)
}

pub async fn get_small_proof_status_and_percentage(project_id: &str, task_id: &str, split_id: &str) -> Result<(String, f64), String> {
    let mut conn = DB_POOL.lock().await;

    let query = "SELECT status, task_percentage FROM small_proofs WHERE project_id = $1 AND task_id = $2 AND task_split_id = $3";
    let result = diesel::sql_query(query)
        .bind::<Varchar, _>(project_id)
        .bind::<Varchar, _>(task_id)
        .bind::<Varchar, _>(split_id)
        .get_result::<SmallProofStatusAndPercentage>(&mut *conn)
        .map_err(|err| format!("Error getting small proof status and percentage: {}", err))?;

    Ok((result.status, result.task_percentage))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use diesel::prelude::*;
//     fn establish_connection() -> PgConnection {
//         dotenv().ok();
//         let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//         PgConnection::establish(&database_url).expect("Error connecting to database")
//     }

//     #[tokio::test]
//     async fn test_add_big_proof() {
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";

//         // Clean up existing data
//         diesel::delete(big_proofs::dsl::big_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting big proofs");

//         // Add big proof
//         add_big_proof(project_id, task_id).await.expect("Error adding big proof");

//         // Retrieve big proof
//         let result = big_proofs::dsl::big_proofs
//             .filter(big_proofs::project_id.eq(project_id).and(big_proofs::task_id.eq(task_id)))
//             .first::<BigProof>(&mut conn)
//             .expect("Error retrieving big proof");

//         assert_eq!(result.project_id, project_id);
//         assert_eq!(result.task_id, task_id);
//         assert_eq!(result.status, "created");
//     }

//     #[tokio::test]
//     async fn test_set_big_proof_status() {
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";
//         let status = "proving";

//         // Clean up existing data
//         diesel::delete(big_proofs::dsl::big_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting big proofs");

//         // Add big proof
//         diesel::insert_into(big_proofs::dsl::big_proofs)
//             .values((
//                 big_proofs::project_id.eq(project_id),
//                 big_proofs::task_id.eq(task_id),
//                 big_proofs::status.eq("created"),
//             ))
//             .execute(&mut conn)
//             .expect("Error adding big proof");

//         // Set big proof status
//         set_big_proof_status(project_id, task_id, status)
//             .await
//             .expect("Error setting big proof status");

//         // Retrieve big proof
//         let result = big_proofs::dsl::big_proofs
//             .filter(big_proofs::project_id.eq(project_id).and(big_proofs::task_id.eq(task_id)))
//             .first::<BigProof>(&mut conn)
//             .expect("Error retrieving big proof");

//         assert_eq!(result.status, status);
//     }

//     #[tokio::test]
//     async fn test_add_small_proof() {
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";
//         let split_id = "2";

//         // Clean up existing data
//         diesel::delete(small_proofs::dsl::small_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting small proofs");

//         // Add small proof
//         add_small_proof(project_id, task_id, split_id)
//             .await
//             .expect("Error adding small proof");

//         // Retrieve small proof
//         let result = small_proofs::dsl::small_proofs
//             .filter(
//                 small_proofs::project_id
//                     .eq(project_id)
//                     .and(small_proofs::task_id.eq(task_id))
//                     .and(small_proofs::task_split_id.eq(split_id)),
//             )
//             .first::<SmallProof>(&mut conn)
//             .expect("Error retrieving small proof");

//         assert_eq!(result.project_id, project_id);
//         assert_eq!(result.task_id, task_id);
//         assert_eq!(result.task_split_id, split_id);
//         assert_eq!(result.status, "created");
//     }

//     #[tokio::test]
//     async fn test_set_small_proof_status_and_percentage() {
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";
//         let split_id = "2";
//         let status = "proving";
//         let percentage = 0.1;

//         // Clean up existing data
//         diesel::delete(small_proofs::dsl::small_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting small proofs");

//         // Add small proof
//         diesel::insert_into(small_proofs::dsl::small_proofs)
//             .values((
//                 small_proofs::project_id.eq(project_id),
//                 small_proofs::task_id.eq(task_id),
//                 small_proofs::task_split_id.eq(split_id),
//                 small_proofs::task_percentage.eq(0.0),
//                 small_proofs::status.eq("created"),
//             ))
//             .execute(&mut conn)
//             .expect("Error adding small proof");

//         // Set small proof status and percentage
//         set_small_proof_status_and_percentage(project_id, task_id, split_id, status, percentage)
//             .await
//             .expect("Error setting small proof status and percentage");

//         // Retrieve small proof
//         let result = small_proofs::dsl::small_proofs
//             .filter(
//                 small_proofs::project_id
//                     .eq(project_id)
//                     .and(small_proofs::task_id.eq(task_id))
//                     .and(small_proofs::task_split_id.eq(split_id)),
//             )
//             .first::<SmallProof>(&mut conn)
//             .expect("Error retrieving small proof");

//         assert_eq!(result.status, status);
//         assert_eq!(result.task_percentage, percentage);
//     }

//     #[tokio::test]
//     async fn test_get_big_proof_status() -> Result<(), String>{
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";
//         let status = "proving";

//         // Clean up existing data
//         diesel::delete(big_proofs::dsl::big_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting big proofs");

//         // Add big proof
//         diesel::insert_into(big_proofs::dsl::big_proofs)
//             .values((
//                 big_proofs::project_id.eq(project_id),
//                 big_proofs::task_id.eq(task_id),
//                 big_proofs::status.eq(status),
//             ))
//             .execute(&mut conn)
//             .expect("Error adding big proof");

//         // Get big proof status
//         let query = "SELECT status FROM big_proofs WHERE project_id = $1 AND task_id = $2";
//         let result = diesel::sql_query(query)
//             .bind::<Varchar, _>(project_id)
//             .bind::<Varchar, _>(task_id)
//             .get_result::<BigProofStatus>(&mut conn)
//             .map_err(|err| format!("Error getting big proof status: {}", err))?;

//         assert_eq!(result.status, status);
//         Ok(())
//     }

//     #[tokio::test]
//     async fn test_get_small_proof_status_and_percentage() -> Result<(), String> {
//         let mut conn = establish_connection();
//         let project_id = "test";
//         let task_id = "1";
//         let split_id = "2";
//         let status = "proving";
//         let percentage = 0.1;

//         // Clean up existing data
//         diesel::delete(small_proofs::dsl::small_proofs)
//             .execute(&mut conn)
//             .expect("Error deleting small proofs");

//         // Add small proof
//         diesel::insert_into(small_proofs::dsl::small_proofs)
//             .values((
//                 small_proofs::project_id.eq(project_id),
//                 small_proofs::task_id.eq(task_id),
//                 small_proofs::task_split_id.eq(split_id),
//                 small_proofs::task_percentage.eq(percentage),
//                 small_proofs::status.eq(status),
//             ))
//             .execute(&mut conn)
//             .expect("Error adding small proof");

//         // Get small proof status and percentage
//         let query = "SELECT status, task_percentage FROM small_proofs WHERE project_id = $1 AND task_id = $2 AND task_split_id = $3";
//         let result = diesel::sql_query(query)
//             .bind::<Varchar, _>(project_id)
//             .bind::<Varchar, _>(task_id)
//             .bind::<Varchar, _>(split_id)
//             .get_result::<SmallProofStatusAndPercentage>(&mut conn)
//             .map_err(|err| format!("Error getting small proof status and percentage: {}", err))?;

//         assert_eq!(result.status, status);
//         assert_eq!(result.task_percentage, percentage);

//         Ok(())
//     }
// }
