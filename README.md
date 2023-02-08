# Rust API client for openapi

Data and insights APIs, webooks, and dashboards enabling businesses to launch tokenized products in seconds.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 1.0.0
- Package version: 1.0.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.prod.gallop.run/v1*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*EthereumApi* | [**get_eth_collection_floor_price_ohlc**](docs/EthereumApi.md#get_eth_collection_floor_price_ohlc) | **POST** /analytics/eth/getCollectionFloorPriceOHLC | Intraday Marketplace Floor Price by Collection
*EthereumApi* | [**get_eth_collection_forecasts**](docs/EthereumApi.md#get_eth_collection_forecasts) | **POST** /insights/eth/getCollectionForecasts | Price Forecast by Collection
*EthereumApi* | [**get_eth_collection_listings_ohlc**](docs/EthereumApi.md#get_eth_collection_listings_ohlc) | **POST** /analytics/eth/getCollectionListingsOHLC | Collection Price Listings Candlesticks
*EthereumApi* | [**get_eth_collection_owners**](docs/EthereumApi.md#get_eth_collection_owners) | **POST** /data/eth/getCollectionOwners | Wallet Owners by Collection
*EthereumApi* | [**get_eth_collection_price_diff**](docs/EthereumApi.md#get_eth_collection_price_diff) | **POST** /analytics/eth/getCollectionPriceDiff | Price Differentiation by Trait
*EthereumApi* | [**get_eth_collection_sales_ohlc**](docs/EthereumApi.md#get_eth_collection_sales_ohlc) | **POST** /analytics/eth/getCollectionSalesOHLC | Collection Sales Price Candlesticks
*EthereumApi* | [**get_eth_collection_summary**](docs/EthereumApi.md#get_eth_collection_summary) | **POST** /analytics/eth/getCollectionSummary | Summary Statistics by Collection
*EthereumApi* | [**get_eth_collection_transactions**](docs/EthereumApi.md#get_eth_collection_transactions) | **POST** /data/eth/getCollectionTransactions | Transactions by Collection
*EthereumApi* | [**get_eth_collections**](docs/EthereumApi.md#get_eth_collections) | **POST** /data/eth/getCollections | Aggregated Collections Supported by Gallop
*EthereumApi* | [**get_eth_custom_collection_risk**](docs/EthereumApi.md#get_eth_custom_collection_risk) | **POST** /insights/eth/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
*EthereumApi* | [**get_eth_custom_token_risk**](docs/EthereumApi.md#get_eth_custom_token_risk) | **POST** /insights/eth/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
*EthereumApi* | [**get_eth_default_collection_risk**](docs/EthereumApi.md#get_eth_default_collection_risk) | **POST** /insights/eth/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
*EthereumApi* | [**get_eth_default_token_risk**](docs/EthereumApi.md#get_eth_default_token_risk) | **POST** /insights/eth/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
*EthereumApi* | [**get_eth_ens_lookup**](docs/EthereumApi.md#get_eth_ens_lookup) | **POST** /data/eth/getEnsLookup | ENS Lookup
*EthereumApi* | [**get_eth_historical_events**](docs/EthereumApi.md#get_eth_historical_events) | **POST** /data/eth/getHistoricalEvents | Marketplace Activity by Collection
*EthereumApi* | [**get_eth_historical_transactions**](docs/EthereumApi.md#get_eth_historical_transactions) | **POST** /data/eth/getHistoricalTransactions | Historical Transactions by Collection
*EthereumApi* | [**get_eth_leader_board**](docs/EthereumApi.md#get_eth_leader_board) | **POST** /analytics/eth/getLeaderBoard | Ethereum Leaderboard by Collection
*EthereumApi* | [**get_eth_marketplace_data**](docs/EthereumApi.md#get_eth_marketplace_data) | **POST** /data/eth/getMarketplaceData | Collection Summary by Marketplace
*EthereumApi* | [**get_eth_marketplace_floor_price**](docs/EthereumApi.md#get_eth_marketplace_floor_price) | **POST** /data/eth/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
*EthereumApi* | [**get_eth_marketplace_trait_data**](docs/EthereumApi.md#get_eth_marketplace_trait_data) | **POST** /data/eth/getMarketplaceTraitData | Collection Listings by Trait & Marketplace
*EthereumApi* | [**get_eth_rarity**](docs/EthereumApi.md#get_eth_rarity) | **POST** /analytics/eth/getRarity | Token Rarity by Collection
*EthereumApi* | [**get_eth_token_appraisal**](docs/EthereumApi.md#get_eth_token_appraisal) | **POST** /insights/eth/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
*EthereumApi* | [**get_eth_token_forecasts**](docs/EthereumApi.md#get_eth_token_forecasts) | **POST** /insights/eth/getTokenForecasts | Price Forecast by Token
*EthereumApi* | [**get_eth_token_summary**](docs/EthereumApi.md#get_eth_token_summary) | **POST** /analytics/eth/getTokenSummary | Summary Statistics by Token
*EthereumApi* | [**get_eth_token_transactions**](docs/EthereumApi.md#get_eth_token_transactions) | **POST** /data/eth/getTokenTransactions | Transactions by Token
*EthereumApi* | [**get_eth_tokens**](docs/EthereumApi.md#get_eth_tokens) | **POST** /data/eth/getTokens | Tokens by Collection
*EthereumApi* | [**get_eth_wallet_labels**](docs/EthereumApi.md#get_eth_wallet_labels) | **POST** /insights/eth/getWalletLabels | Wallet Activity Labels
*EthereumApi* | [**get_eth_wallet_nfts**](docs/EthereumApi.md#get_eth_wallet_nfts) | **POST** /data/eth/getWalletNFTs | Tokens Owned by Wallet
*EthereumApi* | [**get_eth_wallet_transactions**](docs/EthereumApi.md#get_eth_wallet_transactions) | **POST** /data/eth/getWalletTransactions | Historical Token Transactions by Wallet
*EthereumApi* | [**get_eth_wash_trade**](docs/EthereumApi.md#get_eth_wash_trade) | **POST** /analytics/eth/getWashTrade | Wash Trades by Transaction
*EthereumApi* | [**get_eth_wash_transactions**](docs/EthereumApi.md#get_eth_wash_transactions) | **POST** /analytics/eth/getWashTransactions | Wash Trades by Collection
*PolygonApi* | [**get_pol_collection_forecasts**](docs/PolygonApi.md#get_pol_collection_forecasts) | **POST** /insights/pol/getCollectionForecasts | Price Forecast by Collection
*PolygonApi* | [**get_pol_collection_owners**](docs/PolygonApi.md#get_pol_collection_owners) | **POST** /data/pol/getCollectionOwners | Wallet Owners by Collection
*PolygonApi* | [**get_pol_collection_price_diff**](docs/PolygonApi.md#get_pol_collection_price_diff) | **POST** /analytics/pol/getCollectionPriceDiff | Price Differentiation by Trait
*PolygonApi* | [**get_pol_collection_sales_ohlc**](docs/PolygonApi.md#get_pol_collection_sales_ohlc) | **POST** /analytics/pol/getCollectionSalesOHLC | Collection Sales Price Candlesticks
*PolygonApi* | [**get_pol_collection_summary**](docs/PolygonApi.md#get_pol_collection_summary) | **POST** /analytics/pol/getCollectionSummary | Summary Statistics by Collection
*PolygonApi* | [**get_pol_collection_traits**](docs/PolygonApi.md#get_pol_collection_traits) | **POST** /data/pol/getCollectionTraits | Traits by Collection
*PolygonApi* | [**get_pol_collection_transactions**](docs/PolygonApi.md#get_pol_collection_transactions) | **POST** /data/pol/getCollectionTransactions | Transactions by Collection
*PolygonApi* | [**get_pol_collections**](docs/PolygonApi.md#get_pol_collections) | **POST** /data/pol/getCollections | Aggregated Collections Supported by Gallop
*PolygonApi* | [**get_pol_custom_collection_risk**](docs/PolygonApi.md#get_pol_custom_collection_risk) | **POST** /insights/pol/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
*PolygonApi* | [**get_pol_custom_token_risk**](docs/PolygonApi.md#get_pol_custom_token_risk) | **POST** /insights/pol/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
*PolygonApi* | [**get_pol_default_collection_risk**](docs/PolygonApi.md#get_pol_default_collection_risk) | **POST** /insights/pol/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
*PolygonApi* | [**get_pol_default_token_risk**](docs/PolygonApi.md#get_pol_default_token_risk) | **POST** /insights/pol/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
*PolygonApi* | [**get_pol_historical_transactions**](docs/PolygonApi.md#get_pol_historical_transactions) | **POST** /data/pol/getHistoricalTransactions | Historical Transactions by Collection
*PolygonApi* | [**get_pol_leader_board**](docs/PolygonApi.md#get_pol_leader_board) | **POST** /analytics/pol/getLeaderBoard | Polygon Leaderboard by Collection
*PolygonApi* | [**get_pol_marketplace_data**](docs/PolygonApi.md#get_pol_marketplace_data) | **POST** /data/pol/getMarketplaceData | Collection Summary by Marketplace
*PolygonApi* | [**get_pol_marketplace_floor_price**](docs/PolygonApi.md#get_pol_marketplace_floor_price) | **POST** /data/pol/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
*PolygonApi* | [**get_pol_rarity**](docs/PolygonApi.md#get_pol_rarity) | **POST** /analytics/pol/getRarity | Token Rarity by Collection
*PolygonApi* | [**get_pol_token_appraisal**](docs/PolygonApi.md#get_pol_token_appraisal) | **POST** /insights/pol/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
*PolygonApi* | [**get_pol_token_forecasts**](docs/PolygonApi.md#get_pol_token_forecasts) | **POST** /insights/pol/getTokenForecasts | Price Forecast by Token
*PolygonApi* | [**get_pol_token_summary**](docs/PolygonApi.md#get_pol_token_summary) | **POST** /analytics/pol/getTokenSummary | Summary Statistics by Token
*PolygonApi* | [**get_pol_token_transactions**](docs/PolygonApi.md#get_pol_token_transactions) | **POST** /data/pol/getTokenTransactions | Transactions by Token
*PolygonApi* | [**get_pol_tokens**](docs/PolygonApi.md#get_pol_tokens) | **POST** /data/pol/getTokens | Tokens by Collection
*PolygonApi* | [**get_pol_wallet_labels**](docs/PolygonApi.md#get_pol_wallet_labels) | **POST** /insights/pol/getWalletLabels | Wallet Activity Labels
*PolygonApi* | [**get_pol_wallet_nfts**](docs/PolygonApi.md#get_pol_wallet_nfts) | **POST** /data/pol/getWalletNFTs | Tokens Owned by Wallet
*PolygonApi* | [**get_pol_wallet_transactions**](docs/PolygonApi.md#get_pol_wallet_transactions) | **POST** /data/pol/getWalletTransactions | Historical Token Transactions by Wallet
*PolygonApi* | [**get_pol_wash_trade**](docs/PolygonApi.md#get_pol_wash_trade) | **POST** /analytics/pol/getWashTrade | Wash Trades by Transaction
*PolygonApi* | [**get_pol_wash_transactions**](docs/PolygonApi.md#get_pol_wash_transactions) | **POST** /analytics/pol/getWashTransactions | Wash Trades by Collection
*SolanaApi* | [**get_sol_account_nfts**](docs/SolanaApi.md#get_sol_account_nfts) | **POST** /data/sol/getAccountNFTs | Tokens Owned by Wallet
*SolanaApi* | [**get_sol_collection_forecasts**](docs/SolanaApi.md#get_sol_collection_forecasts) | **POST** /insights/sol/getCollectionForecasts | Price Forecast by Collection
*SolanaApi* | [**get_sol_collection_price_diff**](docs/SolanaApi.md#get_sol_collection_price_diff) | **POST** /analytics/sol/getCollectionPriceDiff | Price Differentiation by Trait
*SolanaApi* | [**get_sol_collection_sales_ohlc**](docs/SolanaApi.md#get_sol_collection_sales_ohlc) | **POST** /analytics/sol/getCollectionSalesOHLC | Collection Sales Price Candlesticks
*SolanaApi* | [**get_sol_collection_summary**](docs/SolanaApi.md#get_sol_collection_summary) | **POST** /analytics/sol/getCollectionSummary | Summary Statistics by Collection
*SolanaApi* | [**get_sol_collection_traits**](docs/SolanaApi.md#get_sol_collection_traits) | **POST** /data/sol/getCollectionTraits | Traits by Collection
*SolanaApi* | [**get_sol_collection_transactions**](docs/SolanaApi.md#get_sol_collection_transactions) | **POST** /data/sol/getCollectionTransactions | Transactions by Collections
*SolanaApi* | [**get_sol_collections**](docs/SolanaApi.md#get_sol_collections) | **POST** /data/sol/getCollections | Aggregated Collections Supported by Gallop
*SolanaApi* | [**get_sol_custom_collection_risk**](docs/SolanaApi.md#get_sol_custom_collection_risk) | **POST** /insights/sol/getCustomCollectionRisk | Custom Volatility & Risk Metrics by Collection
*SolanaApi* | [**get_sol_custom_token_risk**](docs/SolanaApi.md#get_sol_custom_token_risk) | **POST** /insights/sol/getCustomTokenRisk | Custom Volatility & Risk Metrics by Token
*SolanaApi* | [**get_sol_default_collection_risk**](docs/SolanaApi.md#get_sol_default_collection_risk) | **POST** /insights/sol/getDefaultCollectionRisk | Default Volatility & Risk Metrics by Collection
*SolanaApi* | [**get_sol_default_token_risk**](docs/SolanaApi.md#get_sol_default_token_risk) | **POST** /insights/sol/getDefaultTokenRisk | Default Volatility & Risk Metrics by Token
*SolanaApi* | [**get_sol_historical_transactions**](docs/SolanaApi.md#get_sol_historical_transactions) | **POST** /data/sol/getHistoricalTransactions | Historical Transactions by Collection
*SolanaApi* | [**get_sol_marketplace_data**](docs/SolanaApi.md#get_sol_marketplace_data) | **POST** /data/sol/getMarketplaceData | Collection Summary by Marketplace
*SolanaApi* | [**get_sol_marketplace_floor_price**](docs/SolanaApi.md#get_sol_marketplace_floor_price) | **POST** /data/sol/getMarketplaceFloorPrice | Marketplace Floor Price by Collection
*SolanaApi* | [**get_sol_marketplace_trait_data**](docs/SolanaApi.md#get_sol_marketplace_trait_data) | **POST** /data/sol/getMarketplaceTraitData | Collection Listings by Trait & Marketplace
*SolanaApi* | [**get_sol_nft_account**](docs/SolanaApi.md#get_sol_nft_account) | **POST** /data/sol/getNFTAccount | Wallet Owners by Token
*SolanaApi* | [**get_sol_rarity**](docs/SolanaApi.md#get_sol_rarity) | **POST** /analytics/sol/getRarity | Token Rarity by Collection
*SolanaApi* | [**get_sol_token_appraisal**](docs/SolanaApi.md#get_sol_token_appraisal) | **POST** /insights/sol/getTokenAppraisal | Liquidation & Appraisal Estimate by Token
*SolanaApi* | [**get_sol_token_forecasts**](docs/SolanaApi.md#get_sol_token_forecasts) | **POST** /insights/sol/getTokenForecasts | Price Forecast by Token
*SolanaApi* | [**get_sol_token_summary**](docs/SolanaApi.md#get_sol_token_summary) | **POST** /analytics/sol/getTokenSummary | Summary Statistics by Token
*SolanaApi* | [**get_sol_token_transactions**](docs/SolanaApi.md#get_sol_token_transactions) | **POST** /data/sol/getTokenTransactions | Transactions by Token
*SolanaApi* | [**get_sol_tokens**](docs/SolanaApi.md#get_sol_tokens) | **POST** /data/sol/getTokens | Tokens by Collection
*SolanaApi* | [**get_sol_wash_trade**](docs/SolanaApi.md#get_sol_wash_trade) | **POST** /analytics/sol/getWashTrade | Wash Trades by Transaction
*SolanaApi* | [**get_sol_wash_transactions**](docs/SolanaApi.md#get_sol_wash_transactions) | **POST** /analytics/sol/getWashTransactions | Wash Trades by Collection
*StarknetApi* | [**get_skn_marketplace_data**](docs/StarknetApi.md#get_skn_marketplace_data) | **POST** /data/skn/getMarketplaceData | Gallop Marketplace Data
*StarknetApi* | [**get_skn_marketplace_floor_price**](docs/StarknetApi.md#get_skn_marketplace_floor_price) | **POST** /data/skn/getMarketplaceFloorPrice | Marketplace Floor Price by Collection


## Documentation For Models

 - [GetEthCollectionFloorPriceOhlcRequest](docs/GetEthCollectionFloorPriceOhlcRequest.md)
 - [GetEthCollectionForecastsRequest](docs/GetEthCollectionForecastsRequest.md)
 - [GetEthCollectionForecastsRequestArchParams](docs/GetEthCollectionForecastsRequestArchParams.md)
 - [GetEthCollectionListingsOhlcRequest](docs/GetEthCollectionListingsOhlcRequest.md)
 - [GetEthCollectionOwnersRequest](docs/GetEthCollectionOwnersRequest.md)
 - [GetEthCollectionPriceDiffRequest](docs/GetEthCollectionPriceDiffRequest.md)
 - [GetEthCollectionSalesOhlcRequest](docs/GetEthCollectionSalesOhlcRequest.md)
 - [GetEthCollectionSummaryRequest](docs/GetEthCollectionSummaryRequest.md)
 - [GetEthCollectionTransactionsRequest](docs/GetEthCollectionTransactionsRequest.md)
 - [GetEthCollectionsRequest](docs/GetEthCollectionsRequest.md)
 - [GetEthCustomCollectionRiskRequest](docs/GetEthCustomCollectionRiskRequest.md)
 - [GetEthCustomCollectionRiskRequestArcParams](docs/GetEthCustomCollectionRiskRequestArcParams.md)
 - [GetEthCustomCollectionRiskRequestGarParams](docs/GetEthCustomCollectionRiskRequestGarParams.md)
 - [GetEthCustomCollectionRiskRequestHarParams](docs/GetEthCustomCollectionRiskRequestHarParams.md)
 - [GetEthCustomTokenRiskRequest](docs/GetEthCustomTokenRiskRequest.md)
 - [GetEthDefaultCollectionRiskRequest](docs/GetEthDefaultCollectionRiskRequest.md)
 - [GetEthDefaultTokenRiskRequest](docs/GetEthDefaultTokenRiskRequest.md)
 - [GetEthEnsLookupRequest](docs/GetEthEnsLookupRequest.md)
 - [GetEthHistoricalEventsRequest](docs/GetEthHistoricalEventsRequest.md)
 - [GetEthHistoricalTransactionsRequest](docs/GetEthHistoricalTransactionsRequest.md)
 - [GetEthLeaderBoardRequest](docs/GetEthLeaderBoardRequest.md)
 - [GetEthMarketplaceDataRequest](docs/GetEthMarketplaceDataRequest.md)
 - [GetEthMarketplaceFloorPriceRequest](docs/GetEthMarketplaceFloorPriceRequest.md)
 - [GetEthMarketplaceTraitDataRequest](docs/GetEthMarketplaceTraitDataRequest.md)
 - [GetEthRarityRequest](docs/GetEthRarityRequest.md)
 - [GetEthTokenAppraisalRequest](docs/GetEthTokenAppraisalRequest.md)
 - [GetEthTokenForecastsRequest](docs/GetEthTokenForecastsRequest.md)
 - [GetEthTokenSummaryRequest](docs/GetEthTokenSummaryRequest.md)
 - [GetEthTokenTransactionsRequest](docs/GetEthTokenTransactionsRequest.md)
 - [GetEthTokensRequest](docs/GetEthTokensRequest.md)
 - [GetEthWalletLabelsRequest](docs/GetEthWalletLabelsRequest.md)
 - [GetEthWalletNftsRequest](docs/GetEthWalletNftsRequest.md)
 - [GetEthWalletTransactionsRequest](docs/GetEthWalletTransactionsRequest.md)
 - [GetEthWashTradeRequest](docs/GetEthWashTradeRequest.md)
 - [GetEthWashTransactionsRequest](docs/GetEthWashTransactionsRequest.md)
 - [GetPolCollectionForecastsRequest](docs/GetPolCollectionForecastsRequest.md)
 - [GetPolCollectionOwnersRequest](docs/GetPolCollectionOwnersRequest.md)
 - [GetPolCollectionPriceDiffRequest](docs/GetPolCollectionPriceDiffRequest.md)
 - [GetPolCollectionSalesOhlcRequest](docs/GetPolCollectionSalesOhlcRequest.md)
 - [GetPolCollectionSummaryRequest](docs/GetPolCollectionSummaryRequest.md)
 - [GetPolCollectionTraitsRequest](docs/GetPolCollectionTraitsRequest.md)
 - [GetPolCollectionTransactionsRequest](docs/GetPolCollectionTransactionsRequest.md)
 - [GetPolCollectionsRequest](docs/GetPolCollectionsRequest.md)
 - [GetPolCustomCollectionRiskRequest](docs/GetPolCustomCollectionRiskRequest.md)
 - [GetPolCustomTokenRiskRequest](docs/GetPolCustomTokenRiskRequest.md)
 - [GetPolDefaultCollectionRiskRequest](docs/GetPolDefaultCollectionRiskRequest.md)
 - [GetPolDefaultTokenRiskRequest](docs/GetPolDefaultTokenRiskRequest.md)
 - [GetPolHistoricalTransactionsRequest](docs/GetPolHistoricalTransactionsRequest.md)
 - [GetPolMarketplaceDataRequest](docs/GetPolMarketplaceDataRequest.md)
 - [GetPolMarketplaceFloorPriceRequest](docs/GetPolMarketplaceFloorPriceRequest.md)
 - [GetPolRarityRequest](docs/GetPolRarityRequest.md)
 - [GetPolTokenAppraisalRequest](docs/GetPolTokenAppraisalRequest.md)
 - [GetPolTokenForecastsRequest](docs/GetPolTokenForecastsRequest.md)
 - [GetPolTokenSummaryRequest](docs/GetPolTokenSummaryRequest.md)
 - [GetPolTokenTransactionsRequest](docs/GetPolTokenTransactionsRequest.md)
 - [GetPolTokensRequest](docs/GetPolTokensRequest.md)
 - [GetPolWalletNftsRequest](docs/GetPolWalletNftsRequest.md)
 - [GetPolWalletTransactionsRequest](docs/GetPolWalletTransactionsRequest.md)
 - [GetPolWashTradeRequest](docs/GetPolWashTradeRequest.md)
 - [GetPolWashTransactionsRequest](docs/GetPolWashTransactionsRequest.md)
 - [GetSknMarketplaceDataRequest](docs/GetSknMarketplaceDataRequest.md)
 - [GetSknMarketplaceFloorPriceRequest](docs/GetSknMarketplaceFloorPriceRequest.md)
 - [GetSolAccountNftsRequest](docs/GetSolAccountNftsRequest.md)
 - [GetSolCollectionForecastsRequest](docs/GetSolCollectionForecastsRequest.md)
 - [GetSolCollectionForecastsRequestArchParams](docs/GetSolCollectionForecastsRequestArchParams.md)
 - [GetSolCollectionPriceDiffRequest](docs/GetSolCollectionPriceDiffRequest.md)
 - [GetSolCollectionSalesOhlcRequest](docs/GetSolCollectionSalesOhlcRequest.md)
 - [GetSolCollectionSummaryRequest](docs/GetSolCollectionSummaryRequest.md)
 - [GetSolCollectionTraitsRequest](docs/GetSolCollectionTraitsRequest.md)
 - [GetSolCollectionTransactionsRequest](docs/GetSolCollectionTransactionsRequest.md)
 - [GetSolCollectionsRequest](docs/GetSolCollectionsRequest.md)
 - [GetSolCustomCollectionRiskRequest](docs/GetSolCustomCollectionRiskRequest.md)
 - [GetSolCustomTokenRiskRequest](docs/GetSolCustomTokenRiskRequest.md)
 - [GetSolDefaultCollectionRiskRequest](docs/GetSolDefaultCollectionRiskRequest.md)
 - [GetSolDefaultTokenRiskRequest](docs/GetSolDefaultTokenRiskRequest.md)
 - [GetSolHistoricalTransactionsRequest](docs/GetSolHistoricalTransactionsRequest.md)
 - [GetSolMarketplaceDataRequest](docs/GetSolMarketplaceDataRequest.md)
 - [GetSolMarketplaceFloorPriceRequest](docs/GetSolMarketplaceFloorPriceRequest.md)
 - [GetSolMarketplaceTraitDataRequest](docs/GetSolMarketplaceTraitDataRequest.md)
 - [GetSolNftAccountRequest](docs/GetSolNftAccountRequest.md)
 - [GetSolRarityRequest](docs/GetSolRarityRequest.md)
 - [GetSolTokenAppraisalRequest](docs/GetSolTokenAppraisalRequest.md)
 - [GetSolTokenForecastsRequest](docs/GetSolTokenForecastsRequest.md)
 - [GetSolTokenForecastsRequestArchParams](docs/GetSolTokenForecastsRequestArchParams.md)
 - [GetSolTokenSummaryRequest](docs/GetSolTokenSummaryRequest.md)
 - [GetSolTokenTransactionsRequest](docs/GetSolTokenTransactionsRequest.md)
 - [GetSolTokensRequest](docs/GetSolTokensRequest.md)
 - [GetSolWashTradeRequest](docs/GetSolWashTradeRequest.md)
 - [GetSolWashTransactionsRequest](docs/GetSolWashTransactionsRequest.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author

support@higallop.com

