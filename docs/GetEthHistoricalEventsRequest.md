# GetEthHistoricalEventsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**collection_address** | **String** | The contract address of a collection. | 
**token_id** | Option<**String**> | The id for the token. | [optional]
**page** | Option<**i32**> | The pagination cursor. | [optional]
**page_size** | Option<**i32**> | The number of records returned per page. | [optional]
**event_date** | Option<**String**> | Only return events occuring after this day [YYYY-MM-DD] | [optional]
**event_type** | Option<**String**> | The type of event: list, transfer, offer, mint, sale, cancel_list or cancel_offer | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


