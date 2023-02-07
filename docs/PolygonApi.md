# \PolygonApi

All URIs are relative to *https://api.prod.gallop.run/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pol_collection_forecasts**](PolygonApi.md#get_pol_collection_forecasts) | **POST** /insights/pol/getCollectionForecasts | Price Forecast by Collection
[**get_pol_collection_owners**](PolygonApi.md#get_pol_collection_owners) | **POST** /data/pol/getCollectionOwners | Wallet Owners by Collection
[**get_pol_collection_price_diff**](PolygonApi.md#get_pol_collection_price_diff) | **POST** /analytics/pol/getCollectionPriceDiff | Price Differentiation by Trait
[**get_pol_collection_sales_ohlc**](PolygonApi.md#get_pol_collection_sales_ohlc) | **POST** /analytics/pol/getCollectionSalesOHLC | Collection Sales Price Candlesticks
[**get_pol_collection_summary**](PolygonApi.md#get_pol_collection_summary) | **POST** /analytics/pol/getCollectionSummary | Summary Statistics by Collection
[**get_pol_collection_traits**](PolygonApi.md#get_pol_collection_traits) | **POST** /data/pol/getCollectionTraits | Traits by Collection
[**get_pol_collection_transactions**](PolygonApi.md#get_pol_collection_transactions) | **POST** /data/pol/getCollectionTransactions | Transactions by Collection
[**get_pol_collections**](PolygonApi.md#get_pol_collections) | **POST** /data/pol/getCollections | Aggregated Collections Supported by Gallop
[**get_pol_custom_collection_risk**](PolygonApi.md#get_pol_custom_collection_risk) | **POST** /insights/pol/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
[**get_pol_custom_token_risk**](PolygonApi.md#get_pol_custom_token_risk) | **POST** /insights/pol/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
[**get_pol_default_collection_risk**](PolygonApi.md#get_pol_default_collection_risk) | **POST** /insights/pol/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
[**get_pol_default_token_risk**](PolygonApi.md#get_pol_default_token_risk) | **POST** /insights/pol/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
[**get_pol_historical_transactions**](PolygonApi.md#get_pol_historical_transactions) | **POST** /data/pol/getHistoricalTransactions | Historical Transactions by Collection
[**get_pol_leader_board**](PolygonApi.md#get_pol_leader_board) | **POST** /analytics/pol/getLeaderBoard | Polygon Leaderboard by Collection
[**get_pol_marketplace_data**](PolygonApi.md#get_pol_marketplace_data) | **POST** /data/pol/getMarketplaceData | Collection Summary by Marketplace
[**get_pol_marketplace_floor_price**](PolygonApi.md#get_pol_marketplace_floor_price) | **POST** /data/pol/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
[**get_pol_rarity**](PolygonApi.md#get_pol_rarity) | **POST** /analytics/pol/getRarity | Token Rarity by Collection
[**get_pol_token_appraisal**](PolygonApi.md#get_pol_token_appraisal) | **POST** /insights/pol/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
[**get_pol_token_forecasts**](PolygonApi.md#get_pol_token_forecasts) | **POST** /insights/pol/getTokenForecasts | Price Forecast by Token
[**get_pol_token_summary**](PolygonApi.md#get_pol_token_summary) | **POST** /analytics/pol/getTokenSummary | Summary Statistics by Token
[**get_pol_token_transactions**](PolygonApi.md#get_pol_token_transactions) | **POST** /data/pol/getTokenTransactions | Transactions by Token
[**get_pol_tokens**](PolygonApi.md#get_pol_tokens) | **POST** /data/pol/getTokens | Tokens by Collection
[**get_pol_wallet_labels**](PolygonApi.md#get_pol_wallet_labels) | **POST** /insights/pol/getWalletLabels | Wallet Activity Labels
[**get_pol_wallet_nfts**](PolygonApi.md#get_pol_wallet_nfts) | **POST** /data/pol/getWalletNFTs | Tokens Owned by Wallet
[**get_pol_wallet_transactions**](PolygonApi.md#get_pol_wallet_transactions) | **POST** /data/pol/getWalletTransactions | Historical Token Transactions by Wallet
[**get_pol_wash_trade**](PolygonApi.md#get_pol_wash_trade) | **POST** /analytics/pol/getWashTrade | Wash Trades by Transaction
[**get_pol_wash_transactions**](PolygonApi.md#get_pol_wash_transactions) | **POST** /analytics/pol/getWashTransactions | Wash Trades by Collection



## get_pol_collection_forecasts

> get_pol_collection_forecasts(get_pol_collection_forecasts_request)
Price Forecast by Collection

Returns price, return, and volatility forecast for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_forecasts_request** | Option<[**GetPolCollectionForecastsRequest**](GetPolCollectionForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_owners

> get_pol_collection_owners(get_pol_collection_owners_request)
Wallet Owners by Collection

Returns all wallet owners for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_owners_request** | Option<[**GetPolCollectionOwnersRequest**](GetPolCollectionOwnersRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_price_diff

> get_pol_collection_price_diff(get_pol_collection_price_diff_request)
Price Differentiation by Trait

Returns how trait differentiates price for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_price_diff_request** | Option<[**GetPolCollectionPriceDiffRequest**](GetPolCollectionPriceDiffRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_sales_ohlc

> get_pol_collection_sales_ohlc(get_pol_collection_sales_ohlc_request)
Collection Sales Price Candlesticks

Returns collection sales price open, high, low, close and volume at a selected time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_sales_ohlc_request** | Option<[**GetPolCollectionSalesOhlcRequest**](GetPolCollectionSalesOhlcRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_summary

> get_pol_collection_summary(get_pol_collection_summary_request)
Summary Statistics by Collection

Returns summary analytics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_summary_request** | Option<[**GetPolCollectionSummaryRequest**](GetPolCollectionSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_traits

> get_pol_collection_traits(get_pol_collection_traits_request)
Traits by Collection

Returns a list of traits for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_traits_request** | Option<[**GetPolCollectionTraitsRequest**](GetPolCollectionTraitsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collection_transactions

> get_pol_collection_transactions(get_pol_collection_transactions_request)
Transactions by Collection

Returns all transactions for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collection_transactions_request** | Option<[**GetPolCollectionTransactionsRequest**](GetPolCollectionTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_collections

> get_pol_collections(get_pol_collections_request)
Aggregated Collections Supported by Gallop

Returns all Gallop aggregated collections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_collections_request** | Option<[**GetPolCollectionsRequest**](GetPolCollectionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_custom_collection_risk

> get_pol_custom_collection_risk(get_pol_custom_collection_risk_request)
Custom Volatility & Risk Metrics by Collection

Returns summary of customizable volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_custom_collection_risk_request** | Option<[**GetPolCustomCollectionRiskRequest**](GetPolCustomCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_custom_token_risk

> get_pol_custom_token_risk(get_pol_custom_token_risk_request)
Custom Volatility & Risk Metrics by Token

Returns summary of customizable volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_custom_token_risk_request** | Option<[**GetPolCustomTokenRiskRequest**](GetPolCustomTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_default_collection_risk

> get_pol_default_collection_risk(get_pol_default_collection_risk_request)
Default Volatility & Risk Metrics by Collection

Returns summary of default volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_default_collection_risk_request** | Option<[**GetPolDefaultCollectionRiskRequest**](GetPolDefaultCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_default_token_risk

> get_pol_default_token_risk(get_pol_default_token_risk_request)
Default Volatility & Risk Metrics by Token

Returns summary of default volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_default_token_risk_request** | Option<[**GetPolDefaultTokenRiskRequest**](GetPolDefaultTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_historical_transactions

> get_pol_historical_transactions(get_pol_historical_transactions_request)
Historical Transactions by Collection

Returns all transactions for a given collection in bulk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_historical_transactions_request** | Option<[**GetPolHistoricalTransactionsRequest**](GetPolHistoricalTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_leader_board

> get_pol_leader_board(get_eth_leader_board_request)
Polygon Leaderboard by Collection

Returns top collections by volume transaction volume or sales count

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_leader_board_request** | Option<[**GetEthLeaderBoardRequest**](GetEthLeaderBoardRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_marketplace_data

> get_pol_marketplace_data(get_pol_marketplace_data_request)
Collection Summary by Marketplace

Returns summary statistics for collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_marketplace_data_request** | Option<[**GetPolMarketplaceDataRequest**](GetPolMarketplaceDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_marketplace_floor_price

> get_pol_marketplace_floor_price(get_pol_marketplace_floor_price_request)
Marketplace Floor Price by Collection

Returns current floor price for all collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_marketplace_floor_price_request** | Option<[**GetPolMarketplaceFloorPriceRequest**](GetPolMarketplaceFloorPriceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_rarity

> get_pol_rarity(get_pol_rarity_request)
Token Rarity by Collection

Returns rarity by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_rarity_request** | Option<[**GetPolRarityRequest**](GetPolRarityRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_token_appraisal

> get_pol_token_appraisal(get_pol_token_appraisal_request)
Liquidation & Appraisal Estimate by Token

Get estimates of appraisal and liquidation values for a set of tokens. The app returns nowcasts by default, but if provided a `horizon` and `frequency`, it will return forcasts for `horizon` periods out at interval `frequency`. The app is does not deliver individualized financial advice, but merely provides analytical estimates of token appraisal and liquidation values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_token_appraisal_request** | Option<[**GetPolTokenAppraisalRequest**](GetPolTokenAppraisalRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_token_forecasts

> get_pol_token_forecasts(get_pol_token_forecasts_request)
Price Forecast by Token

Returns price, return, and volatility forecast for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_token_forecasts_request** | Option<[**GetPolTokenForecastsRequest**](GetPolTokenForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_token_summary

> get_pol_token_summary(get_pol_token_summary_request)
Summary Statistics by Token

Returns summary analytics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_token_summary_request** | Option<[**GetPolTokenSummaryRequest**](GetPolTokenSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_token_transactions

> get_pol_token_transactions(get_pol_token_transactions_request)
Transactions by Token

Returns all transactions for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_token_transactions_request** | Option<[**GetPolTokenTransactionsRequest**](GetPolTokenTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_tokens

> get_pol_tokens(get_pol_tokens_request)
Tokens by Collection

Returns all tokens for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_tokens_request** | Option<[**GetPolTokensRequest**](GetPolTokensRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_wallet_labels

> get_pol_wallet_labels(get_eth_wallet_labels_request)
Wallet Activity Labels

Classifies a wallet's behaviour according to its on-chain activity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_wallet_labels_request** | Option<[**GetEthWalletLabelsRequest**](GetEthWalletLabelsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_wallet_nfts

> get_pol_wallet_nfts(get_pol_wallet_nfts_request)
Tokens Owned by Wallet

Returns all tokens owned for a given wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_wallet_nfts_request** | Option<[**GetPolWalletNftsRequest**](GetPolWalletNftsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_wallet_transactions

> get_pol_wallet_transactions(get_pol_wallet_transactions_request)
Historical Token Transactions by Wallet

Returns all historical token transactions for a given wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_wallet_transactions_request** | Option<[**GetPolWalletTransactionsRequest**](GetPolWalletTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_wash_trade

> get_pol_wash_trade(get_pol_wash_trade_request)
Wash Trades by Transaction

Returns suspected wash trades for a given transaction hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_wash_trade_request** | Option<[**GetPolWashTradeRequest**](GetPolWashTradeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pol_wash_transactions

> get_pol_wash_transactions(get_pol_wash_transactions_request)
Wash Trades by Collection

Returns suspected wash trades by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_pol_wash_transactions_request** | Option<[**GetPolWashTransactionsRequest**](GetPolWashTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

