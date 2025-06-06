/*
 * ExKoin API Documentation
 *
 * API documentation for ExKoin Crypto exchange
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PublicFundsTransparencyGet200ResponseFundsValueValue {
    #[serde(rename = "total")]
    pub total: f64,
    #[serde(rename = "in_cold")]
    pub in_cold: f64,
    #[serde(rename = "free")]
    pub free: f64,
    #[serde(rename = "locked")]
    pub locked: f64,
    #[serde(rename = "address_count")]
    pub address_count: f64,
    #[serde(rename = "addresses")]
    pub addresses: Vec<String>,
}

impl PublicFundsTransparencyGet200ResponseFundsValueValue {
    pub fn new(total: f64, in_cold: f64, free: f64, locked: f64, address_count: f64, addresses: Vec<String>) -> PublicFundsTransparencyGet200ResponseFundsValueValue {
        PublicFundsTransparencyGet200ResponseFundsValueValue {
            total,
            in_cold,
            free,
            locked,
            address_count,
            addresses,
        }
    }
}

