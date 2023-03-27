use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "users")] // singular 'user' is a keyword..
pub struct User {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
}

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "plantdata")]
pub struct PlantData {
    pub plant_id: i32,
    pub created_at: String,
    pub updated_at: String,
    pub planned_data: i32,
    pub unplanned_data: i32,
}