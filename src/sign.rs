extern crate base64;
extern crate crypto;

use self::crypto::mac::Mac;
use self::crypto::hmac::Hmac;
use self::crypto::sha1::Sha1;

use common::{Config, Params};
use util::percent_encode;

pub fn build_signature(config: &Config, base_url: &String, params: &Params)
    -> String
{
    let ps = build_parameter_string(&params);

    let sbs = vec![
        String::from("POST"),
        percent_encode(base_url),
        percent_encode(ps.as_str()),
    ].join("&");

    let key = vec![
        percent_encode(config.consumer_secret.as_str()),
        percent_encode(config.access_token_secret.as_str()),
    ].join("&");

    digest_hmac_sha1(sbs, key)
}

fn build_parameter_string(params: &Params) -> String {
    let mut params: Vec<_> = params.iter().collect();
    params.sort_by(|&(k1, v1), &(k2, v2)| k1.cmp(k2));

    let strings: Vec<String> = params.iter().map(|&(k, v)| {
        format!("{}={}", percent_encode(k), percent_encode(v))
    }).collect();

    strings.join("&")
}

fn digest_hmac_sha1(sbs: String, key: String) -> String {
    let mut hmac = Hmac::new(Sha1::new(), key.as_bytes());
    hmac.input(sbs.as_bytes());
    base64::encode(hmac.result().code())
}