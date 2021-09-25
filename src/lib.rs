#[cfg(test)]
#[warn(non_snake_case)]
extern crate reqwest;
extern crate serde_json;

use serde_json::Value as JsonValue;

mod player;
mod server;

fn _fetch_json(url: &String) -> JsonValue {
    match reqwest::get(&url) {
        Ok(mut res) => {
           let json = serde_json::from_str(&res.text()); 
           return json
        }, 
        Err(e) => println!("{}", e)
    }
}

pub struct fivem {
    ip: String,
    port: String
}

impl fivem {
    pub fn get_players_raw(&self) -> JsonValue {
        let url = format!("http://{}:{}/players.json", self.ip, self.port);
        let players: JsonValue = _fetch_json(&url);
        return players;
    }

    pub fn get_info_raw(&self) -> JsonValue {
        let url = format!("http://{}:{}/info.json", self.ip, self.port);
        let info: JsonValue = _fetch_json(&url);
        return info;
    }

    pub fn get_dynamic_raw(&self) -> JsonValue {
        let url = format!("http://{}:{}/dynamic.json", self.ip, self.port);
        let dynamic: JsonValue = _fetch_json(&url);
        return dynamic;
    }

    pub fn get_players(&self) -> Vec<String> {
        let players_data: JsonValue = self.get_players_raw();
        return player::parse_players_json(&players_data);
    }

    pub fn get_server_info(&self) -> server::Server {
        let server_data: JsonValue = self.get_info_raw();
        return server::parse_server_data(&server_data);
    }
}

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
