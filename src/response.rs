use crate::error::Code;
use std::collections::HashSet;

#[derive(Debug, Deserialize)]
pub struct RecaptchaResponse {
    pub success: bool,
    #[serde(rename = "error-codes")]
    pub error_codes: Option<HashSet<Code>>,
}

#[test]
fn decoding_test() {
    extern crate serde_json as json;
    use super::*;

    let resp = json::from_str::<RecaptchaResponse>(
        r#"{
        "success": true,
        "error-codes": ["missing-input-secret", "foo"]
    }"#,
    )
    .unwrap();

    assert!(resp.success);
    assert!(resp.error_codes.is_some());

    let errors = resp.error_codes.unwrap();
    assert!(errors.len() == 2);
    assert!(errors.contains(&Code::MissingSecret));
    assert!(errors.contains(&Code::Unknown("foo".to_string())));
}
