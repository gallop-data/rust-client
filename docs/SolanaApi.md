# \SolanaApi

All URIs are relative to *https://api.prod.gallop.run/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_sol_account_nfts**](SolanaApi.md#get_sol_account_nfts) | **POST** /data/sol/getAccountNFTs | Tokens Owned by Wallet
[**get_sol_collection_forecasts**](SolanaApi.md#get_sol_collection_forecasts) | **POST** /insights/sol/getCollectionForecasts | Price Forecast by Collection
[**get_sol_collection_price_diff**](SolanaApi.md#get_sol_collection_price_diff) | **POST** /analytics/sol/getCollectionPriceDiff | Price Differentiation by Trait
[**get_sol_collection_sales_ohlc**](SolanaApi.md#get_sol_collection_sales_ohlc) | **POST** /analytics/sol/getCollectionSalesOHLC | Collection Sales Price Candlesticks
[**get_sol_collection_summary**](SolanaApi.md#get_sol_collection_summary) | **POST** /analytics/sol/getCollectionSummary | Summary Statistics by Collection
[**get_sol_collection_traits**](SolanaApi.md#get_sol_collection_traits) | **POST** /data/sol/getCollectionTraits | Traits by Collection
[**get_sol_collection_transactions**](SolanaApi.md#get_sol_collection_transactions) | **POST** /data/sol/getCollectionTransactions | Transactions by Collections
[**get_sol_collections**](SolanaApi.md#get_sol_collections) | **POST** /data/sol/getCollections | Aggregated Collections Supported by Gallop
[**get_sol_custom_collection_risk**](SolanaApi.md#get_sol_custom_collection_risk) | **POST** /insights/sol/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
[**get_sol_custom_token_risk**](SolanaApi.md#get_sol_custom_token_risk) | **POST** /insights/sol/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
[**get_sol_default_collection_risk**](SolanaApi.md#get_sol_default_collection_risk) | **POST** /insights/sol/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
[**get_sol_default_token_risk**](SolanaApi.md#get_sol_default_token_risk) | **POST** /insights/sol/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
[**get_sol_historical_transactions**](SolanaApi.md#get_sol_historical_transactions) | **POST** /data/sol/getHistoricalTransactions | Historical Transactions by Collection
[**get_sol_marketplace_data**](SolanaApi.md#get_sol_marketplace_data) | **POST** /data/sol/getMarketplaceData | Collection Summary by Marketplace
[**get_sol_marketplace_floor_price**](SolanaApi.md#get_sol_marketplace_floor_price) | **POST** /data/sol/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
[**get_sol_marketplace_trait_data**](SolanaApi.md#get_sol_marketplace_trait_data) | **POST** /data/sol/getMarketplaceTraitData | Collection Listings by Trait & Marketplace
[**get_sol_nft_account**](SolanaApi.md#get_sol_nft_account) | **POST** /data/sol/getNFTAccount | Wallet Owners by Token
[**get_sol_rarity**](SolanaApi.md#get_sol_rarity) | **POST** /analytics/sol/getRarity | Token Rarity by Collection
[**get_sol_token_appraisal**](SolanaApi.md#get_sol_token_appraisal) | **POST** /insights/sol/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
[**get_sol_token_forecasts**](SolanaApi.md#get_sol_token_forecasts) | **POST** /insights/sol/getTokenForecasts | Price Forecast by Token
[**get_sol_token_summary**](SolanaApi.md#get_sol_token_summary) | **POST** /analytics/sol/getTokenSummary | Summary Statistics by Token
[**get_sol_token_transactions**](SolanaApi.md#get_sol_token_transactions) | **POST** /data/sol/getTokenTransactions | Transactions by Token
[**get_sol_tokens**](SolanaApi.md#get_sol_tokens) | **POST** /data/sol/getTokens | Tokens by Collection
[**get_sol_wash_trade**](SolanaApi.md#get_sol_wash_trade) | **POST** /analytics/sol/getWashTrade | Wash Trades by Transaction
[**get_sol_wash_transactions**](SolanaApi.md#get_sol_wash_transactions) | **POST** /analytics/sol/getWashTransactions | Wash Trades by Collection



## get_sol_account_nfts

> get_sol_account_nfts(get_sol_account_nfts_request)
Tokens Owned by Wallet

Returns all tokens owned for a given wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_account_nfts_request** | Option<[**GetSolAccountNftsRequest**](GetSolAccountNftsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_forecasts

> get_sol_collection_forecasts(get_sol_collection_forecasts_request)
Price Forecast by Collection

Returns price, return, and volatility forecast for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_forecasts_request** | Option<[**GetSolCollectionForecastsRequest**](GetSolCollectionForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_price_diff

> get_sol_collection_price_diff(get_sol_collection_price_diff_request)
Price Differentiation by Trait

Returns how trait differentiates price for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_price_diff_request** | Option<[**GetSolCollectionPriceDiffRequest**](GetSolCollectionPriceDiffRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_sales_ohlc

> get_sol_collection_sales_ohlc(get_sol_collection_sales_ohlc_request)
Collection Sales Price Candlesticks

Returns collection sales price open, high, low, close and volume at a selected time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_sales_ohlc_request** | Option<[**GetSolCollectionSalesOhlcRequest**](GetSolCollectionSalesOhlcRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_summary

> get_sol_collection_summary(get_sol_collection_summary_request)
Summary Statistics by Collection

Returns summary analytics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_summary_request** | Option<[**GetSolCollectionSummaryRequest**](GetSolCollectionSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_traits

> get_sol_collection_traits(get_sol_collection_traits_request)
Traits by Collection

Returns a list of traits for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_traits_request** | Option<[**GetSolCollectionTraitsRequest**](GetSolCollectionTraitsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collection_transactions

> get_sol_collection_transactions(get_sol_collection_transactions_request)
Transactions by Collections

Returns all transactions for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collection_transactions_request** | Option<[**GetSolCollectionTransactionsRequest**](GetSolCollectionTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_collections

> get_sol_collections(get_sol_collections_request)
Aggregated Collections Supported by Gallop

Returns all Gallop aggregated collections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_collections_request** | Option<[**GetSolCollectionsRequest**](GetSolCollectionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_custom_collection_risk

> get_sol_custom_collection_risk(get_sol_custom_collection_risk_request)
Custom Volatility & Risk Metrics by Collection

Returns summary of customizable volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_custom_collection_risk_request** | Option<[**GetSolCustomCollectionRiskRequest**](GetSolCustomCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_custom_token_risk

> get_sol_custom_token_risk(get_sol_custom_token_risk_request)
Custom Volatility & Risk Metrics by Token

Returns summary of customizable volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_custom_token_risk_request** | Option<[**GetSolCustomTokenRiskRequest**](GetSolCustomTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_default_collection_risk

> get_sol_default_collection_risk(get_sol_default_collection_risk_request)
Default Volatility & Risk Metrics by Collection

Returns summary of default volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_default_collection_risk_request** | Option<[**GetSolDefaultCollectionRiskRequest**](GetSolDefaultCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_default_token_risk

> get_sol_default_token_risk(get_sol_default_token_risk_request)
Default Volatility & Risk Metrics by Token

Returns summary of default volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_default_token_risk_request** | Option<[**GetSolDefaultTokenRiskRequest**](GetSolDefaultTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_historical_transactions

> get_sol_historical_transactions(get_sol_historical_transactions_request)
Historical Transactions by Collection

Returns all transactions for a given collection in bulk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_historical_transactions_request** | Option<[**GetSolHistoricalTransactionsRequest**](GetSolHistoricalTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_marketplace_data

> get_sol_marketplace_data(get_sol_marketplace_data_request)
Collection Summary by Marketplace

Returns summary statistics for collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_marketplace_data_request** | Option<[**GetSolMarketplaceDataRequest**](GetSolMarketplaceDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_marketplace_floor_price

> get_sol_marketplace_floor_price(get_sol_marketplace_floor_price_request)
Marketplace Floor Price by Collection

Returns current floor price for all collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_marketplace_floor_price_request** | Option<[**GetSolMarketplaceFloorPriceRequest**](GetSolMarketplaceFloorPriceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_marketplace_trait_data

> get_sol_marketplace_trait_data(get_sol_marketplace_trait_data_request)
Collection Listings by Trait & Marketplace

Returns listing statistics for a collection by trait and marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_marketplace_trait_data_request** | Option<[**GetSolMarketplaceTraitDataRequest**](GetSolMarketplaceTraitDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_nft_account

> get_sol_nft_account(get_sol_nft_account_request)
Wallet Owners by Token

Returns all wallet owners for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_nft_account_request** | Option<[**GetSolNftAccountRequest**](GetSolNftAccountRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_rarity

> get_sol_rarity(get_sol_rarity_request)
Token Rarity by Collection

Returns rarity by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_rarity_request** | Option<[**GetSolRarityRequest**](GetSolRarityRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_token_appraisal

> get_sol_token_appraisal(get_sol_token_appraisal_request)
Liquidation & Appraisal Estimate by Token

Get estimates of appraisal and liquidation values for a set of tokens. The app returns nowcasts by default, but if provided a `horizon` and `frequency`, it will return forcasts for `horizon` periods out at interval `frequency`. The app is does not deliver individualized financial advice, but merely provides analytical estimates of token appraisal and liquidation values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_token_appraisal_request** | Option<[**GetSolTokenAppraisalRequest**](GetSolTokenAppraisalRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_token_forecasts

> get_sol_token_forecasts(get_sol_token_forecasts_request)
Price Forecast by Token

Returns price, return, and volatility forecast for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_token_forecasts_request** | Option<[**GetSolTokenForecastsRequest**](GetSolTokenForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_token_summary

> get_sol_token_summary(get_sol_token_summary_request)
Summary Statistics by Token

Returns summary analytics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_token_summary_request** | Option<[**GetSolTokenSummaryRequest**](GetSolTokenSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_token_transactions

> get_sol_token_transactions(get_sol_token_transactions_request)
Transactions by Token

Returns all transactions for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_token_transactions_request** | Option<[**GetSolTokenTransactionsRequest**](GetSolTokenTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_tokens

> get_sol_tokens(get_sol_tokens_request)
Tokens by Collection

Returns all tokens for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_tokens_request** | Option<[**GetSolTokensRequest**](GetSolTokensRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_wash_trade

> get_sol_wash_trade(get_sol_wash_trade_request)
Wash Trades by Transaction

Returns suspected wash trades for a given transaction hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_wash_trade_request** | Option<[**GetSolWashTradeRequest**](GetSolWashTradeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_sol_wash_transactions

> get_sol_wash_transactions(get_sol_wash_transactions_request)
Wash Trades by Collection

Returns suspected wash trades by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_sol_wash_transactions_request** | Option<[**GetSolWashTransactionsRequest**](GetSolWashTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

