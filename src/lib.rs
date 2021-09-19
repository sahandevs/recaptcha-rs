#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

pub mod error;
mod response;

use reqwest::Url;
use reqwest::blocking::{ClientBuilder};
use std::collections::HashSet;
use std::net::IpAddr;
use std::time::Duration;

pub use crate::error::Error;

use crate::response::RecaptchaResponse;

/// Verify a recaptcha user response
pub fn verify(
    key: &str,
    response: &str,
    user_ip: Option<&IpAddr>,
    timeout: Duration,
) -> Result<(), Error> {
    let user_ip = user_ip.map(ToString::to_string);

    let mut url = Url::parse("https://www.google.com/recaptcha/api/siteverify").unwrap();

    url.query_pairs_mut()
        .extend_pairs(&[("secret", key), ("response", response)]);

    if let Some(user_ip) = user_ip {
        url.query_pairs_mut().append_pair("remoteip", &user_ip);
    }

    let client = ClientBuilder::new()
        .connect_timeout(timeout)
        .timeout(timeout)
        .build()?;

    let response = client.get(url).send()?;
    let recaptcha_response = response.json::<RecaptchaResponse>()?;

    match (recaptcha_response.success, recaptcha_response.error_codes) {
        (true, _) => Ok(()),
        (false, Some(errors)) => Err(Error::Codes(errors)),
        (false, _) => Err(Error::Codes(HashSet::new())),
    }
}

#[test]
fn test_invalid_secret_missing_response() {
    use error::Code::*;
    use error::Error::*;
    let resp = verify("", "", None, Duration::from_secs(10));

    match resp {
        Ok(()) => panic!("unexpected response: Ok(())"),
        Err(Codes(ref errors)) => {
            assert!(errors.contains(&InvalidSecret));
            assert!(errors.contains(&MissingResponse));
        }
        Err(e) => panic!("unexpected error: {}", e),
    };

    println!("{:?}", resp);
}
