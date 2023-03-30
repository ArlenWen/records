use std::fs;


#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub log: LogConfig,
    pub service: ServiceConf,

}

#[derive(serde::Deserialize, Debug)]
pub struct ServiceConf {
    pub host: String,
    pub port: u16,

}

#[derive(serde::Deserialize, Debug)]
pub struct LogConfig {
    pub stdout: StdoutLogConfig,
    pub file: FileLogConfig,
    pub level: String,
}


#[derive(serde::Deserialize, Debug)]
pub struct StdoutLogConfig {
    pub pattern: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct FileLogConfig {
    pub pattern: String,
    pub path: String,
}


pub fn read_conf_file() -> String {
    fs::read_to_string("conf.yaml")
        .expect("should have been able to read file[conf.yaml].")
}

pub fn parse_config(conf_str: String) -> Config {
    let config: Config = serde_yaml::from_str(&conf_str).expect("cannot parse config with yaml format.");
    config
}





