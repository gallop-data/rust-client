# GetSolTokenSummaryRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**mint_address** | **Vec<String>** | A token mint address or list of token addresses. | 
**token_id** | Option<**String**> | The numerical id for the token. Provide either id or mint address. | [optional]
**start_date** | Option<**String**> | The start date to pull data for calculations | [optional]
**end_date** | Option<**String**> | The end date to pull data for calculations | [optional]
**rept_curr** | Option<**String**> | The currency to report results in | [optional]
**exclude_wash** | Option<**bool**> | Exclude suspected wash transactions? | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


