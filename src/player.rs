extern crate serde_json;

use serde_json::Value as JsonValue;

struct Player {
    name: String,
    id: u8,
    ping: u16,
    xbl_id: String,
    steam_id: String,
    discord_id: String,
    live_id: String,
    license_id: String
}

impl ToString for Player {
    fn to_string(&self) -> String {
        return format!("<Player: {}, id: {}, ping: {}>", self.name, self.id, self.ping)
    }
}

fn parse_identifier(identifier: String) -> Vec<String> {
    let (service, id) = identifier.split(":");
    return vec![&service, &id]
}

fn parse_players_json(data: &JsonValue) -> Vec<String> {
    let mut players: Vec<Player> =  Vec::new();

    for p in &data.iter() {
        let name = p["name"];
        let id = p["id"];
        let ping = p["ping"];
        let identifiers: Vec<String> = p["identifiers"];

        let mut xbl_id = String::new();
        let mut steam_id = String::new();
        let mut discord_id = String::new();
        let mut live_id = String::new();
        let mut license_id = String::new();

        if identifiers != None {
            for identifier in &identifiers.iter() {
                let (service, id) = parse_identifier(identifier);
                match service {
                    "xbl" => xbl_id = id,
                    "steam" => steam_id = id,
                    "discord" => discord_id = id,
                    "live" => live_id = id,
                    "license" => license_id = id
                }
                players.push(Player {
                    name: &name,
                    id: &id,
                    ping: &ping,
                    xbl_id: &xbl_id,
                    steam_id: &steam_id,
                    discord_id: &discord_id,
                    live_id: &live_id,
                    license_id: &license_id

                });
            }
        }
        return players;
        
    }

}