# \EthereumApi

All URIs are relative to *https://api.prod.gallop.run/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_eth_collection_floor_price_ohlc**](EthereumApi.md#get_eth_collection_floor_price_ohlc) | **POST** /analytics/eth/getCollectionFloorPriceOHLC | Intraday Marketplace Floor Price by Collection
[**get_eth_collection_forecasts**](EthereumApi.md#get_eth_collection_forecasts) | **POST** /insights/eth/getCollectionForecasts | Price Forecast by Collection
[**get_eth_collection_listings_ohlc**](EthereumApi.md#get_eth_collection_listings_ohlc) | **POST** /analytics/eth/getCollectionListingsOHLC | Collection Floor Price and Listings Candlesticks
[**get_eth_collection_owners**](EthereumApi.md#get_eth_collection_owners) | **POST** /data/eth/getCollectionOwners | Wallet Owners by Collection
[**get_eth_collection_price_diff**](EthereumApi.md#get_eth_collection_price_diff) | **POST** /analytics/eth/getCollectionPriceDiff | Price Differentiation by Trait
[**get_eth_collection_sales_ohlc**](EthereumApi.md#get_eth_collection_sales_ohlc) | **POST** /analytics/eth/getCollectionSalesOHLC | Collection Sales Price Candlesticks
[**get_eth_collection_summary**](EthereumApi.md#get_eth_collection_summary) | **POST** /analytics/eth/getCollectionSummary | Summary Statistics by Collection
[**get_eth_collection_transactions**](EthereumApi.md#get_eth_collection_transactions) | **POST** /data/eth/getCollectionTransactions | Transactions by Collection
[**get_eth_collections**](EthereumApi.md#get_eth_collections) | **POST** /data/eth/getCollections | Aggregated Collections Supported by Gallop
[**get_eth_custom_collection_risk**](EthereumApi.md#get_eth_custom_collection_risk) | **POST** /insights/eth/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
[**get_eth_custom_token_risk**](EthereumApi.md#get_eth_custom_token_risk) | **POST** /insights/eth/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
[**get_eth_default_collection_risk**](EthereumApi.md#get_eth_default_collection_risk) | **POST** /insights/eth/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
[**get_eth_default_token_risk**](EthereumApi.md#get_eth_default_token_risk) | **POST** /insights/eth/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
[**get_eth_ens_lookup**](EthereumApi.md#get_eth_ens_lookup) | **POST** /data/eth/getEnsLookup | ENS Lookup
[**get_eth_historical_events**](EthereumApi.md#get_eth_historical_events) | **POST** /data/eth/getHistoricalEvents | Marketplace Activity by Collection
[**get_eth_historical_transactions**](EthereumApi.md#get_eth_historical_transactions) | **POST** /data/eth/getHistoricalTransactions | Historical Transactions by Collection
[**get_eth_leader_board**](EthereumApi.md#get_eth_leader_board) | **POST** /analytics/eth/getLeaderBoard | Ethereum Leaderboard by Collection
[**get_eth_marketplace_data**](EthereumApi.md#get_eth_marketplace_data) | **POST** /data/eth/getMarketplaceData | Collection Summary by Marketplace
[**get_eth_marketplace_floor_price**](EthereumApi.md#get_eth_marketplace_floor_price) | **POST** /data/eth/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
[**get_eth_marketplace_trait_data**](EthereumApi.md#get_eth_marketplace_trait_data) | **POST** /data/eth/getMarketplaceTraitData | Collection Listings by Trait & Marketplace
[**get_eth_rarity**](EthereumApi.md#get_eth_rarity) | **POST** /analytics/eth/getRarity | Token Rarity by Collection
[**get_eth_token_appraisal**](EthereumApi.md#get_eth_token_appraisal) | **POST** /insights/eth/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
[**get_eth_token_forecasts**](EthereumApi.md#get_eth_token_forecasts) | **POST** /insights/eth/getTokenForecasts | Price Forecast by Token
[**get_eth_token_summary**](EthereumApi.md#get_eth_token_summary) | **POST** /analytics/eth/getTokenSummary | Summary Statistics by Token
[**get_eth_token_transactions**](EthereumApi.md#get_eth_token_transactions) | **POST** /data/eth/getTokenTransactions | Transactions by Token
[**get_eth_tokens**](EthereumApi.md#get_eth_tokens) | **POST** /data/eth/getTokens | Tokens by Collection
[**get_eth_wallet_labels**](EthereumApi.md#get_eth_wallet_labels) | **POST** /insights/eth/getWalletLabels | Wallet Activity Labels
[**get_eth_wallet_nfts**](EthereumApi.md#get_eth_wallet_nfts) | **POST** /data/eth/getWalletNFTs | Tokens Owned by Wallet
[**get_eth_wallet_transactions**](EthereumApi.md#get_eth_wallet_transactions) | **POST** /data/eth/getWalletTransactions | Historical Token Transactions by Wallet
[**get_eth_wash_trade**](EthereumApi.md#get_eth_wash_trade) | **POST** /analytics/eth/getWashTrade | Wash Trades by Transaction
[**get_eth_wash_transactions**](EthereumApi.md#get_eth_wash_transactions) | **POST** /analytics/eth/getWashTransactions | Wash Trades by Collection



## get_eth_collection_floor_price_ohlc

> get_eth_collection_floor_price_ohlc(get_eth_collection_floor_price_ohlc_request)
Intraday Marketplace Floor Price by Collection

Returns intraday floor price for a given collection by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_floor_price_ohlc_request** | Option<[**GetEthCollectionFloorPriceOhlcRequest**](GetEthCollectionFloorPriceOhlcRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_forecasts

> get_eth_collection_forecasts(get_eth_collection_forecasts_request)
Price Forecast by Collection

Returns price, return, and volatility forecast for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_forecasts_request** | Option<[**GetEthCollectionForecastsRequest**](GetEthCollectionForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_listings_ohlc

> get_eth_collection_listings_ohlc(get_eth_collection_listings_ohlc_request)
Collection Floor Price and Listings Candlesticks

Returns historical floor price or more extensive open / high / floor / close candlesticks for collection listings at marketplaces at a selected time interval, as well as the number of active listings, the number of unique owners and the average age of open listings

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_listings_ohlc_request** | Option<[**GetEthCollectionListingsOhlcRequest**](GetEthCollectionListingsOhlcRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_owners

> get_eth_collection_owners(get_eth_collection_owners_request)
Wallet Owners by Collection

Returns all wallet owners for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_owners_request** | Option<[**GetEthCollectionOwnersRequest**](GetEthCollectionOwnersRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_price_diff

> get_eth_collection_price_diff(get_eth_collection_price_diff_request)
Price Differentiation by Trait

Returns how trait differentiates price for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_price_diff_request** | Option<[**GetEthCollectionPriceDiffRequest**](GetEthCollectionPriceDiffRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_sales_ohlc

> get_eth_collection_sales_ohlc(get_eth_collection_sales_ohlc_request)
Collection Sales Price Candlesticks

Returns collection sales price open, high, low, close and volume at a selected time interval

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_sales_ohlc_request** | Option<[**GetEthCollectionSalesOhlcRequest**](GetEthCollectionSalesOhlcRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_summary

> get_eth_collection_summary(get_eth_collection_summary_request)
Summary Statistics by Collection

Returns summary analytics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_summary_request** | Option<[**GetEthCollectionSummaryRequest**](GetEthCollectionSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collection_transactions

> get_eth_collection_transactions(get_eth_collection_transactions_request)
Transactions by Collection

Returns all transactions for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collection_transactions_request** | Option<[**GetEthCollectionTransactionsRequest**](GetEthCollectionTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_collections

> get_eth_collections(get_eth_collections_request)
Aggregated Collections Supported by Gallop

Returns all Gallop aggregated collections

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_collections_request** | Option<[**GetEthCollectionsRequest**](GetEthCollectionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_custom_collection_risk

> get_eth_custom_collection_risk(get_eth_custom_collection_risk_request)
Custom Volatility & Risk Metrics by Collection

Returns summary of customizable volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_custom_collection_risk_request** | Option<[**GetEthCustomCollectionRiskRequest**](GetEthCustomCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_custom_token_risk

> get_eth_custom_token_risk(get_eth_custom_token_risk_request)
Custom Volatility & Risk Metrics by Token

Returns summary of customizable volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_custom_token_risk_request** | Option<[**GetEthCustomTokenRiskRequest**](GetEthCustomTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_default_collection_risk

> get_eth_default_collection_risk(get_eth_default_collection_risk_request)
Default Volatility & Risk Metrics by Collection

Returns summary of default volatility and risk metrics for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_default_collection_risk_request** | Option<[**GetEthDefaultCollectionRiskRequest**](GetEthDefaultCollectionRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_default_token_risk

> get_eth_default_token_risk(get_eth_default_token_risk_request)
Default Volatility & Risk Metrics by Token

Returns summary of default volatility and risk metrics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_default_token_risk_request** | Option<[**GetEthDefaultTokenRiskRequest**](GetEthDefaultTokenRiskRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_ens_lookup

> get_eth_ens_lookup(get_eth_ens_lookup_request)
ENS Lookup

Returns Ethereum Name Service data for a given wallet address

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_ens_lookup_request** | Option<[**GetEthEnsLookupRequest**](GetEthEnsLookupRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_historical_events

> get_eth_historical_events(get_eth_historical_events_request)
Marketplace Activity by Collection

Returns marketplace activity for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_historical_events_request** | Option<[**GetEthHistoricalEventsRequest**](GetEthHistoricalEventsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_historical_transactions

> get_eth_historical_transactions(get_eth_historical_transactions_request)
Historical Transactions by Collection

Returns all transactions for a given collection in bulk

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_historical_transactions_request** | Option<[**GetEthHistoricalTransactionsRequest**](GetEthHistoricalTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_leader_board

> get_eth_leader_board(get_eth_leader_board_request)
Ethereum Leaderboard by Collection

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


## get_eth_marketplace_data

> get_eth_marketplace_data(get_eth_marketplace_data_request)
Collection Summary by Marketplace

Returns summary statistics for a collection by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_marketplace_data_request** | Option<[**GetEthMarketplaceDataRequest**](GetEthMarketplaceDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_marketplace_floor_price

> get_eth_marketplace_floor_price(get_eth_marketplace_floor_price_request)
Marketplace Floor Price by Collection

Returns current floor price for all collections by marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_marketplace_floor_price_request** | Option<[**GetEthMarketplaceFloorPriceRequest**](GetEthMarketplaceFloorPriceRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_marketplace_trait_data

> get_eth_marketplace_trait_data(get_eth_marketplace_trait_data_request)
Collection Listings by Trait & Marketplace

Returns listing statistics for a collection by trait and marketplace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_marketplace_trait_data_request** | Option<[**GetEthMarketplaceTraitDataRequest**](GetEthMarketplaceTraitDataRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_rarity

> get_eth_rarity(get_eth_rarity_request)
Token Rarity by Collection

Returns rarity by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_rarity_request** | Option<[**GetEthRarityRequest**](GetEthRarityRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_token_appraisal

> get_eth_token_appraisal(get_eth_token_appraisal_request)
Liquidation & Appraisal Estimate by Token

Get estimates of appraisal and liquidation values for a set of tokens. The app returns nowcasts by default, but if provided a `horizon` and `frequency`, it will return forcasts for `horizon` periods out at interval `frequency`. The app is does not deliver individualized financial advice, but merely provides analytical estimates of token appraisal and liquidation values

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_token_appraisal_request** | Option<[**GetEthTokenAppraisalRequest**](GetEthTokenAppraisalRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_token_forecasts

> get_eth_token_forecasts(get_eth_token_forecasts_request)
Price Forecast by Token

Returns price, return, and volatility forecast for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_token_forecasts_request** | Option<[**GetEthTokenForecastsRequest**](GetEthTokenForecastsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_token_summary

> get_eth_token_summary(get_eth_token_summary_request)
Summary Statistics by Token

Returns summary analytics for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_token_summary_request** | Option<[**GetEthTokenSummaryRequest**](GetEthTokenSummaryRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_token_transactions

> get_eth_token_transactions(get_eth_token_transactions_request)
Transactions by Token

Returns all transactions for a given token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_token_transactions_request** | Option<[**GetEthTokenTransactionsRequest**](GetEthTokenTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_tokens

> get_eth_tokens(get_eth_tokens_request)
Tokens by Collection

Returns all tokens for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_tokens_request** | Option<[**GetEthTokensRequest**](GetEthTokensRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_wallet_labels

> get_eth_wallet_labels(get_eth_wallet_labels_request)
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


## get_eth_wallet_nfts

> get_eth_wallet_nfts(get_eth_wallet_nfts_request)
Tokens Owned by Wallet

Returns all tokens owned for a given wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_wallet_nfts_request** | Option<[**GetEthWalletNftsRequest**](GetEthWalletNftsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_wallet_transactions

> get_eth_wallet_transactions(get_eth_wallet_transactions_request)
Historical Token Transactions by Wallet

Returns all historical token transactions for a given wallet

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_wallet_transactions_request** | Option<[**GetEthWalletTransactionsRequest**](GetEthWalletTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_wash_trade

> get_eth_wash_trade(get_eth_wash_trade_request)
Wash Trades by Transaction

Returns suspected wash trades for a given transaction hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_wash_trade_request** | Option<[**GetEthWashTradeRequest**](GetEthWashTradeRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_eth_wash_transactions

> get_eth_wash_transactions(get_eth_wash_transactions_request)
Wash Trades by Collection

Returns suspected wash trades by token for a given collection

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**get_eth_wash_transactions_request** | Option<[**GetEthWashTransactionsRequest**](GetEthWashTransactionsRequest.md)> |  |  |

### Return type

 (empty response body)

### Authorization

[api_key](../README.md#api_key)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

