use std::error::Error;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::from_str;


use lazy_static::lazy_static;
use rbatis::rbatis::Rbatis;

lazy_static! {
  pub(crate) static ref RB : Rbatis = Rbatis::new();
}


pub async fn create_connection() {
    let connectionstring = load_connection_string();
    crate::database::RB.link(&*connectionstring).await.unwrap();
}

fn load_connection_string() -> String {
    let content :String;
    match load_config_file() {
        Ok(config) => content = config,
        Err(_err) => {
            let empty_config =  Config {
                address: "".to_string(),
                username: "".to_string(),
                password: "".to_string(),
                database: "".to_string(),
            };

            match serde_json::to_vec_pretty(&empty_config){
                Ok(value) => {
                    let _result = fs::write("./config.json", value);
                    panic!();
                },
                Err(_err) => {
                    panic!();
                },
            }
        }
    }

    let config: Config = from_str(&*content).unwrap();


    return format!("postgres://{}:{}@{}:5432/{}", config.username, config.password, config.address, config.database);
}

fn load_config_file() -> Result<String, Box<dyn Error>>{
    let content = fs::read_to_string("./config.json")?;
    Ok(content)
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub address: String,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[crud_table(table_name:"ewd.lnk_links" | table_columns:"lnk_id,lnk_path,lnk_target,lnk_hide_target")]
#[derive(Clone, Debug)]
pub struct Links {
    pub lnk_id: Option<i32>,
    pub lnk_path: String,
    pub lnk_target: String,
    pub lnk_hide_target: i32,
}

#[crud_table(table_name:"ewd.lnk_opengraph" | table_columns:"log_id,log_link_id,log_tag,log_content")]
#[derive(Clone, Debug)]
pub struct OpenGraph {
    pub log_id: Option<i32>,
    pub log_link_id: i32,
    pub log_tag: String,
    pub log_content: String,
}

#[crud_table(table_name:"ewd.adm_api_keys" | table_columns:"aak_id, aak_key, aak_enabled")]
#[derive(Clone, Debug)]
pub struct ApiKeys {
    pub aak_id: i32,
    pub aak_key: String,
    pub aak_enabled: bool,
}

#[crud_table(table_name:"ewd.adm_api_keys_blocked_list" | table_columns:"akb_id, akb_key_id, akb_ip")]
#[derive(Clone, Debug)]
pub struct ApiKeysBlocked {
    pub akb_id: i32,
    pub akb_key_id: i32,
    pub akb_ip: String,
}