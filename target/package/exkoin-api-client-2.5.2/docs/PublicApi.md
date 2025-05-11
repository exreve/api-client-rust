# \PublicApi

All URIs are relative to *https://api.exkoin.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**public_currencies_get**](PublicApi.md#public_currencies_get) | **GET** /public/currencies | 
[**public_exk_config_get**](PublicApi.md#public_exk_config_get) | **GET** /public/exk/config | 
[**public_exk_reserves_get**](PublicApi.md#public_exk_reserves_get) | **GET** /public/exk/reserves | 
[**public_forex_prices_get**](PublicApi.md#public_forex_prices_get) | **GET** /public/forex-prices | 
[**public_funds_transparency_get**](PublicApi.md#public_funds_transparency_get) | **GET** /public/funds-transparency | 
[**public_gains_losses_transparency_get**](PublicApi.md#public_gains_losses_transparency_get) | **GET** /public/gains-losses-transparency | 
[**public_markets_get**](PublicApi.md#public_markets_get) | **GET** /public/markets | 
[**public_markets_tickers_list_get**](PublicApi.md#public_markets_tickers_list_get) | **GET** /public/markets/tickers-list | 
[**public_networks_get**](PublicApi.md#public_networks_get) | **GET** /public/networks | 
[**public_ohlcv_get**](PublicApi.md#public_ohlcv_get) | **GET** /public/ohlcv | 
[**public_orderbook_get**](PublicApi.md#public_orderbook_get) | **GET** /public/orderbook | 
[**public_pools_fees_history_get**](PublicApi.md#public_pools_fees_history_get) | **GET** /public/pools/fees-history | 
[**public_pools_get**](PublicApi.md#public_pools_get) | **GET** /public/pools | 
[**public_trades_get**](PublicApi.md#public_trades_get) | **GET** /public/trades | 



## public_currencies_get

> Vec<models::PublicCurrenciesGet200ResponseInner> public_currencies_get()


Get all active currencies

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PublicCurrenciesGet200ResponseInner>**](_public_currencies_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_exk_config_get

> models::PublicExkConfigGet200Response public_exk_config_get()


Get the EXK token configuration

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicExkConfigGet200Response**](_public_exk_config_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_exk_reserves_get

> std::collections::HashMap<String, String> public_exk_reserves_get()


Get the EXK reserves

### Parameters

This endpoint does not need any parameter.

### Return type

**std::collections::HashMap<String, String>**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_forex_prices_get

> models::PublicForexPricesGet200Response public_forex_prices_get()


Get the forex prices, base is in USD, eg. EUR, GBP, HKD, SGD, ... Data is from an external provider.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicForexPricesGet200Response**](_public_forex_prices_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_funds_transparency_get

> models::PublicFundsTransparencyGet200Response public_funds_transparency_get()


Get the funds transparency data (overall exchange balances)

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PublicFundsTransparencyGet200Response**](_public_funds_transparency_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_gains_losses_transparency_get

> models::PublicGainsLossesTransparencyGet200Response public_gains_losses_transparency_get(unclaimed_only, period)


Get transparency information about exchange gains and losses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**unclaimed_only** | Option<[**serde_json::Value**](.md)> |  |  |
**period** | Option<**String**> |  |  |

### Return type

[**models::PublicGainsLossesTransparencyGet200Response**](_public_gains_losses_transparency_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_markets_get

> Vec<models::PublicMarketsGet200ResponseInner> public_markets_get()


Get all active markets

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PublicMarketsGet200ResponseInner>**](_public_markets_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_markets_tickers_list_get

> Vec<models::PublicMarketsTickersListGet200ResponseInner> public_markets_tickers_list_get()


Get the 24h tickers for all markets

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PublicMarketsTickersListGet200ResponseInner>**](_public_markets_tickers_list_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_networks_get

> Vec<models::PublicNetworksGet200ResponseInner> public_networks_get()


Get all active networks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PublicNetworksGet200ResponseInner>**](_public_networks_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_ohlcv_get

> Vec<models::PublicOhlcvGet200ResponseInner> public_ohlcv_get(symbol, interval, page, limit)


Get the OHLCV data for a trading pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |
**interval** | **f64** |  | [required] |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PublicOhlcvGet200ResponseInner>**](_public_ohlcv_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_orderbook_get

> models::PublicOrderbookGet200Response public_orderbook_get(symbol)


Get the orderbook for a trading pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |

### Return type

[**models::PublicOrderbookGet200Response**](_public_orderbook_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_pools_fees_history_get

> Vec<models::PublicPoolsFeesHistoryGet200ResponseInner> public_pools_fees_history_get(pool_id, start_time, end_time, limit, page)


Get the pool fees earnings history

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | Option<**String**> |  |  |
**start_time** | Option<**f64**> |  |  |
**end_time** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |
**page** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PublicPoolsFeesHistoryGet200ResponseInner>**](_public_pools_fees_history_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_pools_get

> Vec<models::PublicPoolsGet200ResponseInner> public_pools_get()


Get all active pools

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::PublicPoolsGet200ResponseInner>**](_public_pools_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## public_trades_get

> Vec<models::PublicTradesGet200ResponseInner> public_trades_get(symbol, page, limit)


Get the public trades for a trading pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**symbol** | **String** |  | [required] |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PublicTradesGet200ResponseInner>**](_public_trades_get_200_response_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

