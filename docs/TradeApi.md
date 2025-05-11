# \TradeApi

All URIs are relative to *https://api.exkoin.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**private_orders_cancel_post**](TradeApi.md#private_orders_cancel_post) | **POST** /private/orders/cancel | 
[**private_orders_get**](TradeApi.md#private_orders_get) | **GET** /private/orders | 
[**private_orders_post**](TradeApi.md#private_orders_post) | **POST** /private/orders | 
[**private_trades_get**](TradeApi.md#private_trades_get) | **GET** /private/trades | 



## private_orders_cancel_post

> models::PrivateOrdersGet200ResponseInner private_orders_cancel_post(private_orders_cancel_post_request)


Cancel an order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_orders_cancel_post_request** | [**PrivateOrdersCancelPostRequest**](PrivateOrdersCancelPostRequest.md) |  | [required] |

### Return type

[**models::PrivateOrdersGet200ResponseInner**](_private_orders_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_orders_get

> Vec<models::PrivateOrdersGet200ResponseInner> private_orders_get(symbol, is_open, has_filled, side, page, limit)


Get the user orders

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> |  |  |
**is_open** | Option<[**serde_json::Value**](.md)> |  |  |
**has_filled** | Option<[**serde_json::Value**](.md)> |  |  |
**side** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PrivateOrdersGet200ResponseInner>**](_private_orders_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_orders_post

> models::PrivateOrdersGet200ResponseInner private_orders_post(private_orders_post_request)


Create a new order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_orders_post_request** | [**PrivateOrdersPostRequest**](PrivateOrdersPostRequest.md) |  | [required] |

### Return type

[**models::PrivateOrdersGet200ResponseInner**](_private_orders_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_trades_get

> Vec<models::PrivateTradesGet200ResponseInner> private_trades_get(symbol, side, taker_or_maker, page, limit)


Get the user trades

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | Option<**String**> |  |  |
**side** | Option<**String**> |  |  |
**taker_or_maker** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PrivateTradesGet200ResponseInner>**](_private_trades_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

