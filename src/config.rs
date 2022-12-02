use serde_yaml::Error;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Configuration{
    log_level: String,
    db_url: String,
    port: u16,
    sleep_time: u16,
    username: String,
    password: String,
    title: String,
    description: String,
    url: String,
    cookies: String,
    folder: String,
    hmac_key: String,
}

impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        serde_yaml::from_str(content)
    }
    pub fn get_log_level(&self) -> &str{
        &self.log_level
    }
    pub fn get_db_url(&self) -> &str{
        &self.db_url
    }
    pub fn get_sleep_time(&self) -> u16{
        self.sleep_time
    }
    pub fn get_port(&self) -> u16{
        self.port
    }
    pub fn get_username(&self) -> &str{
        &self.username
    }
    pub fn get_password(&self) -> &str{
        &self.password
    }
    pub fn get_title(&self) -> &str{
        &self.title
    }
    pub fn get_description(&self) -> &str{
        &self.description
    }
    pub fn get_url(&self) -> &str{
        &self.url
    }
    pub fn get_cookies(&self) -> &str{
        &self.cookies
    }
    pub fn get_folder(&self) -> &str{
        &self.folder
    }
    pub fn get_hmac_key(&self) -> &str{
        &self.hmac_key
    }
}