# PrivateWithdrawalsDetailGet200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**currency** | [**models::PublicCurrenciesGet200ResponseInner**](_public_currencies_get_200_response_inner.md) |  | 
**network** | [**models::PublicNetworksGet200ResponseInner**](_public_networks_get_200_response_inner.md) |  | 
**balance** | **String** |  | 
**fee_usd** | **String** |  | 
**fee_in_currency** | **String** |  | 
**max_withdraw_exchange** | **String** |  | 
**available_hot** | **std::collections::HashMap<String, String>** |  | 
**available_hot_and_cold** | **std::collections::HashMap<String, String>** |  | 
**address_regex** | **String** |  | 
**min_withdraw_amount** | **String** |  | 
**is_memo_required** | Option<[**serde_json::Value**](.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


