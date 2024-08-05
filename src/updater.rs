use redis::Commands;
use crate::xlsx_parser;
pub fn update() {
    let client = redis::Client::open("redis://default:gGoRyUmsYKRj4G9GKPx80ofuE9ZiftM8@redis-10734.c81.us-east-1-2.ec2.redns.redis-cloud.com:10734").unwrap();
    let mut con = client.get_connection().unwrap();

    let groups = xlsx_parser::get_groups_map();
    let json = serde_json::to_string(&groups).unwrap();
    let _: () = con.set("groups", json).unwrap();
}