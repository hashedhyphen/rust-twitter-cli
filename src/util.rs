extern crate base64;
extern crate rand;
extern crate regex;

use std::time;

use percent_encoding::{QUERY_ENCODE_SET, SIMPLE_ENCODE_SET,
                       utf8_percent_encode};
use self::regex::Regex;

use common::Params;

pub fn build_nonce() -> String {
    const BYTES: usize = 32;

    let mut buf: Vec<u8> = Vec::with_capacity(BYTES);
    for _ in 0..BYTES {
        buf.push(rand::random());
    }

    let nonce = base64::encode(&buf);

    let re = Regex::new("[^A-Za-z0-9]").unwrap();
    re.replace(nonce.as_str(), "").to_string()
}

pub fn build_timestamp() -> String {
    let duration = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap();
    duration.as_secs().to_string()
}

pub fn build_auth_value(params: &Params) -> String {
    let escaped: Vec<String> = params.iter().map(|(k, v)| {
        format!("{}=\"{}\"", percent_encode(k), percent_encode(v))
    }).collect();

    format!("OAuth {}", escaped.join(", "))
}

define_encode_set! {
    pub RFC_3986_ENCODE_SET = [SIMPLE_ENCODE_SET] | {
        ' ', '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '/',
        ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '`', '{', '|',
        '}'
    }
}

pub fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, RFC_3986_ENCODE_SET).to_string()
}

fn query_encode(input: &str) -> String {
    utf8_percent_encode(input, QUERY_ENCODE_SET).to_string()
}

pub fn stringify(params: &Params) -> String {
    let escaped: Vec<String> = params.iter().map(|(k, v)| {
        format!("{}={}", percent_encode(k), percent_encode(v))
    }).collect();

    escaped.join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encode_correctly() {
        let input = "An encoded string!";
        let expected = String::from("An%20encoded%20string%21");
        assert_eq!(expected, percent_encode(input));
    }
}
