use actix_web::{web, Error, HttpResponse};
use chrono::Utc;
use deadpool_postgres::{Client, Pool};
use rand::Rng;
use tokio_pg_mapper::FromTokioPostgresRow;

use crate::{
    db,
    errors::MyError,
    models::{PlantData, User},
};

pub async fn add_user(
    user: web::Json<User>,
    db_pool: web::Data<Pool>,
) -> Result<HttpResponse, Error> {
    let user_info: User = user.into_inner();

    let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

    let new_user = db::add_user(&client, user_info).await?;

    Ok(HttpResponse::Ok().json(new_user))
}

pub async fn add_plant_data(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let handles = (0..100)
        .map(|_| {
            let db_pool = db_pool.clone();
            tokio::spawn(async move {
                let plant_data = PlantData {
                    plant_id: rand::thread_rng().gen_range(1..10),
                    created_at: Utc::now().to_rfc3339(),
                    updated_at: Utc::now().to_rfc3339(),
                    planned_data: rand::thread_rng().gen_range(1..101),
                    unplanned_data: rand::thread_rng().gen_range(1..101),
                };
                let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
                db::add_plant_data(web::Data::new(client), actix_web::web::Json(plant_data)).await;
                Ok::<_, MyError>(())
            })
        })
        .collect::<Vec<_>>();

    // Wait for all the threads to complete
    for handle in handles {
        handle.await;
    }

    Ok(HttpResponse::Ok().json({}))
}

pub async fn get_plant_data(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client = db_pool.get().await.map_err(MyError::PoolError)?;
    let stmt = client
        .prepare("SELECT * FROM testing.plantdata ORDER BY created_at DESC;")
        .await
        .unwrap();
    let plant_data = client
        .query(&stmt, &[])
        .await
        .expect("Error retrieving plant data")
        .iter()
        .map(|row| PlantData::from_row_ref(row).unwrap())
        .collect::<Vec<PlantData>>();

    Ok(HttpResponse::Ok().json(plant_data))
}