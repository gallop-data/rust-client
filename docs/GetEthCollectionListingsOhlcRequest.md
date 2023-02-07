# GetEthCollectionListingsOhlcRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The Ethereum contract address to identify the collection. | 
**frequency** | Option<**String**> | The interval at which to return OHLC, e.g. `1D` for daily, `1M` for monthly etc. | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**listing_start_date** | Option<**String**> | The ISO 8601 date/datetime of the oldest listing to pull for calculations | [optional]
**listing_end_date** | Option<**String**> | The ISO 8601 date/datetime of the most recent listing to pull for calculations | [optional]
**report_start_date** | Option<**String**> | The ISO 8601 start date/datetime to return results for | [optional]
**report_end_date** | Option<**String**> | The ISO 8601 end date/datetime to return results for | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


