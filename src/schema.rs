// @generated automatically by Diesel CLI.

diesel::table! {
    big_proofs (id) {
        id -> Int8,
        project_id -> Varchar,
        task_id -> Varchar,
        status -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::table! {
    small_proofs (id) {
        id -> Int8,
        project_id -> Varchar,
        task_id -> Varchar,
        task_split_id -> Varchar,
        task_percentage -> Float8,
        status -> Varchar,
        create_time -> Timestamp,
        update_time -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    big_proofs,
    small_proofs,
);
