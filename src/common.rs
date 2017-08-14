use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub consumer_key: String,
    pub consumer_secret: String,
    pub access_token: String,
    pub access_token_secret: String,
}

impl Config {
    pub fn new()-> Result<Config, Box<env::VarError>> {
        let consumer_key = env::var("TWITTER_CONSUMER_KEY")?;
        let consumer_secret = env::var("TWITTER_CONSUMER_SECRET")?;
        let access_token = env::var("TWITTER_ACCESS_TOKEN")?;
        let access_token_secret = env::var("TWITTER_ACCESS_TOKEN_SECRET")?;

        Ok(Config {
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        })
    }
}

pub type Params = HashMap<&'static str, String>;
