extern crate serde_json;

use serde_json::Value as JsonValue;

pub struct Server<'a> {
    hostname: &'a String,
    clients: &'a u8,
    max_clients: &'a u8,
    map_name: &'a String,
    game_type: &'a String
}

impl<'a> ToString for Server<'a> {
    fn to_string(&self) -> String {
        return format!("<hostname: {}, client: {}>", self.hostname, self.clients)
    }
}

pub fn parse_server_data<'a>(data: &JsonValue) -> Server {
    let hostname = data["hostname"].as_str().unwrap();
    let clients = data["clients"].as_str().unwrap();
    let max_clients = data["sv_maxclients"].as_str().unwrap();
    let game_type = data["gametype"].as_str().unwrap();
    let map_name = data["mapname"].as_str().unwrap();
    return Server {
        hostname: &hostname,
        clients: &clients,
        max_clients: &max_clients,
        map_name: &map_name,
        game_type: &game_type
    }
}