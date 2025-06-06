/*
 * ExKoin API Documentation
 *
 * API documentation for ExKoin Crypto exchange
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`private_orders_cancel_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateOrdersCancelPostError {
    DefaultResponse(models::PublicCurrenciesGetDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_orders_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateOrdersGetError {
    DefaultResponse(models::PublicCurrenciesGetDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_orders_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateOrdersPostError {
    DefaultResponse(models::PublicCurrenciesGetDefaultResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`private_trades_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrivateTradesGetError {
    DefaultResponse(models::PublicCurrenciesGetDefaultResponse),
    UnknownValue(serde_json::Value),
}


/// Cancel an order
pub async fn private_orders_cancel_post(configuration: &configuration::Configuration, private_orders_cancel_post_request: models::PrivateOrdersCancelPostRequest) -> Result<models::PrivateOrdersGet200ResponseInner, Error<PrivateOrdersCancelPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_private_orders_cancel_post_request = private_orders_cancel_post_request;

    let uri_str = format!("{}/private/orders/cancel", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("sessiontoken", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-NONCE", value);
    };
    req_builder = req_builder.json(&p_private_orders_cancel_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PrivateOrdersGet200ResponseInner`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PrivateOrdersGet200ResponseInner`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateOrdersCancelPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get the user orders
pub async fn private_orders_get(configuration: &configuration::Configuration, symbol: Option<&str>, is_open: Option<models::serde_json::Value>, has_filled: Option<models::serde_json::Value>, side: Option<&str>, page: Option<f64>, limit: Option<f64>) -> Result<Vec<models::PrivateOrdersGet200ResponseInner>, Error<PrivateOrdersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_is_open = is_open;
    let p_has_filled = has_filled;
    let p_side = side;
    let p_page = page;
    let p_limit = limit;

    let uri_str = format!("{}/private/orders", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_is_open {
        req_builder = req_builder.query(&[("is_open", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_has_filled {
        req_builder = req_builder.query(&[("has_filled", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_side {
        req_builder = req_builder.query(&[("side", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("sessiontoken", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-NONCE", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::PrivateOrdersGet200ResponseInner&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::PrivateOrdersGet200ResponseInner&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateOrdersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Create a new order
pub async fn private_orders_post(configuration: &configuration::Configuration, private_orders_post_request: models::PrivateOrdersPostRequest) -> Result<models::PrivateOrdersGet200ResponseInner, Error<PrivateOrdersPostError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_private_orders_post_request = private_orders_post_request;

    let uri_str = format!("{}/private/orders", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("sessiontoken", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-NONCE", value);
    };
    req_builder = req_builder.json(&p_private_orders_post_request);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PrivateOrdersGet200ResponseInner`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PrivateOrdersGet200ResponseInner`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateOrdersPostError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// Get the user trades
pub async fn private_trades_get(configuration: &configuration::Configuration, symbol: Option<&str>, side: Option<&str>, taker_or_maker: Option<&str>, page: Option<f64>, limit: Option<f64>) -> Result<Vec<models::PrivateTradesGet200ResponseInner>, Error<PrivateTradesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_symbol = symbol;
    let p_side = side;
    let p_taker_or_maker = taker_or_maker;
    let p_page = page;
    let p_limit = limit;

    let uri_str = format!("{}/private/trades", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_symbol {
        req_builder = req_builder.query(&[("symbol", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_side {
        req_builder = req_builder.query(&[("side", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_taker_or_maker {
        req_builder = req_builder.query(&[("taker_or_maker", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_page {
        req_builder = req_builder.query(&[("page", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_limit {
        req_builder = req_builder.query(&[("limit", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-KEY", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-SIGN", value);
    };
    if let Some(ref auth_conf) = configuration.basic_auth {
        req_builder = req_builder.basic_auth(auth_conf.0.to_owned(), auth_conf.1.to_owned());
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("sessiontoken", value);
    };
    if let Some(ref apikey) = configuration.api_key {
        let key = apikey.key.clone();
        let value = match apikey.prefix {
            Some(ref prefix) => format!("{} {}", prefix, key),
            None => key,
        };
        req_builder = req_builder.header("X-API-NONCE", value);
    };

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::PrivateTradesGet200ResponseInner&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::PrivateTradesGet200ResponseInner&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrivateTradesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

