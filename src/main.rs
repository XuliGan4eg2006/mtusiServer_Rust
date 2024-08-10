#[macro_use] extern crate rocket;

use rocket::tokio::sync::Mutex;
use rocket::State;
use redis::AsyncCommands;
use tokio_postgres::{NoTls, Error};

mod updater;
mod xlsx_parser;
struct AppState {
    redis_client: Mutex<redis::Client>,
    postgres_client: Mutex<tokio_postgres::Client>,
}

#[rocket::main]
async fn main() {
    let redis_client = redis::Client::open("redis://192.168.0.112:8002").unwrap();
    let (client, connection) = tokio_postgres::connect("host=192.168.0.112 user=postgres password=456789 port=8001", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
        else {
            println!("connection established");
        }
    });
    
    updater::update();

    let app_state = AppState {
        redis_client: Mutex::new(redis_client),
        postgres_client: Mutex::new(client)
    };

    rocket::build()
        .manage(app_state)
        .mount("/", routes![get_groups])
        .mount("/", routes![get_timetable])
        .launch()
        .await
        .unwrap();
}

#[get("/get_groups")]
async fn get_groups(state: &State<AppState>) -> serde_json::Value {
    let client = state.redis_client.lock().await;
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let groups: String = con.get("groups_map").await.unwrap();

    serde_json::from_str(&groups).unwrap()
}

#[get("/get_timetable/<group>")]
async fn get_timetable(state: &State<AppState>, group: &str) -> serde_json::Value {

    let client = state.redis_client.lock().await;
    let mut con = client.get_multiplexed_async_connection().await.unwrap();
    let groups: String = con.get(group).await.unwrap_or("{}".to_string());

    let groups_json = serde_json::from_str(&groups).unwrap();

    groups_json
}

#[get("/get_timetable/<token>")]
async fn post_token(state: &State<AppState>, token: &str) -> serde_json::Value {

    let client = state.postgres_client.lock().await;
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await.unwrap();



    let groups_json = serde_json::from_str(&token).unwrap();

    groups_json
}