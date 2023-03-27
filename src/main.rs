//Importing created modules (This is to enhance readibility)
mod db;
mod models;
mod config;
mod errors; 
mod handlers;


//Using essential features (These are imported from crates)
use dotenv::dotenv;
use ::config::Config;
use tokio_postgres::NoTls;
use crate::{config::ExampleConfig};
use actix_web::{web, App, HttpServer};
use handlers::{add_user, get_plant_data, add_plant_data};


//Main Program Starts From Here
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
            .service(web::resource("/getplantdata").route(web::get().to(get_plant_data)))

    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running at http://{}/", config.server_addr);
    server.await
}