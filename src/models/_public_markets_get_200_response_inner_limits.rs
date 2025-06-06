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
pub struct PublicMarketsGet200ResponseInnerLimits {
    #[serde(rename = "price")]
    pub price: Box<models::PublicMarketsGet200ResponseInnerLimitsPrice>,
    #[serde(rename = "amount")]
    pub amount: Box<models::PublicMarketsGet200ResponseInnerLimitsPrice>,
    #[serde(rename = "cost")]
    pub cost: Box<models::PublicMarketsGet200ResponseInnerLimitsPrice>,
}

impl PublicMarketsGet200ResponseInnerLimits {
    pub fn new(price: models::PublicMarketsGet200ResponseInnerLimitsPrice, amount: models::PublicMarketsGet200ResponseInnerLimitsPrice, cost: models::PublicMarketsGet200ResponseInnerLimitsPrice) -> PublicMarketsGet200ResponseInnerLimits {
        PublicMarketsGet200ResponseInnerLimits {
            price: Box::new(price),
            amount: Box::new(amount),
            cost: Box::new(cost),
        }
    }
}

