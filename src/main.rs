mod config {
    use serde::Deserialize;
    #[derive(Debug, Default, Deserialize)]
    pub struct ExampleConfig {
        pub server_addr: String,
        pub pg: deadpool_postgres::Config,
        pub plantdata_endpoint: String,    
    }
}

mod models {
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


}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use actix_web::{HttpResponse, web};
    use deadpool_postgres::{Client};
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::{User, PlantData}};

    pub async fn add_user(client: &Client, user_info: User) -> Result<User, MyError> {
        let _stmt = include_str!("../sql/add_user.sql");
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


    pub async fn add_plant_data(client: web::Data<Client>,
        plant_data: web::Json<PlantData>,) -> HttpResponse {
        let _stmt = include_str!("../sql/add_plantdata.sql");
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

}

mod handlers {
    use actix_web::{web, Error, HttpResponse};
    use deadpool_postgres::{Client, Pool};
    use rand::Rng;
    use chrono::Utc;

    use crate::{db, errors::MyError, models::{User, PlantData}};

    pub async fn add_user(
        user: web::Json<User>,
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let user_info: User = user.into_inner();

        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let new_user = db::add_user(&client, user_info).await?;

        Ok(HttpResponse::Ok().json(new_user))
    }

    pub async fn add_plant_data(
        db_pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
        let handles = (0..100)
            .map(|_| {
                let db_pool = db_pool.clone();
                tokio::spawn(async move {                    
                    let plant_data = PlantData {
                        plant_id: rand::thread_rng().gen_range(1 .. 10),
                        created_at: Utc::now().to_rfc3339(),
                        updated_at: Utc::now().to_rfc3339(),
                        planned_data: rand::thread_rng().gen_range(1 .. 101),
                        unplanned_data: rand::thread_rng().gen_range(1 .. 101),
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

}

use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::add_user;
use tokio_postgres::NoTls;

use crate::{config::ExampleConfig, handlers::add_plant_data};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/users").route(web::post().to(add_user)))
            .service(web::resource("/plantdata").route(web::post().to(add_plant_data)))
            
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}