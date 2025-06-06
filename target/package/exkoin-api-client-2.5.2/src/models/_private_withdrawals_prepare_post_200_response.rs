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
pub struct PrivateWithdrawalsPreparePost200Response {
    #[serde(rename = "currency")]
    pub currency: Box<models::PublicCurrenciesGet200ResponseInner>,
    #[serde(rename = "amount_you_receive")]
    pub amount_you_receive: String,
    #[serde(rename = "fee")]
    pub fee: String,
    #[serde(rename = "amount_you_pay")]
    pub amount_you_pay: String,
    #[serde(rename = "address")]
    pub address: String,
    #[serde(rename = "network")]
    pub network: Box<models::PublicNetworksGet200ResponseInner>,
    #[serde(rename = "memo", skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(rename = "payload")]
    pub payload: String,
    #[serde(rename = "expire_in_ms")]
    pub expire_in_ms: f64,
}

impl PrivateWithdrawalsPreparePost200Response {
    pub fn new(currency: models::PublicCurrenciesGet200ResponseInner, amount_you_receive: String, fee: String, amount_you_pay: String, address: String, network: models::PublicNetworksGet200ResponseInner, payload: String, expire_in_ms: f64) -> PrivateWithdrawalsPreparePost200Response {
        PrivateWithdrawalsPreparePost200Response {
            currency: Box::new(currency),
            amount_you_receive,
            fee,
            amount_you_pay,
            address,
            network: Box::new(network),
            memo: None,
            payload,
            expire_in_ms,
        }
    }
}

