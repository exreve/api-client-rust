# \PoolApi

All URIs are relative to *https://api.exkoin.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**private_pool_join_post**](PoolApi.md#private_pool_join_post) | **POST** /private/pool/join | 
[**private_pool_leave_post**](PoolApi.md#private_pool_leave_post) | **POST** /private/pool/leave | 
[**private_pool_task_get**](PoolApi.md#private_pool_task_get) | **GET** /private/pool/task | 
[**private_pool_tasks_get**](PoolApi.md#private_pool_tasks_get) | **GET** /private/pool/tasks | 
[**public_pools_get**](PoolApi.md#public_pools_get) | **GET** /public/pools | 



## private_pool_join_post

> models::PrivatePoolJoinPost200Response private_pool_join_post(private_pool_join_post_request)


Join a pool, it will deduct the currencies from your balance, and you will be credited the corresponding lp token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_pool_join_post_request** | [**PrivatePoolJoinPostRequest**](PrivatePoolJoinPostRequest.md) |  | [required] |

### Return type

[**models::PrivatePoolJoinPost200Response**](_private_pool_join_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_pool_leave_post

> models::PrivatePoolJoinPost200Response private_pool_leave_post(private_pool_leave_post_request)


Leave a pool, it consumes the lp token and credits your balance with the corresponding currencies

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**private_pool_leave_post_request** | [**PrivatePoolLeavePostRequest**](PrivatePoolLeavePostRequest.md) |  | [required] |

### Return type

[**models::PrivatePoolJoinPost200Response**](_private_pool_join_post_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_pool_task_get

> models::PrivatePoolTasksGet200ResponseInner private_pool_task_get(task_id)


get a pool task (join/leave) by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task_id** | **String** |  | [required] |

### Return type

[**models::PrivatePoolTasksGet200ResponseInner**](_private_pool_tasks_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## private_pool_tasks_get

> Vec<models::PrivatePoolTasksGet200ResponseInner> private_pool_tasks_get(pool_id, r#type, status, page, limit, start_time, end_time)


Get the user pool tasks (join/leave)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pool_id** | Option<**String**> |  |  |
**r#type** | Option<**String**> |  |  |
**status** | Option<**String**> |  |  |
**page** | Option<**f64**> |  |  |
**limit** | Option<**f64**> |  |  |
**start_time** | Option<**f64**> |  |  |
**end_time** | Option<**f64**> |  |  |

### Return type

[**Vec<models::PrivatePoolTasksGet200ResponseInner>**](_private_pool_tasks_get_200_response_inner.md)

### Authorization

[apiKey](../README.md#apiKey), [signature](../README.md#signature), [basicAuth](../README.md#basicAuth), [sessionToken](../README.md#sessionToken), [nonce](../README.md#nonce)

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

