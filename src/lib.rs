extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
#[macro_use] extern crate percent_encoding;

use std::collections::HashMap;
use std::env;
use std::error;
use std::io::{self, Write};

use futures::{Future, Stream};
use hyper::Client;
use hyper::Method::Post;
use hyper::Request;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Core;

mod common;
mod util;
mod sign;

pub use common::Config;
use common::Params;

fn build_oauth_params_from(config: &Config) -> Params {
    let mut params = HashMap::new();
    params.insert("oauth_consumer_key", config.consumer_key.clone());
    params.insert("oauth_nonce", util::build_nonce());
    params.insert("oauth_signature_method", String::from("HMAC-SHA1"));
    params.insert("oauth_timestamp", util::build_timestamp());
    params.insert("oauth_token", config.access_token.clone());
    params.insert("oauth_version", String::from("1.0"));
    params
}

fn build_params_set_from(oauth_params: &Params, body_params: &Params) -> Params {
    let mut params = oauth_params.clone();
    for (key, val) in body_params.iter() {
        params.insert(key, val.clone());
    }
    params
}

pub fn run(message: String, config: Config) -> Result<(), Box<error::Error>> {
    let mut core = Core::new().unwrap();

    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle()).unwrap())
        .build(&core.handle());

    let mut body_params = HashMap::new();
    body_params.insert("status", message);

    let mut oauth_params = build_oauth_params_from(&config);

    let params_set = build_params_set_from(&oauth_params, &body_params);

    let base_url = String::from("https://api.twitter.com/1.1/statuses/update.json");

    let signature = sign::build_signature(&config, &base_url, &params_set);
    oauth_params.insert("oauth_signature", signature);

    let auth_value = util::build_auth_value(&oauth_params);

    let uri = base_url.parse().unwrap();
    let mut request = Request::new(Post, uri);
    {
        let mut headers = request.headers_mut();
        headers.set_raw("Content-Type", "application/x-www-form-urlencoded");
        headers.set_raw("Authorization", auth_value);
    }
    let body = util::stringify(&body_params);
    println!("{}", body);
    request.set_body(util::stringify(&body_params));
    println!("{:?}", request);
    let work = client.request(request).map(|res| {
        println!("Response: {}", res.status());
    });

    core.run(work)?;

    Ok(())
}
