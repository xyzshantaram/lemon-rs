extern crate serde_derive;
use std::collections::HashMap;

use serde_derive::{Deserialize, Serialize};
use toml::{value::Map, Value};

use crate::emitter::Alignment;

pub fn read_config() -> Result<String, std::io::Error> {
    let cfg_dir = dirs::config_dir().expect("Error: home dir couldn't be determined!");
    let cfg_path = cfg_dir.join("lemon-rs").join("config.toml");
    std::fs::read_to_string(cfg_path)
}

pub fn cfg_toml() -> Option<toml::Value> {
    match read_config() {
        Err(_) => {
            eprintln!("Reading config file failed. Falling back to defaults");
            None
        }
        Ok(x) => Some(match x.parse::<toml::Value>() {
            Ok(val) => val,
            Err(x) => panic!("Error parsing config: {}", x),
        }),
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub order: Vec<String>,
    pub fallback_tmp_path: Option<String>,
    pub options: HashMap<String, EmitterOptions>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EmitterOptions {
    delay: Option<i64>,
    icon: Option<String>,
    alignment: Option<Alignment>,
}

fn default_order() -> Vec<String> {
    ["title", "clock", "mem", "vol", "net"]
        .map(String::from)
        .to_vec()
}

fn get_emitter_options(table: &Map<String, Value>) -> EmitterOptions {
    let mut options = EmitterOptions::default();
    if table.contains_key("delay") {
        options.delay = table.get("delay").unwrap().as_integer();
    }
    if table.contains_key("alignment") {
        options.alignment = table
            .get("alignment")
            .unwrap()
            .as_str()
            .map(|val| match val {
                "left" => Alignment::Left,
                "right" => Alignment::Right,
                "center" => Alignment::Center,
                _ => Alignment::Continue,
            });
    };

    options
}

pub fn get() -> Config {
    let cfg = cfg_toml();
    let mut res = Config::default();
    if let Some(cfg) = cfg {
        let order: Vec<String> = cfg["emitters"]["order"]
            .as_array()
            .expect("error parsing config.toml: order should not be empty")
            .iter()
            .map(|v| v.as_str())
            .map(|v| v.expect("???").to_owned())
            .collect();
        res.order = order.clone();

        for emitter in &order {
            if let Some(val) = cfg["emitters"].get(emitter) {
                let tmp = Map::default();
                let table = val.as_table().unwrap_or(&tmp);
                res.options
                    .insert(emitter.clone(), get_emitter_options(table));
                if let Some(val) = table
                    .get("temp_path")
                    .map(|v| v.as_str().expect("temp path must be string").to_owned())
                {
                    res.fallback_tmp_path = Some(val);
                }
            }
        }

        res
    } else {
        Config {
            order: default_order(),
            ..Default::default()
        }
    }
}
