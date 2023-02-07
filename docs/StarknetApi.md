# \StarknetApi

All URIs are relative to *https://api.prod.gallop.run/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_skn_marketplace_data**](StarknetApi.md#get_skn_marketplace_data) | **POST** /data/skn/getMarketplaceData | Gallop Marketplace Data
[**get_skn_marketplace_floor_price**](StarknetApi.md#get_skn_marketplace_floor_price) | **POST** /data/skn/getMarketplaceFloorPrice | Marketplace Floor Price by Collection



## get_skn_marketplace_data

> get_skn_marketplace_data(get_skn_marketplace_data_request)
Gallop Marketplace Data

Lists marketplace data from contract address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_skn_marketplace_data_request** | Option<[**GetSknMarketplaceDataRequest**](GetSknMarketplaceDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_skn_marketplace_floor_price

> get_skn_marketplace_floor_price(get_skn_marketplace_floor_price_request)
Marketplace Floor Price by Collection

Returns current floor price for all collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_skn_marketplace_floor_price_request** | Option<[**GetSknMarketplaceFloorPriceRequest**](GetSknMarketplaceFloorPriceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

