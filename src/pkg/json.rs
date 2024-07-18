use actix_web::web::{self};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;

use super::error::InternalError;

pub fn unmarshal<'a, T>(str: &'a str) -> T
where
    T: Deserialize<'a>,
{
    let result = serde_json::from_str(str).unwrap();
    result
}

pub fn marshal<T>(obj: T) -> String
where
    T: Serialize,
{
    serde_json::to_string(&obj).unwrap()
}

pub async fn unmarshal_from_payload<T>(payload: web::Payload) -> Result<T, InternalError>
where
    T: DeserializeOwned,
{
    match payload.to_bytes().await {
        Ok(bytes) => {
            // Payload successfully loaded, now we can deserialize serde-json
            match serde_json::from_slice(&bytes) {
                Ok(obj) => Ok(obj),
                Err(e) => {
                    eprintln!("Error deserializing JSON: {}", e);
                    Err(InternalError::new(format!(
                        "Error deserializing JSON: {}",
                        e
                    )))
                }
            }
        }
        Err(e) => {
            eprintln!("Error loading payload: {}", e);
            Err(InternalError::new(format!(
                "Error deserializing JSON: {}",
                e
            )))
        }
    }
}
