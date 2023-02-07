# GetPolCollectionSalesOhlcRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The Polygon contract address to identify the collection. | 
**frequency** | Option<**String**> | The interval at which to return OHLC, e.g. `1D` for daily, `1M` for monthly etc. | [optional]
**group_by** | Option<**String**> | An attribute of the NFT to group results by. | [optional]
**volume** | Option<**bool**> | If 'true', response dicts contain OHLCV | [optional]
**n_trades** | Option<**bool**> | If 'true', append number of trades to OHLC(V) | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**start_date** | Option<**String**> | The start date to pull data for calculations | [optional]
**end_date** | Option<**String**> | The end date to pull data for calculations | [optional]
**exclude_wash** | Option<**bool**> | Exclude suspected wash transactions? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


