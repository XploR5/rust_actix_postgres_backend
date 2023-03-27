use actix_web::{web, HttpResponse};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    errors::MyError,
    models::{PlantData, User},
};

pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
    let _stmt = include_str!("../../sql/add_user.sql");
    let _stmt = _stmt.replace("$table_fields", &User::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &user_info.email,
                &user_info.first_name,
                &user_info.last_name,
                &user_info.username,
            ],
        )
        .await?
        .iter()
        .map(|row| User::from_row_ref(row).unwrap())
        .collect::<Vec<User>>()
        .pop()
        .ok_or(MyError::NotFound) // more applicable for SELECTs
}

pub async fn add_plant_data(
    client: web::Data<Client>,
    plant_data: web::Json<PlantData>,
) -> HttpResponse {
    let _stmt = include_str!("../../sql/add_plantdata.sql");
    let _stmt = _stmt.replace("$table_fields", &PlantData::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    match client
        .execute(
            &stmt,
            &[
                &(plant_data.plant_id),
                &(plant_data.created_at),
                &(plant_data.updated_at),
                &(plant_data.planned_data),
                &(plant_data.unplanned_data),
            ],
        )
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
