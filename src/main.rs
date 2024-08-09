#[macro_use] extern crate rocket;

use rocket::tokio::sync::Mutex;
use rocket::State;
use redis::AsyncCommands;

mod updater;
mod xlsx_parser;
struct AppState {
    redis_client: Mutex<redis::Client>,
}

#[rocket::main]
async fn main() {
    let redis_client = redis::Client::open("redis://default:gGoRyUmsYKRj4G9GKPx80ofuE9ZiftM8@redis-10734.c81.us-east-1-2.ec2.redns.redis-cloud.com:10734").unwrap();

    updater::update();

    let app_state = AppState {
        redis_client: Mutex::new(redis_client),
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
