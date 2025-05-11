# \AccountApi

All URIs are relative to *https://api.exkoin.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**private_balances_get**](AccountApi.md#private_balances_get) | **GET** /private/balances | 
[**private_deposit_address_get**](AccountApi.md#private_deposit_address_get) | **GET** /private/deposit-address | 
[**private_deposits_get**](AccountApi.md#private_deposits_get) | **GET** /private/deposits | 
[**private_withdrawal_get**](AccountApi.md#private_withdrawal_get) | **GET** /private/withdrawal | 
[**private_withdrawals_detail_get**](AccountApi.md#private_withdrawals_detail_get) | **GET** /private/withdrawals/detail | 
[**private_withdrawals_finish_post**](AccountApi.md#private_withdrawals_finish_post) | **POST** /private/withdrawals/finish | 
[**private_withdrawals_get**](AccountApi.md#private_withdrawals_get) | **GET** /private/withdrawals | 
[**private_withdrawals_prepare_post**](AccountApi.md#private_withdrawals_prepare_post) | **POST** /private/withdrawals/prepare | 



## private_balances_get

> models::PrivateBalancesGet200Response private_balances_get()


Get the user balances

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::PrivateBalancesGet200Response**](_private_balances_get_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_deposit_address_get

> models::PrivateDepositAddressGet200Response private_deposit_address_get(network, currency)


Get a deposit address and informations for a currency/network pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**network** | **String** |  | [required] |
**currency** | **String** |  | [required] |

### Return type

[**models::PrivateDepositAddressGet200Response**](_private_deposit_address_get_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_deposits_get

> Vec<models::PrivateDepositsGet200ResponseInner> private_deposits_get(currency, network, status, limit, page, start_time, end_time)


Get the user deposits

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<**String**> |  |  |
**network** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**page** | Option<**f64**> |  |  |
**start_time** | Option<**f64**> |  |  |
**end_time** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PrivateDepositsGet200ResponseInner>**](_private_deposits_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_withdrawal_get

> models::PrivateWithdrawalsFinishPost200Response private_withdrawal_get(id)


Get a specific withdrawal

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

[**models::PrivateWithdrawalsFinishPost200Response**](_private_withdrawals_finish_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_withdrawals_detail_get

> models::PrivateWithdrawalsDetailGet200Response private_withdrawals_detail_get(currency, network)


Get the withdrawal details for a currency/network pair

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | **String** |  | [required] |
**network** | **String** |  | [required] |

### Return type

[**models::PrivateWithdrawalsDetailGet200Response**](_private_withdrawals_detail_get_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_withdrawals_finish_post

> models::PrivateWithdrawalsFinishPost200Response private_withdrawals_finish_post(private_withdrawals_finish_post_request)


Finish a withdrawal (use the payload from prepare-withdraw)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_withdrawals_finish_post_request** | [**PrivateWithdrawalsFinishPostRequest**](PrivateWithdrawalsFinishPostRequest.md) |  | [required] |

### Return type

[**models::PrivateWithdrawalsFinishPost200Response**](_private_withdrawals_finish_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_withdrawals_get

> Vec<models::PrivateWithdrawalsFinishPost200Response> private_withdrawals_get(currency, network, limit, status, page, start_time, end_time)


Get the user withdrawals

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**currency** | Option<**String**> |  |  |
**network** | Option<**String**> |  |  |
**limit** | Option<**f64**> |  |  |
**status** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**start_time** | Option<**f64**> |  |  |
**end_time** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PrivateWithdrawalsFinishPost200Response>**](_private_withdrawals_finish_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_withdrawals_prepare_post

> models::PrivateWithdrawalsPreparePost200Response private_withdrawals_prepare_post(private_withdrawals_prepare_post_request)


Prepare a withdrawal, then you need to call withdraw-finish with the payload

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_withdrawals_prepare_post_request** | [**PrivateWithdrawalsPreparePostRequest**](PrivateWithdrawalsPreparePostRequest.md) |  | [required] |

### Return type

[**models::PrivateWithdrawalsPreparePost200Response**](_private_withdrawals_prepare_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

