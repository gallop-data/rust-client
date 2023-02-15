# GetEthCollectionListingsOhlcRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The Ethereum contract address to identify the collection. | 
**floor_only** | Option<**bool**> | If `true`, report only historical floor prices. Otherwise, report OHFC candlesticks, number of active listings, number of unique owners and the average age of open listings. | [optional]
**frequency** | Option<**String**> | The interval at which to return Floor prices / OHLF, e.g. `1D` for daily, `1M` for monthly etc. Must be >= `6H` | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**report_start_date** | Option<**String**> | The ISO 8601 start date/datetime to return results for | [optional]
**report_end_date** | Option<**String**> | The ISO 8601 end date/datetime to return results for | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


