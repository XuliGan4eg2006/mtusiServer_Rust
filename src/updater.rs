use redis::Commands;
use serde_json::json;
use crate::xlsx_parser;

const XLSX_PATH: &str = "D:\\Desktop_unz\\projects\\mtusiServer\\src\\test.xlsx";

pub fn update() {

    let client = redis::Client::open("redis://default:gGoRyUmsYKRj4G9GKPx80ofuE9ZiftM8@redis-10734.c81.us-east-1-2.ec2.redns.redis-cloud.com:10734").unwrap();
    let mut con = client.get_connection().unwrap();

    let group_map = xlsx_parser::get_groups_map();
    // let json_groups = json!({"groups_map": groups}); maybe later

    let json = serde_json::to_string(&group_map).unwrap();
    let _: () = con.set("groups_map", json).unwrap();
    println!("Groups map updated");

    let groups_timetable = xlsx_parser::get_timetable(group_map);

    for (key, value) in groups_timetable.as_object().unwrap().iter() {
        let json_stringed = serde_json::to_string(value).unwrap();
        let _: () = con.set(key, json_stringed).unwrap();
        println!("Timetable for {} updated", key);
    }
    println!("Timetable updated, ready to start....");
}