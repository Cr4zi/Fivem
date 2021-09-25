extern crate serde_json;

use serde_json::Value as JsonValue;

pub struct Server {
    hostname: &String,
    clients: &u8,
    max_clients: &u8,
    map_name: &String,
    game_type: &String
}

impl ToString for Server {
    fn to_string(&self) -> String {
        return format!("<hostname: {}, client: {}>", self.hostname, self.clients)
    }
}

pub fn parse_server_data(data: &JsonValue) -> Server {
    let hostname = data["hostname"];
    let clients = data["clients"];
    let max_clients = data["sv_maxclients"];
    let game_type = data["gametype"];
    let map_name = data["mapname"];
    return Server {
        hostname: &hostname,
        clients: &clients,
        max_clients: &max_clients,
        map_name: &map_name,
        game_type: &game_type
    }
}