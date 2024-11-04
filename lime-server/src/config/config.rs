use std::env::args;
use std::fs;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    server: Server
}

impl Config {
    pub fn load_from_file() -> Result<Self, String>{
        let args: Vec<String> = args().collect();
        if args.len() < 2 {
            return Err("config not provide".to_string())
        }


        let path: PathBuf = args[1].clone().into();
        if !path.exists() {
            return Err(format!("config doesn't on the path {}", path.display()))
        }

        let config_string = fs::read_to_string(path)
            .map_err(|err| format!("read file: {err}"))?;

        let config: Config = toml::from_str(&config_string).
            map_err(|err| format!("parse config: {err}"))?;

        config.validate().map_err(|err| format!("validating config: {err}"))?;

        Ok(config)
    }

    fn validate(&self) -> Result<(), String> {
        self.server.validate().map_err(|err| format!( "validating server: {err}"))?;

        Ok(())
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Server {
    host: String,
    port: u16
}

impl Server {
    fn validate(&self) -> Result<(), String> {
        let addr = format!("{}:{}", self.host, self.port);
        SocketAddr::from_str(&addr)
            .map_err(|err| format!("invalid address {addr}: {err}"))?;

        Ok(())
    }
    pub fn addr(&self) -> SocketAddr {
        SocketAddr::from_str(&format!("{}:{}", self.host, self.port)).unwrap()
    }
}